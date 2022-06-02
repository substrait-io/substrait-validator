// SPDX-License-Identifier: Apache-2.0

//! Module for parsing/validating literals.

use crate::input::proto::substrait;
use crate::output::data_type;
use crate::output::diagnostic;
use crate::output::extension;
use crate::parse::context;
use crate::parse::extensions;
use crate::parse::types;
use crate::util;
use crate::util::string::Describe;
use std::sync::Arc;

/// The value of a literal, not including type information.
#[derive(Clone, Debug, PartialEq)]
enum LiteralValue {
    /// May be used for any nullable type.
    Null,

    /// May be used only for booleans.
    Boolean(bool),

    /// May be used only for I8, I16, I32, I64, Timestamp, TimestampTz, Date, and Time.
    Integer(i64),

    /// May be used only for Fp32 and Fp64.
    Float(f64),

    /// May be used only for decimals and UUIDs.
    Data16(i128),

    /// May be used only for strings, FixedChars, and VarChars.
    String(String),

    /// May be used only for binary and FixedBinary.
    Binary(Vec<u8>),

    /// May be used only for IntervalYearToMonth and IntervalDayToSecond.
    Interval(i64, i64),

    /// May be used only for structs and lists.
    Items(Vec<Literal>),

    /// May be used only for maps.
    Pairs(Vec<(Literal, Literal)>),

    /// A literal for a user-defined type, cannot parse more information.
    UserDefined,
}

impl Default for LiteralValue {
    fn default() -> Self {
        LiteralValue::Null
    }
}

/// A complete literal, including type information.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Literal {
    /// The value of the literal.
    value: LiteralValue,

    /// The data type of the literal. LiteralValue must be a valid instance of
    /// this.
    data_type: Arc<data_type::DataType>,
}

/// Converts a value in microseconds since the epoch to a chrono::NaiveDateTime.
fn to_date_time(micros: i64) -> diagnostic::Result<chrono::NaiveDateTime> {
    let secs = micros.div_euclid(1_000_000);
    let nsecs = ((micros.rem_euclid(1_000_000)) * 1000) as u32;
    chrono::NaiveDateTime::from_timestamp_opt(secs, nsecs).ok_or(ecause!(
        ExpressionIllegalLiteralValue,
        "timestamp out of range"
    ))
}

/// Converts a value in microseconds since the epoch to a string.
fn to_date_time_str(micros: i64, fmt: &str) -> String {
    to_date_time(micros)
        .map(|x| x.format(fmt).to_string())
        .unwrap_or_else(|_| String::from("?"))
}

impl Literal {
    /// Shorthand for a new null literal.
    pub fn new_null(data_type: Arc<data_type::DataType>) -> Literal {
        Literal {
            value: LiteralValue::Null,
            data_type,
        }
    }

    /// Shorthand for a new simple literal.
    fn new_simple(
        value: LiteralValue,
        simple: data_type::Simple,
        nullable: bool,
        variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
    ) -> diagnostic::Result<Literal> {
        Ok(Literal {
            value,
            data_type: data_type::DataType::new(
                data_type::Class::Simple(simple),
                nullable,
                variation,
                vec![],
            )?,
        })
    }

    /// Shorthand for a new compound literal.
    fn new_compound<T: Into<data_type::Parameter>>(
        value: LiteralValue,
        compound: data_type::Compound,
        nullable: bool,
        variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
        args: Vec<T>,
    ) -> diagnostic::Result<Literal> {
        Ok(Literal {
            value,
            data_type: data_type::DataType::new(
                data_type::Class::Compound(compound),
                nullable,
                variation,
                args.into_iter().map(|x| x.into()).collect(),
            )?,
        })
    }

    /// Returns the data type of this literal.
    pub fn data_type(&self) -> &Arc<data_type::DataType> {
        &self.data_type
    }
}

