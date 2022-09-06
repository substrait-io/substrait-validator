// SPDX-License-Identifier: Apache-2.0

//! Module for representing namespaces used within extension definitions.

use crate::output::diagnostic;
use crate::output::extension;
use crate::parse::context;
use crate::parse::traversal;
use std::collections::HashMap;
use std::sync::Arc;

/// Reference to a nested namespace structure.
pub type Reference<T> = Arc<Definition<T>>;

/// Nested namespace structure.
///
/// There are a lot of things "weird" about this structure, because Substrait
/// is defined horribly; everything is a valid identifier (including
/// identifiers with periods in them, so a period is ambiguous between a
/// namespace separator and just part of an identifier), matching is
/// case-insensitive (but for the validator we do want to store what the user's
/// case convention was for a definition), and for type variations names
/// need not even be unique (arguably there's a different variation namespace
/// for each type class, but the type class isn't yet known when a type
/// variation anchor is defined, and that's where we want to resolve the name;
/// see <https://github.com/substrait-io/substrait/issues/268>).
///
/// So here we have an abomination of a namespace system that allows for name
/// conflicts and period ambiguity and just tries to deal with it as cleanly as
/// it can.
#[derive(Debug)]
pub struct Definition<T> {
    /// Members of the namespace. The key is stored in lowercase.
    members: HashMap<String, Vec<InternalReference<T>>>,
}

impl<T> Clone for Definition<T> {
    fn clone(&self) -> Self {
        Self {
            members: self.members.clone(),
        }
    }
}

impl<T> Default for Definition<T> {
    fn default() -> Self {
        Self {
            members: Default::default(),
        }
    }
}

impl<T> Definition<T> {
    /// Creates a new, empty namespace.
    pub fn new() -> Self {
        Default::default()
    }

    /// Defines an item with the given (case-insensitive) name. If an item
    /// going by a conflicting name already existed, the new item is inserted
    /// anyway; if the caller is concerned about that, it must first resolve
    /// the name and throw a warning or error based on its result.
    pub fn define_item<S: AsRef<str>>(&mut self, name: S, item: Arc<T>, public: bool) {
        let name = name.as_ref();
        self.members
            .entry(name.to_ascii_lowercase())
            .or_default()
            .push(InternalReference {
                member: Arc::new(Member::Item(item)),
                original_name: name.to_string(),
                public,
            })
    }

    /// Registers a nested namespace with the given (case-insensitive) name.
    /// Same rules for duplicates as define_item(). If None is specified for
    /// item, the namespace reference is registered, but its contents are
    /// left unspecified.
    pub fn define_nested<S: AsRef<str>>(
        &mut self,
        name: S,
        item: Option<Reference<T>>,
        public: bool,
    ) {
        let name = name.as_ref();
        self.members
            .entry(name.to_ascii_lowercase())
            .or_default()
            .push(InternalReference {
                member: Arc::new(if let Some(item) = item {
                    Member::Nested(item)
                } else {
                    Member::UnresolvedNested
                }),
                original_name: name.to_string(),
                public,
            })
    }

    /// Helper function that determines whether a reference is visible, based
    /// on:
    ///  - whether this is a local lookup (private members visible) or an
    ///    external lookup (private members not visible);
    ///  - whether we had to traverse into namespaces in order to get to the
    ///    reference we're looking at;
    ///  - whether all of the above namespaces were public;
    ///  - whether the reference we're looking at is public.
    fn visibility(
        local: bool,
        with_prefix: bool,
        prefix_public: bool,
        reference_public: bool,
    ) -> bool {
        // Clippy doesn't like this, but the logic is hard enough to understand
        // without being buried in a big boolean expression.
        #[allow(clippy::if_same_then_else)]
        #[allow(clippy::needless_bool)]
        if local && !with_prefix {
            // Everything defined locally is visible.
            true
        } else if (prefix_public || !with_prefix) && reference_public {
            // If all references in the path are public, the item is visible.
            true
        } else {
            // Otherwise, the item is not visible.
            false
        }
    }

