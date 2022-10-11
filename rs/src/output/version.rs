// SPDX-License-Identifier: Apache-2.0

//! Module for the version number struct.

use crate::output::diagnostic;

/// A version number.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version(pub u32, pub u32, pub u32);

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.0, self.1, self.2)
    }
}

impl std::str::FromStr for Version {
    type Err = diagnostic::Cause;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        static VERSION_RE: once_cell::sync::Lazy<regex::Regex> = once_cell::sync::Lazy::new(|| {
            regex::Regex::new("([0-9]+)\\.([0-9]+)\\.([0-9]+)").unwrap()
        });
        if let Some(captures) = VERSION_RE.captures(s) {
            let major = u32::from_str(
                captures
                    .get(1)
                    .ok_or_else(|| {
                        cause!(
                            IllegalValue,
                            "failed to parse version number: major version component missing"
                        )
                    })?
                    .as_str(),
            )
            .map_err(|_| {
                cause!(
                    IllegalValue,
                    "failed to parse version number: major version component is not a u32"
                )
            })?;
            let minor = u32::from_str(
                captures
                    .get(2)
                    .ok_or_else(|| {
                        cause!(
                            IllegalValue,
                            "failed to parse version number: minor version component missing"
                        )
                    })?
                    .as_str(),
            )
            .map_err(|_| {
                cause!(
                    IllegalValue,
                    "failed to parse version number: minor version component is not a u32"
                )
            })?;
            let patch = u32::from_str(
                captures
                    .get(3)
                    .ok_or_else(|| {
                        cause!(
                            IllegalValue,
                            "failed to parse version number: patch version component missing"
                        )
                    })?
                    .as_str(),
            )
            .map_err(|_| {
                cause!(
                    IllegalValue,
                    "failed to parse version number: patch version component is not a u32"
                )
            })?;
            Ok(Version(major, minor, patch))
        } else {
            Err(cause!(
                IllegalValue,
                "failed to parse version number: invalid structure"
            ))
        }
    }
}

impl Version {
    /// Returns whether two versions are compatible, i.e. no breaking changes
    /// were introduced between them, using Cargo's versioning scheme.
    pub fn compatible(&self, other: &Version) -> bool {
        if self.0 == 0 {
            self.1 == other.1
        } else {
            self.0 == other.0
        }
    }

    /// Returns whether two versions are compatible, i.e. no breaking changes
    /// were introduced between them. Substrait's versioning scheme does not
    /// support automatic detection of this in the 0.x.y regime, so this
    /// returns None in that case.
    pub fn compatible_substrait(&self, other: &Version) -> Option<bool> {
        if self.0 == 0 && other.0 == 0 && self.1 != other.1 {
            None
        } else {
            Some(self.compatible(other))
        }
    }
}