impl Describe for Literal {
    /// Represents the value of this literal with some size limit. The size
    /// limit very roughly corresponds to a number of characters, but this is
    /// purely a heuristic thing.
    fn describe(
        &self,
        f: &mut std::fmt::Formatter<'_>,
        limit: util::string::Limit,
    ) -> std::fmt::Result {
        match &self.value {
            LiteralValue::Null => {
                if self.data_type.is_unresolved() {
                    write!(f, "!")
                } else {
                    write!(f, "null")
                }
            }
            LiteralValue::Boolean(true) => write!(f, "true"),
            LiteralValue::Boolean(false) => write!(f, "false"),
            LiteralValue::Integer(i) => match self.data_type.class() {
                data_type::Class::Simple(data_type::Simple::I8) => write!(f, "{i}i8"),
                data_type::Class::Simple(data_type::Simple::I16) => write!(f, "{i}i16"),
                data_type::Class::Simple(data_type::Simple::I32) => write!(f, "{i}i32"),
                data_type::Class::Simple(data_type::Simple::I64) => write!(f, "{i}i64"),
                data_type::Class::Simple(data_type::Simple::Timestamp) => {
                    write!(f, "{}", to_date_time_str(*i, "%Y-%m-%d %H:%M:%S%.6f"))
                }
                data_type::Class::Simple(data_type::Simple::TimestampTz) => {
                    write!(f, "{} UTC", to_date_time_str(*i, "%Y-%m-%d %H:%M:%S%.6f"))
                }
                data_type::Class::Simple(data_type::Simple::Date) => {
                    write!(
                        f,
                        "{}",
                        to_date_time_str(i.saturating_mul(24 * 60 * 60 * 1_000_000), "%Y-%m-%d")
                    )
                }
                data_type::Class::Simple(data_type::Simple::Time) => {
                    write!(f, "{}", to_date_time_str(*i, "%H:%M:%S%.6f"))
                }
                _ => write!(f, "{i}"),
            },
            LiteralValue::Float(v) => {
                let max = std::cmp::min(std::cmp::max(3, limit.chars()), 10);
                write!(f, "{:3.1$}", float_pretty_print::PrettyPrintFloat(*v), max)
            }
            LiteralValue::Data16(d) => match self.data_type.class() {
                data_type::Class::Compound(data_type::Compound::Decimal) => {
                    if let Some(scale) = self.data_type.int_parameter(1) {
                        if d < &0 {
                            write!(f, "-")?;
                        }
                        let d = d.abs() as u128;
                        let s = 10u128.pow(scale as u32);
                        if self
                            .data_type
                            .int_parameter(0)
                            .map(|precision| scale < precision)
                            .unwrap_or(true)
                        {
                            write!(f, "{0}", d.div_euclid(s))?;
                        }
                        write!(f, ".")?;
                        if scale > 0 {
                            write!(f, "{0:01$}", d.rem_euclid(s), scale as usize)?;
                        }
                        Ok(())
                    } else {
                        util::string::describe_binary(f, &d.to_le_bytes(), limit)
                    }
                }
                data_type::Class::Simple(data_type::Simple::Uuid) => {
                    let b = d.to_ne_bytes();
                    write!(
                        f,
                        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
                        b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7], b[8], b[9], b[10], b[11], b[12], b[13], b[14], b[15]
                    )
                }
                _ => util::string::describe_binary(f, &d.to_le_bytes(), limit),
            },
            LiteralValue::String(s) => util::string::describe_string(f, s, limit),
            LiteralValue::Binary(b) => util::string::describe_binary(f, b, limit),
            LiteralValue::Interval(a, b) => match self.data_type.class() {
                data_type::Class::Simple(data_type::Simple::IntervalYear) => {
                    write!(f, "{a}y{b:+}m")
                }
                data_type::Class::Simple(data_type::Simple::IntervalDay) => {
                    let s = b / 1000000;
                    let us = if b > &0 { *b } else { -b } % 1000000;
                    write!(f, "{a}d{s:+}.{us:06}s")
                }
                _ => write!(f, "({a}, {b})"),
            },
            LiteralValue::Items(x) => match self.data_type.class() {
                data_type::Class::Compound(data_type::Compound::Struct) => {
                    write!(f, "(")?;
                    util::string::describe_sequence(f, x, limit, 20, |f, value, index, limit| {
                        write!(f, ".{index}: ")?;
                        value.describe(f, limit)
                    })?;
                    write!(f, ")")
                }
                data_type::Class::Compound(data_type::Compound::NamedStruct) => {
                    write!(f, "(")?;
                    util::string::describe_sequence(f, x, limit, 20, |f, value, index, limit| {
                        if let Some(name) = self
                            .data_type
                            .parameters()
                            .get(index)
                            .and_then(|x| x.get_name())
                        {
                            write!(f, ".{}: ", util::string::as_ident_or_string(name))?;
                        } else {
                            write!(f, ".{index}: ")?;
                        }
                        value.describe(f, limit)
                    })?;
                    write!(f, ")")
                }
                data_type::Class::Compound(data_type::Compound::List) => {
                    write!(f, "[")?;
                    util::string::describe_sequence(f, x, limit, 20, |f, value, _, limit| {
                        value.describe(f, limit)
                    })?;
                    write!(f, "]")
                }
                _ => {
                    write!(f, "(")?;
                    util::string::describe_sequence(f, x, limit, 20, |f, value, _, limit| {
                        value.describe(f, limit)
                    })?;
                    write!(f, ")")
                }
            },
            LiteralValue::Pairs(x) => match self.data_type.class() {
                data_type::Class::Compound(data_type::Compound::Map) => {
                    write!(f, "{{")?;
                    util::string::describe_sequence(
                        f,
                        x,
                        limit,
                        40,
                        |f, (key, value), _, limit| {
                            let (key_limit, value_limit) = limit.split(20);
                            key.describe(f, key_limit)?;
                            write!(f, ": ")?;
                            value.describe(f, value_limit)
                        },
                    )?;
                    write!(f, "}}")
                }
                _ => {
                    write!(f, "(")?;
                    util::string::describe_sequence(
                        f,
                        x,
                        limit,
                        40,
                        |f, (key, value), _, limit| {
                            write!(f, "(")?;
                            let (key_limit, value_limit) = limit.split(20);
                            key.describe(f, key_limit)?;
                            write!(f, ": ")?;
                            value.describe(f, value_limit)?;
                            write!(f, ")")
                        },
                    )?;
                    write!(f, ")")
                }
            },
            LiteralValue::UserDefined => write!(f, "(user-defined)"),
        }
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.display().fmt(f)
    }
}

