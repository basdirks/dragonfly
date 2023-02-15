use crate::{
    literal,
    map,
    parser::{
        between,
        case::capitalized,
        choice,
        literal,
        map,
        tag,
        ParseResult,
    },
    tag,
};

/// Basic types: primitives and identifiers.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Basic {
    /// A UTF-8 string.
    String,
    /// A 64-bit integer.
    Int,
    /// A 64-bit floating point number.
    Float,
    /// A boolean.
    Boolean,
    /// A user-defined type.
    Identifier(String),
}

impl Basic {
    /// Parse a primitive from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// primitive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::Basic;
    ///
    /// assert_eq!(Basic::parse("String"), Ok((Basic::String, "".to_string())));
    /// assert_eq!(Basic::parse("Int"), Ok((Basic::Int, "".to_string())));
    /// assert_eq!(Basic::parse("Float"), Ok((Basic::Float, "".to_string())));
    ///
    /// assert_eq!(
    ///     Basic::parse("Boolean"),
    ///     Ok((Basic::Boolean, "".to_string()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::Basic;
    ///
    /// assert_eq!(
    ///     Basic::parse("Foo"),
    ///     Ok((Basic::Identifier("Foo".to_string()), "".to_string()))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(
            input,
            vec![
                tag!(literal!("String"), Self::String),
                tag!(literal!("Int"), Self::Int),
                tag!(literal!("Float"), Self::Float),
                tag!(literal!("Boolean"), Self::Boolean),
                map!(capitalized, Self::Identifier),
            ],
        )
    }
}

/// A type: either a basic type or an array.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    /// An array of a basic type.
    Array(Basic),
    /// A basic type.
    One(Basic),
}

impl Type {
    fn parse_one(input: &str) -> ParseResult<Self> {
        map(input, Basic::parse, Self::One)
    }

    fn parse_array(input: &str) -> ParseResult<Self> {
        let (primitive, input) = between(input, "[", Basic::parse, "]")?;

        Ok((Self::Array(primitive), input))
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
    ///     ast::r#type::{
    ///         Basic,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(
    ///     Type::parse("String"),
    ///     Ok((Type::One(Basic::String), "".to_string()))
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("Int"),
    ///     Ok((Type::One(Basic::Int), "".to_string()))
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("Float"),
    ///     Ok((Type::One(Basic::Float), "".to_string()))
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("Boolean"),
    ///     Ok((Type::One(Basic::Boolean), "".to_string()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::{
    ///     Basic,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Type::parse("[String]"),
    ///     Ok((Type::Array(Basic::String), "".to_string())),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::{
    ///     Basic,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Type::parse("[Foo]"),
    ///     Ok((
    ///         Type::Array(Basic::Identifier("Foo".to_string())),
    ///         "".to_string()
    ///     )),
    /// );
    /// ```
    ///
    /// Nested arrays are not supported:
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::Type;
    ///
    /// assert!(Type::parse("[[String]]").is_err());
    /// ```
    ///
    /// A type name must start with an uppercase letter:
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::Type;
    ///
    /// assert!(Type::parse("foo").is_err());
    /// ```
    ///
    /// An empty string is not a valid type:
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::Type;
    ///
    /// assert!(Type::parse("").is_err());
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(input, vec![Self::parse_one, Self::parse_array])
    }
}
