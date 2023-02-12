use crate::parser::{chars_if, colon, spaces, ParseResult};

use super::super::r#type::Type;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    pub name: String,
    pub r#type: Type,
}

impl Field {
    /// Parse a field from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::{Primitive, Type};
    /// use dragonfly::ast::model::field::Field;
    ///
    /// let input = "bar: String";
    ///
    /// let expected = Field {
    ///     name: "bar".to_string(),
    ///     r#type: Type::One(Primitive::String),
    /// };
    ///
    /// assert_eq!(Field::parse(input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = chars_if(input, char::is_alphabetic)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((Self { name, r#type }, input))
    }
}