    /// Helper function for resolve. Resolves all references defined in the
    /// current namespace and its children recursively.
    ///
    ///  - target: the result object that matching elements get pushed into.
    ///  - local: whether this is a local lookup (i.e. private members of
    ///    the root namespace are visible).
    ///  - prefix: if specified, the concatenated original names of the
    ///    parent namespaces that were already recursed into.
    ///  - suffix: the name that we need to resolve locally.
    ///  - prefix_public: whether the prefix is public or private. Should be
    ///    true if there is no prefix.
    fn resolve_internal(
        &self,
        target: &mut ResolutionResult<T>,
        local: bool,
        prefix: Option<&str>,
        suffix: &str,
        prefix_public: bool,
    ) {
        // Resolve locally-defined elements.
        if let Some(references) = self.members.get(&suffix.to_ascii_lowercase()) {
            for reference in references {
                let original_name = if let Some(prefix) = prefix {
                    format!("{prefix}.{}", reference.original_name)
                } else {
                    reference.original_name.clone()
                };
                let visible =
                    Self::visibility(local, prefix.is_some(), prefix_public, reference.public);
                let target_vec = if visible {
                    &mut target.visible
                } else {
                    &mut target.invisible
                };
                target_vec.push((original_name, reference.member.clone()))
            }
        }

        // If the suffix has periods in it, try splitting at every period it
        // has and seeing if that prefix resolves to any nested namespace.
        for (split_point, _) in suffix.char_indices().filter(|(_, c)| *c == '.') {
            let namespace_name = &suffix[..split_point];
            let new_suffix = &suffix[split_point + 1..];
            if let Some(references) = self.members.get(&namespace_name.to_ascii_lowercase()) {
                for reference in references {
                    match reference.member.as_ref() {
                        Member::Item(_) => (),
                        Member::Nested(nested) => {
                            let new_prefix = if let Some(prefix) = prefix {
                                format!("{prefix}.{}", reference.original_name)
                            } else {
                                reference.original_name.clone()
                            };
                            nested.resolve_internal(
                                target,
                                local,
                                Some(&new_prefix),
                                new_suffix,
                                prefix_public && reference.public,
                            );
                        }
                        Member::UnresolvedNested => {
                            let visible = Self::visibility(
                                local,
                                prefix.is_some(),
                                prefix_public,
                                reference.public,
                            );
                            if visible {
                                target.visible_incomplete = true;
                            } else {
                                target.invisible_incomplete = true;
                            };
                        }
                    }
                }
            }
        }
    }

    /// Resolves a name to all items with the same name visible from within this
    /// namespace (so, including private items).
    pub fn resolve_local<R>(&self, reference: R) -> ResolutionResult<T>
    where
        R: Into<extension::reference::Data<T>>,
    {
        let reference = reference.into();
        let name = reference.name.name().unwrap_or("!").to_string();
        let mut result = ResolutionResult::new(reference);
        self.resolve_internal(&mut result, true, None, &name, true);
        result
    }

    /// Resolves a name to all items with the same name visible from outside
    /// this namespace (so, excluding private items).
    pub fn resolve_public<R>(&self, reference: R) -> ResolutionResult<T>
    where
        R: Into<extension::reference::Data<T>>,
    {
        let reference = reference.into();
        let name = reference.name.name().unwrap_or("!").to_string();
        let mut result = ResolutionResult::new(reference);
        self.resolve_internal(&mut result, false, None, &name, true);
        result
    }
}

/// Reference to a namespace member.
#[derive(Debug)]
struct InternalReference<T> {
    /// The member that is being referred to.
    pub member: Arc<Member<T>>,

    /// The name of the member using its original case convention.
    pub original_name: String,

    /// Whether this member has public visibility.
    pub public: bool,
}

impl<T> Clone for InternalReference<T> {
    fn clone(&self) -> Self {
        Self {
            member: self.member.clone(),
            original_name: self.original_name.clone(),
            public: self.public,
        }
    }
}

/// A namespace member.
#[derive(Debug)]
enum Member<T> {
    /// A leaf item.
    Item(Arc<T>),

    /// A nested namespace.
    Nested(Reference<T>),

    /// A nested namespace of which the contents were not resolved.
    UnresolvedNested,
}

impl<T> Clone for Member<T> {
    fn clone(&self) -> Self {
        match self {
            Self::Item(x) => Self::Item(x.clone()),
            Self::Nested(x) => Self::Nested(x.clone()),
            Self::UnresolvedNested => Self::UnresolvedNested,
        }
    }
}

impl<T> Member<T> {
    /// Returns the embedded item if this is an item.
    pub fn as_item(&self) -> Option<Arc<T>> {
        match self {
            Member::Item(x) => Some(x.clone()),
            _ => None,
        }
    }

    /// Returns the embedded potentially-unresolved namespace if this is a
    /// namespace.
    pub fn as_namespace(&self) -> Option<Option<Reference<T>>> {
        match self {
            Member::Nested(x) => Some(Some(x.clone())),
            Member::UnresolvedNested => Some(None),
            _ => None,
        }
    }
}

