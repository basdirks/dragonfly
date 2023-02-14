use std::collections::HashSet;

use crate::parser::{
    brace_close, brace_open, capitalized, literal, many1, spaces, ParseError, ParseResult,
};

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
    /// use dragonfly::parser::ParseError;
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
    /// use dragonfly::parser::ParseError;
    /// use dragonfly::ast::r#enum::Enum;
    ///
    /// let input = "enum Foo {
    ///     bar
    /// }";
    ///
    /// assert_eq!(
    ///     Enum::parse(input),
    ///     Err(ParseError::UnmatchedCharPredicate {
    ///         description: "should be uppercase".to_string(),
    ///         actual: 'b'
    ///     })
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::parser::ParseError;
    /// use dragonfly::ast::r#enum::Enum;
    ///
    /// let input = "enum Foo {
    ///     Bar
    ///     Bar
    /// }";
    ///
    /// assert_eq!(
    ///     Enum::parse(input),
    ///     Err(ParseError::CustomError {
    ///         message: "duplicate enum variant".to_string()
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
            });
        }

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, variants }, input))
    }
}
