use {
    crate::parser::{
        brace_close,
        brace_open,
        capitalized,
        literal,
        many1,
        spaces,
        ParseError,
        ParseResult,
    },
    std::collections::HashSet,
};

/// An enumerated type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    /// The name of the enum. Used inside models to reference the enum.
    pub name: String,
    /// The variants of the enum.
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
    /// Returns a `ParseError` if the input does not start with a valid enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Enum,
    ///     parser::ParseError,
    /// };
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
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::r#enum::Enum,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "enum Foo {
    ///     bar
    /// }";
    ///
    /// assert_eq!(
    ///     Enum::parse(input),
    ///     Err(ParseError::UnmetPredicate {
    ///         message: "character is not uppercase".to_string(),
    ///         actual: 'b'
    ///     })
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::r#enum::Enum,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "enum Foo {
    ///     Bar
    ///     Bar
    /// }";
    ///
    /// assert_eq!(
    ///     Enum::parse(input),
    ///     Err(ParseError::CustomError {
    ///         message: "duplicate enum variant".to_string(),
    ///         input: "\n}".to_string()
    ///     })
    /// );
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

            Ok((variant, input))
        })?;

        if variants.len() != variants.iter().collect::<HashSet<_>>().len() {
            return Err(ParseError::CustomError {
                message: "duplicate enum variant".to_string(),
                input,
            });
        }

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, variants }, input))
    }
}
