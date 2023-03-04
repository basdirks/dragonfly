use crate::parser::{
    brace_close,
    brace_open,
    literal,
    many1,
    pascal_case,
    spaces,
    ParseResult,
};

/// An enumerated type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Enum {
    /// The name of the enum. Used inside models to reference the enum.
    pub name: String,
    /// The variants of the enum.
    pub variants: Vec<String>,
}

impl Enum {
    /// Create a new enum.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    /// * `variants` - The variants of the enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Enum;
    ///
    /// let r#enum = Enum::new("Foo", &["Bar", "Baz"]);
    ///
    /// assert_eq!(r#enum.name, "Foo".to_owned());
    /// assert_eq!(r#enum.variants, vec!["Bar".to_owned(), "Baz".to_owned()]);
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        variants: &[&str],
    ) -> Self {
        Self {
            name: name.to_owned(),
            variants: variants.iter().map(ToString::to_string).collect(),
        }
    }

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
    /// let input = "
    ///
    /// enum Foo {
    ///     Bar
    ///     Baz
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Enum {
    ///     name: "Foo".to_owned(),
    ///     variants: vec!["Bar".to_owned(), "Baz".to_owned()],
    /// };
    ///
    /// assert_eq!(Enum::parse(input), Ok((expected, String::new())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Enum,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// enum Foo {
    ///     bar
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Enum::parse(input),
    ///     Err(ParseError::UnexpectedChar {
    ///         message: "Expected segment of PascalCase identifier to start with \
    ///                   uppercase character, found 'b'."
    ///             .to_string(),
    ///         actual: 'b'
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

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, variants }, input))
    }
}