/// Parses a boolean literal.
fn parse_boolean(
    x: &bool,
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Boolean(*x),
        data_type::Simple::Boolean,
        nullable,
        variation,
    )
}

/// Parses an i8 literal.
fn parse_i8(
    x: &i32,
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let x = i8::try_from(*x)
        .map_err(|_| cause!(ExpressionIllegalLiteralValue, "i8 value out of range"))?;
    Literal::new_simple(
        LiteralValue::Integer(x as i64),
        data_type::Simple::I8,
        nullable,
        variation,
    )
}

/// Parses an i16 literal.
fn parse_i16(
    x: &i32,
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let x = i16::try_from(*x)
        .map_err(|_| cause!(ExpressionIllegalLiteralValue, "i16 value out of range"))?;
    Literal::new_simple(
        LiteralValue::Integer(x as i64),
        data_type::Simple::I16,
        nullable,
        variation,
    )
}

/// Parses an i32 literal.
fn parse_i32(
    x: &i32,
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Integer(*x as i64),
        data_type::Simple::I32,
        nullable,
        variation,
    )
}

/// Parses an i64 literal.
fn parse_i64(
    x: &i64,
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Integer(*x),
        data_type::Simple::I64,
        nullable,
        variation,
    )
}

/// Parses an fp32 literal.
fn parse_fp32(
    x: &f32,
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Float(*x as f64),
        data_type::Simple::Fp32,
        nullable,
        variation,
    )
}

/// Parses an fp64 literal.
fn parse_fp64(
    x: &f64,
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Float(*x),
        data_type::Simple::Fp64,
        nullable,
        variation,
    )
}

/// Parses a string literal.
fn parse_string(
    x: &str,
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::String(x.to_string()),
        data_type::Simple::String,
        nullable,
        variation,
    )
}

/// Parses a binary literal.
fn parse_binary(
    x: &[u8],
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    Literal::new_simple(
        LiteralValue::Binary(x.to_owned()),
        data_type::Simple::Binary,
        nullable,
        variation,
    )
}

/// Parses a timestamp literal.
fn parse_timestamp(
    x: &i64,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let dt = to_date_time(*x)?;
    if dt < chrono::NaiveDate::from_ymd(1000, 1, 1).and_hms(0, 0, 0)
        || dt >= chrono::NaiveDate::from_ymd(10000, 1, 1).and_hms(0, 0, 0)
    {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "timestamp out of range 1000-01-01 to 9999-12-31"
        );
    }
    Literal::new_simple(
        LiteralValue::Integer(*x),
        data_type::Simple::Timestamp,
        nullable,
        variation,
    )
}

