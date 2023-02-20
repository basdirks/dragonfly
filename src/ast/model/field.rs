use crate::{
    ast::r#type::Type,
    parser::{
        alphabetics,
        colon,
        spaces,
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
    ///     Field,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "bar: String";
    ///
    /// let expected = Field {
    ///     name: "bar".to_string(),
    ///     r#type: Type::Scalar(Scalar::String),
    /// };
    ///
    /// assert_eq!(Field::parse(input), Ok((expected, "".to_string())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::{
    ///         Field,
    ///         Scalar,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "baz= Int";
    ///
    /// assert_eq!(
    ///     Field::parse(input),
    ///     Err(ParseError::UnmatchedChar {
    ///         expected: ':',
    ///         actual: '=',
    ///     })
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = alphabetics(input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((Self { name, r#type }, input))
    }
}
