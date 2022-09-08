// SPDX-License-Identifier: Apache-2.0

//! Module for representing simple function extensions.

use crate::input::yaml;
use crate::output::extension;
use crate::output::type_system::meta;
use std::collections::HashMap;
use std::sync::Arc;

/// The definition of a function implementation.
#[derive(Clone, Debug)]
pub struct Definition {
    /// Unique number within the tree that can be used to refer to this
    /// extension when exporting in protobuf form.
    pub extension_id: u64,

    /// Link to information common to a set of function implementations going by
    /// the same name.
    pub common: Arc<Common>,

    /// The derived compound name of this function.
    pub compound_name: String,

    /// The expected arguments of the function.
    pub arguments: Vec<ArgumentSlot>,

    /// Specifies the variadic behavior of the last argument slot, if any.
    pub variadic: VariadicBehavior,

    /// Whether this function is session-dependent. If set, evaluation of the
    /// function may differ from session to session.
    pub session_dependent: bool,

    /// Whether this function is deterministic; if set, the function can be
    /// assumed to always return the same value for the same input.
    pub deterministic: bool,

    /// How the function deals with nullability. Note that this information is
    /// also captured in the parsed argument patterns and return type
    /// derivation by means of nullability captures.
    pub nullability_handling: NullabilityHandling,

    /// The type derivation program used to derive the return type.
    pub return_type: meta::Program,

    /// Implementation map. This is not yet specified in Substrait, so this is
    /// still very generic.
    pub implementations: HashMap<String, yaml::Value>,
}

/// Information common to a group of function implementations with the same name.
#[derive(Clone, Debug)]
pub struct Common {
    // The simple name of the function.
    pub name: String,

    /// An optional human-readable description of the behavior of the function.
    pub description: Option<String>,

    /// The function type; scalar, aggregate, or window.
    pub function_type: Type,
}

/// The type of function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Type {
    /// A scalar function, converting a single value to a single value.
    Scalar,

    /// An aggregate function, reducing a list of values to a single value.
    Aggregate,

    /// A window function, reducing a window within a list of values to a
    /// single value.
    Window,
}

/// Information about a function argument slot (a.k.a. parameter outside of
/// Substrait).
#[derive(Clone, Debug)]
pub struct ArgumentSlot {
    /// Optional argument name to aid human understanding.
    pub name: Option<String>,

    /// Optional description of the argument.
    pub description: Option<String>,

    /// The type of argument.
    pub argument_type: ArgumentSlotType,
}

/// An argument type, along with information specific to that type.
#[derive(Clone, Debug)]
pub enum ArgumentSlotType {
    /// This argument slot accepts a value at runtime. The data type of this
    /// value must match the contained pattern.
    Value(ValueArgumentSlot),

    /// This argument slot accepts a data type without a value. The data type
    /// is matched against the contained pattern.
    Type(TypeArgumentSlot),

    /// This argument slot accepts an enumeration option.
    Enumeration(EnumerationArgumentSlot),
}

/// Definition of a value argument slot.
#[derive(Clone, Debug)]
pub struct ValueArgumentSlot {
    /// The expected data type.
    pub pattern: meta::pattern::Value,

    /// If true, the argument slot must be bound to a literal.
    pub constant: bool,
}

/// Definition of a data type argument slot.
#[derive(Clone, Debug)]
pub struct TypeArgumentSlot {
    /// The expected data type. Normally, the pattern used here is a binding
    /// that hasn't been used previously.
    pub pattern: meta::pattern::Value,
}

/// Definition of an enumeration option.
#[derive(Clone, Debug)]
pub struct EnumerationArgumentSlot {
    /// The list of options that can be passed to this enumeration, using their
    /// original case convention. Matching is to be done case-insensitively.
    pub options: Vec<String>,

    /// If false, this enumeration argument can explicitly be left unspecified.
    /// This leaves the choice of the option up to the consumer; it is then to
    /// pick the first option in definition order that it supports.
    pub required: bool,
}

/// Definition of the variadic behavior of the last argument slot.
#[derive(Clone, Debug)]
pub struct VariadicBehavior {
    /// The specified "parameter consistency", the semantics of which are
    /// already captured by the derived patterns.
    pub parameter_consistency: ParameterConsistency,

    /// The minimum number of arguments that have to match the last slot.
    pub min: usize,

    /// The maximum number of arguments that can match the last slot.
    pub max: usize,
}

/// The specified consistency requirement of the variadic arguments passed
/// to the last argument slot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ParameterConsistency {
    /// The arguments must be consistent; that is, a binding can only be
    /// bound to a single metavalue. This is the default behavior of the
    /// pattern matching logic if a pattern is matched more than once.
    Consistent,

    /// The arguments may be inconsistent; that is, a binding can match a
    /// different metavalue each time the pattern is matched. This is
    /// captured in the patterns by converting all normal binding patterns to
    /// inconsistent bindings when evaluating syntactic sugar.
    Inconsistent,
}

/// The specified nullability behavior of a function.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NullabilityHandling {
    /// Specifies that a function can capture any combination of nullability
    /// for its arguments. If and only if none of the arguments are nullable,
    /// will output types be non-nullable. This is captured in the patterns by
    /// replacing all top-level nullability specifiers with an inconsistent
    /// binding named with something not yet used for anything else. Toplevel
    /// bindings that were not yet overriding nullability are furthermore
    /// promoted to bindings that do override nullability, using the same
    /// inconsistent binding for the nullability specifier.
    Mirror,

    /// Specifies that a function can capture any combination of nullability
    /// for its arguments. Nullability of the output types is not modified.
    /// This is captured in the patterns by replacing all top-level nullability
    /// specifiers in the argument patterns with Any patterns.
    DeclaredOutput,

    /// Specifies that the nullability of the arguments and output types are
    /// exactly as specified; no changes are needed for the patterns.
    Discrete,
}

/// A reference to a completed function namespace.
pub type NamespaceReference = extension::namespace::Reference<Definition>;

/// A potentially mutable function namespace definition.
pub type NamespaceDefinition = extension::namespace::Definition<Definition>;

/// A to-be-resolved reference to a function. Includes the name and URI even if
/// unresolved.
pub type UnresolvedReference = extension::reference::Data<Definition>;

/// The result of a name resolution. May consist of any number of definitions
/// that are ambiguously referred to. The results may be further refined at a
/// later stage. For functions, this is used to allow referring to functions
/// by their simple name rather than by their compound name even if Substrait
/// explicitly does not allow this due to ambiguity, in order to give better
/// error messages than just "function not found".
pub type ResolutionResult = extension::namespace::ResolutionResult<Definition>;

/// A potentially unresolved reference to a function implementation. Includes
/// the name and URI even if unresolved.
pub type Reference = extension::Reference<Definition>;
