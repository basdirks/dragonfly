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

/// A field belonging to a model.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Field {
    /// The name of the field. Used inside query schemas and where clauses.
    pub name: String,
    /// The type of the field.
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
    /// Returns a `ParseError` if the input does not start with a valid field.
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
