// SPDX-License-Identifier: Apache-2.0

//! Module for dealing with Substrait's meta-type system, used for type
//! parameters.
//!
//! See [`Type`], [`Value`], and [`Pattern`].

pub mod context;
pub mod function;
pub mod pattern;
pub mod program;
pub mod r#type;
pub mod value;

pub use context::Context;
pub use function::Function;
pub use pattern::Pattern;
pub use program::Program;
pub use program::Statement;
pub use r#type::Type;
pub use value::Value;
