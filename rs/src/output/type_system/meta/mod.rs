// SPDX-License-Identifier: Apache-2.0

//! Module for dealing with Substrait's meta-type system, used for type
//! parameters.
//!
//! See [`Type`], [`Value`], and [`Pattern`].

pub mod context;
pub mod pattern;
pub mod r#type;
pub mod value;

pub use context::Context;
pub use pattern::Pattern;
pub use r#type::Type;
pub use value::Value;
