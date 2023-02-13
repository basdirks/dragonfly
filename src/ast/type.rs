use crate::{
    literal, map,
    parser::{between, capitalized, choice, literal, map, tag, ParseResult},
    tag,
};

/// Primitive types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Primitive {
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

impl Primitive {
    /// Parse a primitive from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid primitive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::Primitive;
    ///
    /// assert_eq!(Primitive::parse("String"), Ok((Primitive::String, "".to_string())));
    /// assert_eq!(Primitive::parse("Int"), Ok((Primitive::Int, "".to_string())));
    /// assert_eq!(Primitive::parse("Float"), Ok((Primitive::Float, "".to_string())));
    /// assert_eq!(Primitive::parse("Boolean"), Ok((Primitive::Boolean, "".to_string())));
    ///
    /// assert_eq!(Primitive::parse("Foo"), Ok((Primitive::Identifier("Foo".to_string()), "".to_string())));
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

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    Array(Primitive),
    One(Primitive),
}

impl Type {
    fn parse_one(input: &str) -> ParseResult<Self> {
        map(input, Primitive::parse, Self::One)
    }

    fn parse_array(input: &str) -> ParseResult<Self> {
        let (primitive, input) = between(input, "[", Primitive::parse, "]")?;

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
    /// * If the input is not a valid type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::parser::ParseError;
    /// use dragonfly::ast::r#type::{Primitive, Type};
    ///
    /// assert_eq!(Type::parse("String"), Ok((Type::One(Primitive::String), "".to_string())));
    /// assert_eq!(Type::parse("Int"), Ok((Type::One(Primitive::Int), "".to_string())));
    /// assert_eq!(Type::parse("Float"), Ok((Type::One(Primitive::Float), "".to_string())));
    /// assert_eq!(Type::parse("Boolean"), Ok((Type::One(Primitive::Boolean), "".to_string())));
    ///
    /// assert_eq!(
    ///     Type::parse("[String]"),
    ///     Ok((Type::Array(Primitive::String), "".to_string())),
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("[Foo]"),
    ///     Ok((Type::Array(Primitive::Identifier("Foo".to_string())), "".to_string())),
    /// );
    ///
    /// // Nested arrays are not supported.
    /// assert!(Type::parse("[[String]]").is_err());
    ///
    /// // A type name must start with an uppercase letter.
    /// assert!(Type::parse("foo").is_err());
    ///
    /// // An empty string is not a valid type.
    /// assert!(Type::parse("").is_err());
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(input, vec![Self::parse_one, Self::parse_array])
    }
}
