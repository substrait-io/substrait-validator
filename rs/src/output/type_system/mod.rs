// SPDX-License-Identifier: Apache-2.0

//! Module for implementations of the two type systems used in Substrait.
//!
//! The [data] module provides an implementation for the static portion of the
//! "normal" type system, used by Substrait to describe schemas for relations
//! and types for expressions. The [meta] module provides a complete
//! implementation for the static-only type system used for type parameters
//! and function return type derivations.

pub mod data;
pub mod meta;
