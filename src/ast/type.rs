use crate::parser::{alphabetic, between, choice, literal, many, uppercase, ParseResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Primitive {
    String,
    Int,
    Float,
    Boolean,
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
                |input| literal(input, "String").map(|(_, rem)| (Self::String, rem)),
                |input| literal(input, "Int").map(|(_, rem)| (Self::Int, rem)),
                |input| literal(input, "Float").map(|(_, rem)| (Self::Float, rem)),
                |input| literal(input, "Boolean").map(|(_, rem)| (Self::Boolean, rem)),
                |input| {
                    let (head, input) = uppercase(input)?;
                    let (tail, input) = many(&input, alphabetic)?;

                    Ok((
                        Self::Identifier(format!("{head}{}", tail.iter().collect::<String>())),
                        input,
                    ))
                },
            ],
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Array(Primitive),
    One(Primitive),
}

impl Type {
    fn parse_one(input: &str) -> ParseResult<Self> {
        Primitive::parse(input).map(|(primitive, input)| (Self::One(primitive), input))
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
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(input, vec![Self::parse_one, Self::parse_array])
    }
}
