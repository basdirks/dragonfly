use crate::parser::{brace_close, brace_open, capitalized, literal, many1, spaces, ParseResult};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variants: Vec<String>,
}

impl Enum {
    /// Parse an enum from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::r#enum::Enum;
    ///
    /// let input = "enum Foo {
    ///     Bar
    ///     Baz
    /// }";
    ///
    /// let expected = Enum {
    ///     name: "Foo".to_string(),
    ///     variants: vec!["Bar".to_string(), "Baz".to_string()],
    /// };
    ///
    /// assert_eq!(Enum::parse(input), Ok((expected, "".to_string())));
    ///
    /// let input = "enum Foo {
    ///     bar
    /// }";
    ///
    /// assert!(Enum::parse(input).is_err());
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "enum")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = capitalized(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (variants, input) = many1(&input, |input| {
            let (_, input) = spaces(input)?;
            let (variant, input) = capitalized(&input)?;
            let (_, input) = spaces(&input)?;

            Ok((variant, input))
        })?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, variants }, input))
    }
}
