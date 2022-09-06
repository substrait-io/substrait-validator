// SPDX-License-Identifier: Apache-2.0

//! Module for representing anchor-based references to extensions.

use crate::output::path;
use crate::util;
use std::sync::Arc;

/// Represents an identifier that was used to reference something. It is
/// stored along with a resolution result to retain information about the
/// reference even if the resolution failed, and is generally only used for
/// identity/equality checks and diagnostic information.
#[derive(Clone, Debug, Default)]
pub struct Identifier {
    /// The name of the object being referred to, if known. Always stored using
    /// the case convention used by the reference.
    name: Option<String>,

    /// If this name is also abstracted by an integer anchor number in the
    /// plan, this is set to the path to the node that defined it, if known.
    anchor_path: Option<path::PathBuf>,
}

impl PartialEq for Identifier {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Eq for Identifier {}

impl std::hash::Hash for Identifier {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

impl std::fmt::Display for Identifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(name) = &self.name {
            write!(f, "{}", util::string::as_ident_or_string(name))
        } else {
            write!(f, "?")
        }
    }
}

impl From<String> for Identifier {
    fn from(name: String) -> Self {
        Identifier {
            name: Some(name),
            anchor_path: None,
        }
    }
}

impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Identifier {
            name: Some(name.to_string()),
            anchor_path: None,
        }
    }
}

impl Identifier {
    /// Create a new anchor-based reference.
    pub fn new<S: ToString>(
        name: Option<S>,
        anchor_path: Option<path::PathBuf>,
    ) -> Arc<Identifier> {
        Arc::new(Identifier {
            name: name.map(|x| x.to_string()),
            anchor_path,
        })
    }

    /// Create a new named reference.
    pub fn new_by_name<S: ToString>(name: S) -> Arc<Identifier> {
        Arc::new(Identifier {
            name: Some(name.to_string()),
            anchor_path: None,
        })
    }

    /// Create a new unknown reference.
    pub fn new_unknown() -> Arc<Identifier> {
        Arc::default()
    }

    /// Returns the name, if known.
    pub fn name(&self) -> Option<&str> {
        self.name.as_ref().map(|s| &s[..])
    }

    /// Returns the path to the anchor, if known.
    pub fn anchor_path(&self) -> Option<&path::PathBuf> {
        self.anchor_path.as_ref()
    }
}

/// Named/namespaced reference to a particular named extension within a
/// particular URI-referenced extension.
#[derive(Debug)]
pub struct Data<T> {
    /// The name of the type, type variation, or function. If we're referring
    /// to a module, this is unused (both name and anchor are set to None)
    pub name: Arc<Identifier>,

    /// The URI of the YAML file that defined this extension.
    pub uri: Arc<Identifier>,

    /// Extension definition information, specific to this type of extension,
    /// if we managed to resolve the reference.
    pub definition: Option<Arc<T>>,
}

impl<T> Clone for Data<T> {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            uri: self.uri.clone(),
            definition: self.definition.clone(),
        }
    }
}

impl<T> Default for Data<T> {
    fn default() -> Self {
        Self {
            name: Default::default(),
            uri: Default::default(),
            definition: Default::default(),
        }
    }
}

impl<T> PartialEq for Data<T> {
    /// References are equal if they refer to the same thing, regardless of how
    /// they refer to it. If we're not sure because either reference is
    /// (partially) unresolved, return false pessimistically.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.uri == other.uri
    }
}

impl<T> Eq for Data<T> {}

impl<T> std::hash::Hash for Data<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.uri.hash(state);
    }
}

impl<T> std::fmt::Display for Data<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}::{}", self.uri, self.name)
    }
}

impl<T> From<String> for Data<T> {
    fn from(name: String) -> Self {
        Data {
            name: Arc::new(name.into()),
            uri: Arc::default(),
            definition: None,
        }
    }
}

impl<T> From<&str> for Data<T> {
    fn from(name: &str) -> Self {
        Data {
            name: Arc::new(name.into()),
            uri: Arc::default(),
            definition: None,
        }
    }
}

/// References are stored in Arcs, so they can be (somewhat) efficiently
/// copied.
pub type Reference<T> = Arc<Data<T>>;
