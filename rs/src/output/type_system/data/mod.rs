// SPDX-License-Identifier: Apache-2.0

//! Module for dealing with Substrait's runtime type system.
//!
//! See [`Type`].

pub mod class;
pub mod parameter;
pub mod r#type;
pub mod variation;

use crate::output::diagnostic;

pub use class::Class;
pub use parameter::Parameter;
pub use r#type::Definition as TypeDef;
pub use r#type::Type;
pub use variation::Variation;

/// Creates a new type.
pub fn new_type(
    class: Class,
    nullable: bool,
    variation: Variation,
    parameters: Vec<Parameter>,
) -> diagnostic::Result<Type> {
    TypeDef::new(class, nullable, variation, parameters)
}

/// Creates a new unresolved type with the given description.
pub fn new_unresolved_type() -> Type {
    TypeDef::new(Class::Unresolved, false, Variation::SystemPreferred, vec![])
        .expect("failed to make valid unresolved type")
}

/// Creates a new struct type.
pub fn new_struct<T: IntoIterator<Item = Type>>(fields: T, nullable: bool) -> Type {
    TypeDef::new(
        Class::Compound(class::Compound::Struct),
        nullable,
        Variation::SystemPreferred,
        fields.into_iter().map(Parameter::from).collect(),
    )
    .expect("failed to make valid struct")
}

/// Creates a new list type.
pub fn new_list(element: Type, nullable: bool) -> Type {
    TypeDef::new(
        Class::Compound(class::Compound::List),
        nullable,
        Variation::SystemPreferred,
        vec![Parameter::from(element)],
    )
    .expect("failed to make valid list")
}

/// Creates a new map type.
pub fn new_map(key: Type, value: Type, nullable: bool) -> Type {
    TypeDef::new(
        Class::Compound(class::Compound::List),
        nullable,
        Variation::SystemPreferred,
        vec![Parameter::from(key), Parameter::from(value)],
    )
    .expect("failed to make valid map")
}

/// Creates the type of a predicate, i.e. a boolean.
pub fn new_predicate() -> Type {
    new_predicate_with_nullability(false)
}

/// Creates the type of a predicate, i.e. a boolean.
pub fn new_predicate_with_nullability(nullable: bool) -> Type {
    TypeDef::new(
        Class::Simple(class::Simple::Boolean),
        nullable,
        Variation::SystemPreferred,
        vec![],
    )
    .expect("failed to make valid predicate type")
}

/// Creates the type of a (default) integer, i.e. i32.
pub fn new_integer() -> Type {
    new_integer_with_nullability(false)
}

/// Creates the type of a (default) integer, i.e. i32.
pub fn new_integer_with_nullability(nullable: bool) -> Type {
    TypeDef::new(
        Class::Simple(class::Simple::I32),
        nullable,
        Variation::SystemPreferred,
        vec![],
    )
    .expect("failed to make valid integer type")
}
