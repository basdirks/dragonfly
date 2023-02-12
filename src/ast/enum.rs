use std::collections::HashSet;

use crate::parser::{
    brace_close, brace_open, capitalized, literal, spaces, ParseError, ParseResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    pub name: String,
    pub variants: HashSet<String>,
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
    /// let mut variants = std::collections::HashSet::new();
    ///
    /// variants.insert("Bar".to_string());
    /// variants.insert("Baz".to_string());
    ///
    /// let expected = Enum {
    ///     name: "Foo".to_string(),
    ///     variants,
    /// };
    ///
    /// assert_eq!(Enum::parse(input), Ok((expected, "".to_string())));
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
        let (_, input) = spaces(&input)?;
        let mut variants = HashSet::new();
        let (variant, input) = capitalized(&input)?;

        variants.insert(variant);

        let (_, mut input) = spaces(&input)?;

        while let Ok((variant, new_input)) = capitalized(&input) {
            let (_, new_input) = spaces(&new_input)?;

            if !variants.insert(variant) {
                return Err(ParseError::CustomError {
                    message: "duplicate enum variant".to_string(),
                });
            }

            input = new_input;
        }

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, variants }, input))
    }
}
