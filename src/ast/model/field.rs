use crate::{
    ast::r#type::Type,
    parser::{
        char::colon,
        char_range::{
            alphabetics,
            spaces,
        },
        ParseResult,
    },
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    /// use dragonfly::ast::{
    ///     model::field::Field,
    ///     r#type::{
    ///         Basic,
    ///         Type,
    ///     },
    /// };
    ///
    /// let input = "bar: String";
    ///
    /// let expected = Field {
    ///     name: "bar".to_string(),
    ///     r#type: Type::One(Basic::String),
    /// };
    ///
    /// assert_eq!(Field::parse(input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = alphabetics(input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((Self { name, r#type }, input))
    }
}