/// Result structure for a namespace resolution.
#[derive(Clone, Debug)]
pub struct ResolutionResult<T> {
    /// The reference that was being resolved.
    unresolved_reference: extension::reference::Data<T>,

    /// Members that are visible from the point where the resolution was
    /// performed.
    visible: Vec<(String, Arc<Member<T>>)>,

    /// Whether there might be more visible members, defined within
    /// unresolved namespaces.
    visible_incomplete: bool,

    /// Members that are not visible from the point where the resolution was
    /// performed. These are enumerated nonetheless to improve the quality of
    /// error messages.
    invisible: Vec<(String, Arc<Member<T>>)>,

    /// Whether there might be more invisible members, defined within
    /// unresolved namespaces.
    invisible_incomplete: bool,

    /// Whether a filter has been applied and visible items were removed by
    /// it.
    filtered: bool,
}

impl<T> std::fmt::Display for ResolutionResult<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.unresolved_reference)?;
        if self.visible_incomplete {
            match self.visible.len() {
                0 => write!(
                    f,
                    " (no known matching definitions, namespace not resolved)"
                ),
                1 => write!(f, " (one or more matching definitions)"),
                n => write!(f, " ({n} or more matching definitions)"),
            }
        } else {
            match self.visible.len() {
                0 => write!(f, " (no matching definitions)"),
                1 => write!(f, " (one matching definition)"),
                n => write!(f, " ({n} matching definitions)"),
            }
        }
    }
}

impl<T> ResolutionResult<T> {
    /// Creates a new, empty resolution result for the given unresolved
    /// reference, to be used as a placeholder when no namespace is actually
    /// available. It will behave as if resolution failed because the namespace
    /// being looked in was itself not resolved. If an item is passed via the
    /// reference, it will be returned by as_item(). as_namespace() will return
    /// None.
    pub fn new(unresolved_reference: extension::reference::Data<T>) -> Self {
        Self {
            unresolved_reference,
            visible: vec![],
            visible_incomplete: true,
            invisible: vec![],
            invisible_incomplete: false,
            filtered: false,
        }
    }

    /// Internal helper for the various filter functions.
    fn filter<F: Fn(&(String, Arc<Member<T>>)) -> bool>(mut self, filter: F) -> Self {
        let size = self.visible.len();
        self.visible.retain(&filter);
        self.invisible.retain(&filter);
        if self.visible.len() < size {
            self.filtered = true;
        }
        self
    }

    /// Filter the resulting items by some predicate. This also filters out
    /// namespaces.
    pub fn filter_items<F: Fn(&T) -> bool>(self, filter: F) -> Self {
        self.filter(|x| {
            x.1.as_item()
                .map(|x| filter(x.as_ref()))
                .unwrap_or_default()
        })
    }

    /// Filter the resulting items by some predicate. This also filters out
    /// namespaces.
    pub fn filter_all_items(self) -> Self {
        self.filter(|x| x.1.as_item().is_some())
    }

    /// Filter out anything that isn't a namespace.
    pub fn filter_namespaces(self) -> Self {
        self.filter(|x| x.1.as_namespace().is_some())
    }

    /// Helper function for expect_one() and expect_multiple().
    fn expect<F1, F2>(
        self,
        parse_context: &mut context::Context,
        if_not_applicable: F1,
        if_ambiguous: F2,
        allow_ambiguity: bool,
        optional: bool,
    ) -> Self
    where
        F1: FnOnce(String, &mut context::Context) -> bool,
        F2: FnOnce(String, &mut context::Context) -> bool,
    {
        let mut visible = self.visible.iter();
        if visible.next().is_some() {
            if visible.next().is_some()
                && !allow_ambiguity
                && !if_ambiguous(self.unresolved_reference.to_string(), parse_context)
            {
                traversal::push_diagnostic(
                    parse_context,
                    diagnostic::Level::Error,
                    cause!(
                        LinkAmbiguousName,
                        "multiple definitions are in scope for {}",
                        self.unresolved_reference
                    ),
                )
            }
        } else if self.visible_incomplete || optional {
            // A visible namespace was not resolved (in which case we
            // optimistically assume the item exists) or the item doesn't need
            // to exist.
        } else if !self.invisible.is_empty() {
            traversal::push_diagnostic(
                parse_context,
                diagnostic::Level::Error,
                cause!(
                    LinkUnresolvedName,
                    "a definition for {} exists, but is not visible from here",
                    self.unresolved_reference
                ),
            );
        } else if self.invisible_incomplete {
            traversal::push_diagnostic(
                parse_context,
                diagnostic::Level::Error,
                cause!(
                    LinkUnresolvedName,
                    "a definition for {} may exist, but would not be visible from here",
                    self.unresolved_reference
                ),
            );
        } else if self.filtered {
            if !if_not_applicable(self.unresolved_reference.to_string(), parse_context) {
                traversal::push_diagnostic(
                    parse_context,
                    diagnostic::Level::Error,
                    cause!(
                        LinkUnresolvedName,
                        "a definition for {} exists, but it cannot be used in this context",
                        self.unresolved_reference
                    ),
                )
            }
        } else {
            traversal::push_diagnostic(
                parse_context,
                diagnostic::Level::Error,
                cause!(
                    LinkUnresolvedName,
                    "no definition found for {}",
                    self.unresolved_reference
                ),
            );
        }
        self
    }

