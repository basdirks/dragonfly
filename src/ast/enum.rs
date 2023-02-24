use {
    crate::parser::{
        brace_close,
        brace_open,
        literal,
        many1,
        pascal_case,
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
    /// Returns `ParseError` if the input does not start with a valid enum.
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
    ///     name: "Foo".to_owned(),
    ///     variants: vec!["Bar".to_owned(), "Baz".to_owned()],
    /// };
    ///
    /// assert_eq!(Enum::parse(input), Ok((expected, "".to_owned())));
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
    ///     Err(ParseError::UnexpectedChar {
    ///         message: "expected segment of PascalCase identifier to start with \
    ///                   uppercase character"
    ///             .to_string(),
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
    ///         message: "duplicate enum variant".to_owned(),
    ///         input: "\n}".to_owned()
    ///     })
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "enum")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = pascal_case(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;

        let (variants, input) = many1(&input, |input| {
            let (_, input) = spaces(input)?;
            let (variant, input) = pascal_case(&input)?;

            Ok((variant, input))
        })?;

        if variants.len() != variants.iter().collect::<HashSet<_>>().len() {
            return Err(ParseError::CustomError {
                message: "duplicate enum variant".to_owned(),
                input,
            });
        }

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, variants }, input))
    }
}