/// Parses a UTC timestamp literal.
fn parse_timestamp_tz(
    x: &i64,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let dt = to_date_time(*x)?;
    if dt < chrono::NaiveDate::from_ymd(1000, 1, 1).and_hms(0, 0, 0)
        || dt >= chrono::NaiveDate::from_ymd(10000, 1, 1).and_hms(0, 0, 0)
    {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "timestamp out of range 1000-01-01 UTC to 9999-12-31 UTC"
        );
    }
    Literal::new_simple(
        LiteralValue::Integer(*x),
        data_type::Simple::TimestampTz,
        nullable,
        variation,
    )
}

/// Parses a date literal.
fn parse_date(
    x: &i32,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let dt = to_date_time((*x as i64).saturating_mul(24 * 60 * 60 * 1_000_000))?;
    if dt < chrono::NaiveDate::from_ymd(1000, 1, 1).and_hms(0, 0, 0)
        || dt >= chrono::NaiveDate::from_ymd(10000, 1, 1).and_hms(0, 0, 0)
    {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "date out of range 1000-01-01 UTC to 9999-12-31 UTC"
        );
    }
    Literal::new_simple(
        LiteralValue::Integer(*x as i64),
        data_type::Simple::Date,
        nullable,
        variation,
    )
}

/// Parses a time literal.
fn parse_time(
    x: &i64,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    if *x < 0 || *x >= 24 * 60 * 60 * 1_000_000 {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "time of day out of range 00:00:00.000000 to 23:59:59.999999"
        );
    }
    Literal::new_simple(
        LiteralValue::Integer(*x),
        data_type::Simple::Time,
        nullable,
        variation,
    )
}

/// Parses a year to month interval literal.
fn parse_interval_year_to_month(
    x: &substrait::expression::literal::IntervalYearToMonth,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    // FIXME: see FIXME for associated type.
    proto_primitive_field!(x, y, years, |x, _| {
        if *x < -10000 || *x > 10000 {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "year count out of range -10000 to 10000"
            ))
        } else {
            Ok(())
        }
    });
    proto_primitive_field!(x, y, months, |x, _| {
        if *x < -120000 || *x > 120000 {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "month count out of range -120000 to 120000"
            ))
        } else {
            Ok(())
        }
    });
    let months = x.months.saturating_add(x.years.saturating_mul(12));
    if months < -120000 || months > 120000 {
        diagnostic!(
            y,
            Error,
            ExpressionIllegalLiteralValue,
            "combined interval out of range -10000 to 10000 years"
        );
    }
    Literal::new_simple(
        LiteralValue::Interval(x.years.into(), x.months.into()),
        data_type::Simple::IntervalYear,
        nullable,
        variation,
    )
}

/// Parses a day to second interval literal.
fn parse_interval_day_to_second(
    x: &substrait::expression::literal::IntervalDayToSecond,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    // FIXME: see FIXME for associated type.
    proto_primitive_field!(x, y, days, |x, _| {
        if *x < -3650000 || *x > 3650000 {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "day count out of range -3_650_000 to 3_650_000"
            ))
        } else {
            Ok(())
        }
    });

    proto_primitive_field!(x, y, seconds);
    proto_primitive_field!(x, y, microseconds);
    Literal::new_simple(
        LiteralValue::Interval(
            x.days.into(),
            i64::from(x.seconds) * 1000000 + i64::from(x.microseconds),
        ),
        data_type::Simple::IntervalDay,
        nullable,
        variation,
    )
}

/// Parses a UUID literal.
fn parse_uuid(
    x: &[u8],
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    if let Ok(x) = x.try_into() {
        Literal::new_simple(
            LiteralValue::Data16(i128::from_ne_bytes(x)),
            data_type::Simple::Uuid,
            nullable,
            variation,
        )
    } else {
        Err(cause!(
            ExpressionIllegalLiteralValue,
            "uuid literals must be 16 bytes in length, got {}",
            x.len()
        ))
    }
}