    /// Expects a single item, yielding diagnostics if this isn't the case. If
    /// ambiguous or if all matching elements were not applicable (filtered
    /// out), the specified functions are called. They receive the reference
    /// name and the parse context as arguments to form a suitable diagnostic
    /// message. If they return false, a default diagnostic message will be
    /// emitted instead.
    pub fn expect_one<F1, F2>(
        self,
        parse_context: &mut context::Context,
        if_not_applicable: F1,
        if_ambiguous: F2,
    ) -> Self
    where
        F1: FnOnce(String, &mut context::Context) -> bool,
        F2: FnOnce(String, &mut context::Context) -> bool,
    {
        self.expect(parse_context, if_not_applicable, if_ambiguous, false, false)
    }

    /// Expects zero or one item(s), yielding diagnostics if this isn't the
    /// case. If ambiguous, the specified function is called. It receives the
    /// reference name and the parse context as arguments to form a suitable
    /// diagnostic message. If it returns false, a default diagnostic message
    /// will be emitted instead.
    pub fn expect_not_ambiguous<F>(
        self,
        parse_context: &mut context::Context,
        if_ambiguous: F,
    ) -> Self
    where
        F: FnOnce(String, &mut context::Context) -> bool,
    {
        self.expect(parse_context, |_, _| true, if_ambiguous, false, true)
    }

    /// Expects a one or more items, yielding diagnostics if this isn't the
    /// case. If all matching elements were not applicable (filtered out), the
    /// specified functions are called. They receive the reference name and the
    /// parse context as arguments to form a suitable diagnostic message. If
    /// they return false, a default diagnostic message will be emitted
    /// instead.
    pub fn expect_multiple<F>(
        self,
        parse_context: &mut context::Context,
        if_not_applicable: F,
    ) -> Self
    where
        F: FnOnce(String, &mut context::Context) -> bool,
    {
        self.expect(parse_context, if_not_applicable, |_, _| true, true, false)
    }

    /// Silently returns the first matching item, if any. If there are none,
    /// this just returns an unresolved reference. Use
    /// filter_items().expect_one() to formulate error messages if there are
    /// multiple or no items available.
    pub fn as_item(&self) -> extension::reference::Reference<T> {
        let mut data = self.unresolved_reference.clone();
        if let Some(item) = self.visible.iter().filter_map(|x| x.1.as_item()).next() {
            data.definition.replace(item);
        }
        Arc::new(data)
    }

    /// Silently returns the first matching item, if any. Unlike as_item(),
    /// this returns None if there are no matches.
    pub fn as_opt_item(&self) -> Option<extension::reference::Reference<T>> {
        self.visible
            .iter()
            .filter_map(|x| x.1.as_item())
            .next()
            .map(|item| {
                let mut data = self.unresolved_reference.clone();
                data.definition.replace(item);
                Arc::new(data)
            })
    }

    /// Silently returns the first matching namespace. Use
    /// filter_namespaces().expect_one() to formulate error messages if there
    /// are multiple or no namespaces available.
    pub fn as_namespace(&self) -> Option<Reference<T>> {
        self.visible
            .iter()
            .filter_map(|x| x.1.as_namespace())
            .next()
            .flatten()
    }

    /// Return an error if one or more definitions were found for this name
    /// resolution, to be used just before defining a new item.
    pub fn expect_not_yet_defined(&self) -> diagnostic::Result<()> {
        if self.visible.is_empty() {
            Ok(())
        } else {
            Err(cause!(
                LinkDuplicateDefinition,
                "{} is already defined",
                self.unresolved_reference
            ))
        }
    }
}
