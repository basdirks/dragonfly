use crate::{
    literal,
    map,
    parser::{
        between,
        capitalized,
        choice,
        literal,
        map,
        tag,
        ParseResult,
    },
    tag,
};

/// Scalar types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Scalar {
    /// A boolean.
    Boolean,
    /// A date and time.
    DateTime,
    /// A 64-bit floating point number.
    Float,
    /// A 64-bit integer.
    Int,
    /// A reference to an enum or model. Might deserve its own type.
    Reference(String),
    /// A UTF-8 string.
    String,
}

impl Scalar {
    /// Parse a scalar type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// scalar type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Scalar;
    ///
    /// assert_eq!(
    ///     Scalar::parse("Boolean"),
    ///     Ok((Scalar::Boolean, "".to_string()))
    /// );
    ///
    /// assert_eq!(
    ///     Scalar::parse("DateTime"),
    ///     Ok((Scalar::DateTime, "".to_string()))
    /// );
    ///
    /// assert_eq!(Scalar::parse("Float"), Ok((Scalar::Float, "".to_string())));
    /// assert_eq!(Scalar::parse("Int"), Ok((Scalar::Int, "".to_string())));
    ///
    /// assert_eq!(
    ///     Scalar::parse("String"),
    ///     Ok((Scalar::String, "".to_string()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::Scalar;
    ///
    /// assert_eq!(
    ///     Scalar::parse("Foo"),
    ///     Ok((Scalar::Reference("Foo".to_string()), "".to_string()))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(
            input,
            vec![
                tag!(literal!("Boolean"), Self::Boolean),
                tag!(literal!("DateTime"), Self::DateTime),
                tag!(literal!("Float"), Self::Float),
                tag!(literal!("Int"), Self::Int),
                tag!(literal!("String"), Self::String),
                map!(capitalized, Self::Reference),
            ],
        )
    }
}

/// A type: a scalar, a reference to a model or enum, or an array.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    /// An array of scalars.
    Array(Scalar),
    /// A basic type.
    Scalar(Scalar),
}

impl Type {
    fn parse_one(input: &str) -> ParseResult<Self> {
        map(input, Scalar::parse, Self::Scalar)
    }

    fn parse_array(input: &str) -> ParseResult<Self> {
        let (scalar, input) = between(input, "[", Scalar::parse, "]")?;

        Ok((Self::Array(scalar), input))
    }

    /// Parse a type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::{
    ///         Scalar,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(
    ///     Type::parse("String"),
    ///     Ok((Type::Scalar(Scalar::String), "".to_string()))
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("Int"),
    ///     Ok((Type::Scalar(Scalar::Int), "".to_string()))
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("Float"),
    ///     Ok((Type::Scalar(Scalar::Float), "".to_string()))
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("Boolean"),
    ///     Ok((Type::Scalar(Scalar::Boolean), "".to_string()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Type::parse("[String]"),
    ///     Ok((Type::Array(Scalar::String), "".to_string())),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Type::parse("[Foo]"),
    ///     Ok((
    ///         Type::Array(Scalar::Reference("Foo".to_string())),
    ///         "".to_string()
    ///     )),
    /// );
    /// ```
    ///
    /// Nested arrays are not supported:
    ///
    /// ```rust
    /// use dragonfly::ast::Type;
    ///
    /// assert!(Type::parse("[[String]]").is_err());
    /// ```
    ///
    /// A type name must start with an uppercase letter:
    ///
    /// ```rust
    /// use dragonfly::ast::Type;
    ///
    /// assert!(Type::parse("foo").is_err());
    /// ```
    ///
    /// An empty string is not a valid type:
    ///
    /// ```rust
    /// use dragonfly::ast::Type;
    ///
    /// assert!(Type::parse("").is_err());
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(input, vec![Self::parse_one, Self::parse_array])
    }
}