/// Parses a fixed-length string literal.
fn parse_fixed_char(
    x: &str,
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    Literal::new_compound(
        LiteralValue::String(x.to_string()),
        data_type::Compound::FixedChar,
        nullable,
        variation,
        vec![x.len() as u64],
    )
}

/// Parses a variable-length string literal.
fn parse_var_char(
    x: &substrait::expression::literal::VarChar,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    proto_primitive_field!(x, y, length);
    let len = x.length as usize;
    proto_primitive_field!(x, y, value, |x, _| {
        if x.len() > len {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "varchar literal value is longer than specified length"
            ))
        } else {
            Ok(())
        }
    });
    Literal::new_compound(
        LiteralValue::String(x.value.clone()),
        data_type::Compound::VarChar,
        nullable,
        variation,
        vec![len as u64],
    )
}

/// Parses a fixed-length binary literal.
fn parse_fixed_binary(
    x: &[u8],
    _y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    Literal::new_compound(
        LiteralValue::Binary(x.to_owned()),
        data_type::Compound::FixedBinary,
        nullable,
        variation,
        vec![x.len() as u64],
    )
}

/// Parses a decimal literal.
fn parse_decimal(
    x: &substrait::expression::literal::Decimal,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    proto_primitive_field!(x, y, precision, |x, _| {
        if *x < 0 {
            Err(cause!(
                IllegalValue,
                "negative type parameters are not supported"
            ))
        } else {
            Ok(())
        }
    });
    proto_primitive_field!(x, y, scale);
    let val = proto_primitive_field!(x, y, value, |x, _| {
        if let Ok(x) = (&x[..]).try_into() {
            Ok(i128::from_le_bytes(x))
        } else {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "decimal literals must be 16 bytes in length, got {}",
                x.len()
            ))
        }
    })
    .1;
    let precision = u64::try_from(x.precision).unwrap_or_default();
    let scale = u64::try_from(x.scale).unwrap_or_default();

    if let Some(val) = val {
        let range = 10i128.saturating_pow(precision.try_into().unwrap_or_default());
        if val >= range || val <= -range {
            Err(cause!(
                ExpressionIllegalLiteralValue,
                "decimal value is out of range for specificied precision and scale"
            ))
        } else {
            Literal::new_compound(
                LiteralValue::Data16(val),
                data_type::Compound::Decimal,
                nullable,
                variation,
                vec![precision, scale],
            )
        }
    } else {
        Ok(Literal::default())
    }
}

/// Parses a struct literal.
fn parse_struct_int(
    x: &substrait::expression::literal::Struct,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let (values, types): (Vec<_>, Vec<_>) = proto_repeated_field!(x, y, fields, parse_literal)
        .1
        .into_iter()
        .map(|x| {
            let x = x.unwrap_or_default();
            let data_type = x.data_type.clone();
            (x, data_type)
        })
        .unzip();
    Literal::new_compound(
        LiteralValue::Items(values),
        data_type::Compound::Struct,
        nullable,
        variation,
        types,
    )
}

/// Parses a struct literal.
pub fn parse_struct(
    x: &substrait::expression::literal::Struct,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let literal = parse_struct_int(x, y, nullable, variation)?;
    y.set_data_type(literal.data_type().clone());
    Ok(literal)
}

/// Parses a list literal.
fn parse_list(
    x: &substrait::expression::literal::List,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let values: Vec<_> = proto_required_repeated_field!(x, y, values, parse_literal)
        .1
        .into_iter()
        .map(|x| x.unwrap_or_default())
        .collect();
    if values.is_empty() {
        comment!(
            y,
            "At least one list element is required to derive type. Use EmptyList instead."
        );
    }
    let mut data_type = Arc::default();
    for (index, value) in values.iter().enumerate() {
        data_type = types::assert_equal(
            y,
            value.data_type(),
            &data_type,
            format!("unexpected type for index {index}"),
        );
    }
    Literal::new_compound(
        LiteralValue::Items(values),
        data_type::Compound::List,
        nullable,
        variation,
        vec![data_type],
    )
}

/// Parses a map literal.
fn parse_map(
    x: &substrait::expression::literal::Map,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let values: Vec<_> = proto_required_repeated_field!(x, y, key_values, |x, y| {
        let key = proto_required_field!(x, y, key, parse_literal)
            .1
            .unwrap_or_default();
        let value = proto_required_field!(x, y, value, parse_literal)
            .1
            .unwrap_or_default();
        Ok((key, value))
    })
    .1
    .into_iter()
    .map(|x| x.unwrap_or_default())
    .collect();
    if values.is_empty() {
        comment!(
            y,
            "At least one key-value pair is required to derive types. Use EmptyMap instead."
        );
    }
    let mut key_type = Arc::default();
    let mut value_type = Arc::default();
    for (index, value) in values.iter().enumerate() {
        key_type = types::assert_equal(
            y,
            value.0.data_type(),
            &key_type,
            format!("unexpected key type for index {index}"),
        );
        value_type = types::assert_equal(
            y,
            value.1.data_type(),
            &value_type,
            format!("unexpected value type for index {index}"),
        );
    }
    Literal::new_compound(
        LiteralValue::Pairs(values),
        data_type::Compound::Map,
        nullable,
        variation,
        vec![key_type, value_type],
    )
}

/// Parses an empty list literal.
fn parse_empty_list(
    x: &substrait::r#type::List,
    y: &mut context::Context,
) -> diagnostic::Result<Literal> {
    // FIXME: nullability is redundantly specified, and the type
    // variation reference would be if it had gotten the same
    // treatment as nullability. Why doesn't EmptyList just map to only
    // the element data type?
    types::parse_list(x, y)?;
    Ok(Literal {
        value: LiteralValue::Items(vec![]),
        data_type: y.data_type(),
    })
}

/// Parses an empty map literal.
fn parse_empty_map(
    x: &substrait::r#type::Map,
    y: &mut context::Context,
) -> diagnostic::Result<Literal> {
    // FIXME: same note as for EmptyList.
    types::parse_map(x, y)?;
    Ok(Literal {
        value: LiteralValue::Pairs(vec![]),
        data_type: y.data_type(),
    })
}

/// Parses a null literal.
fn parse_null(x: &substrait::Type, y: &mut context::Context) -> diagnostic::Result<Literal> {
    // FIXME: same note as for EmptyList.
    types::parse_type(x, y)?;
    let data_type = y.data_type();
    if !data_type.nullable() && !data_type.is_unresolved() {
        Err(cause!(
            TypeMismatchedNullability,
            "type of null literal must be nullable"
        ))
    } else {
        Ok(Literal {
            value: LiteralValue::Null,
            data_type: y.data_type(),
        })
    }
}

fn parse_user_defined(
    x: &substrait::expression::literal::UserDefined,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    let extension_type = proto_primitive_field!(
        x,
        y,
        type_reference,
        extensions::simple::parse_type_reference
    )
    .1;
    proto_required_field!(x, y, value, extensions::advanced::parse_functional_any);
    Ok(Literal {
        value: LiteralValue::UserDefined,
        data_type: data_type::DataType::new(
            if let Some(extension_type) = extension_type {
                data_type::Class::UserDefined(extension_type)
            } else {
                data_type::Class::Unresolved
            },
            nullable,
            variation,
            vec![],
        )?,
    })
}

/// Parse a literal value. Returns the parsed literal.
fn parse_literal_type(
    x: &substrait::expression::literal::LiteralType,
    y: &mut context::Context,
    nullable: bool,
    variation: Option<Arc<extension::Reference<extension::TypeVariation>>>,
) -> diagnostic::Result<Literal> {
    use substrait::expression::literal::LiteralType;
    match x {
        LiteralType::Boolean(x) => parse_boolean(x, y, nullable, variation),
        LiteralType::I8(x) => parse_i8(x, y, nullable, variation),
        LiteralType::I16(x) => parse_i16(x, y, nullable, variation),
        LiteralType::I32(x) => parse_i32(x, y, nullable, variation),
        LiteralType::I64(x) => parse_i64(x, y, nullable, variation),
        LiteralType::Fp32(x) => parse_fp32(x, y, nullable, variation),
        LiteralType::Fp64(x) => parse_fp64(x, y, nullable, variation),
        LiteralType::String(x) => parse_string(x, y, nullable, variation),
        LiteralType::Binary(x) => parse_binary(x, y, nullable, variation),
        LiteralType::Timestamp(x) => parse_timestamp(x, y, nullable, variation),
        LiteralType::TimestampTz(x) => parse_timestamp_tz(x, y, nullable, variation),
        LiteralType::Date(x) => parse_date(x, y, nullable, variation),
        LiteralType::Time(x) => parse_time(x, y, nullable, variation),
        LiteralType::IntervalYearToMonth(x) => {
            parse_interval_year_to_month(x, y, nullable, variation)
        }
        LiteralType::IntervalDayToSecond(x) => {
            parse_interval_day_to_second(x, y, nullable, variation)
        }
        LiteralType::Uuid(x) => parse_uuid(x, y, nullable, variation),
        LiteralType::FixedChar(x) => parse_fixed_char(x, y, nullable, variation),
        LiteralType::VarChar(x) => parse_var_char(x, y, nullable, variation),
        LiteralType::FixedBinary(x) => parse_fixed_binary(x, y, nullable, variation),
        LiteralType::Decimal(x) => parse_decimal(x, y, nullable, variation),
        LiteralType::Struct(x) => parse_struct_int(x, y, nullable, variation),
        LiteralType::List(x) => parse_list(x, y, nullable, variation),
        LiteralType::Map(x) => parse_map(x, y, nullable, variation),
        LiteralType::EmptyList(x) => parse_empty_list(x, y),
        LiteralType::EmptyMap(x) => parse_empty_map(x, y),
        LiteralType::Null(x) => parse_null(x, y),
        LiteralType::UserDefined(x) => parse_user_defined(x, y, nullable, variation),
    }
}

/// Parse a literal value. Returns the parsed literal.
pub fn parse_literal(
    x: &substrait::expression::Literal,
    y: &mut context::Context,
) -> diagnostic::Result<Literal> {
    // Whether the nullability and variation attributes are stored in this
    // message or in an embedded type message.
    let attributes_here = !matches!(
        x.literal_type,
        Some(substrait::expression::literal::LiteralType::EmptyList(_))
            | Some(substrait::expression::literal::LiteralType::EmptyMap(_))
            | Some(substrait::expression::literal::LiteralType::Null(_))
    );

    // Parse nullability attribute.
    // FIXME: why isn't the nullability enum used here? Especially
    // considering nullability here actually should be unspecified when
    // above match yields false, while it must be specified everywhere
    // else. Better yet, change the semantics as described in the other
    // fixmes such that it is always mandatory everywhere, and then use
    // a boolean everywhere? If the point of the enum is to allow types
    // to be "partially unresolved," then the type system is pretty
    // fundamentally broken, since overload resolution depends on it.
    if attributes_here {
        proto_primitive_field!(x, y, nullable);
    } else {
        proto_primitive_field!(x, y, nullable, |x, y| {
            // Send diagnostic only when x is not set to its default value,
            // since the default value is indistinguishable from unspecified.
            if *x {
                diagnostic!(
                    y,
                    Info,
                    RedundantField,
                    "this field is inoperative for empty lists, empty maps, and null."
                );
            } else {
                comment!(
                    y,
                    "This field is inoperative for empty lists, empty maps, and null."
                );
            }
            Ok(())
        });
    }

    // Parse variation.
    let variation = if attributes_here {
        proto_primitive_field!(
            x,
            y,
            type_variation_reference,
            extensions::simple::parse_type_variation_reference
        )
        .1
        .flatten()
    } else {
        proto_primitive_field!(x, y, type_variation_reference, |x, y| {
            // Send diagnostic only when x is not set to its default value,
            // since the default value is indistinguishable from unspecified.
            if x != &0 {
                diagnostic!(
                    y,
                    Info,
                    RedundantField,
                    "this field is inoperative for empty lists, empty maps, and null."
                );
            } else {
                comment!(
                    y,
                    "This field is inoperative for empty lists, empty maps, and null."
                );
            }
            Ok(())
        });
        None
    };

    // Parse the literal value.
    let literal = proto_required_field!(
        x,
        y,
        literal_type,
        parse_literal_type,
        x.nullable,
        variation
    )
    .1
    .unwrap_or_default();

    // Describe node.
    y.set_data_type(literal.data_type().clone());
    describe!(y, Expression, "{}", literal);
    summary!(
        y,
        "Literal of type {:#} with value {:#}",
        literal.data_type(),
        literal
    );
    Ok(literal)
}
