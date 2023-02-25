use {
    crate::{
        ast::r#type::Type,
        parser::{
            camel_case,
            colon,
            spaces,
            ParseResult,
        },
    },
    std::fmt::Display,
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
    /// Returns `ParseError` if the input does not start with a valid field.
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
    ///     name: "bar".to_owned(),
    ///     r#type: Type::Scalar(Scalar::String),
    /// };
    ///
    /// assert_eq!(Field::parse(input), Ok((expected, "".to_owned())));
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
    /// let input = "Baz: Int";
    ///
    /// assert_eq!(
    ///     Field::parse(input),
    ///     Err(ParseError::UnexpectedChar {
    ///         message: "Expected camelCase identifier to start with lowercase \
    ///                   character, found 'B'."
    ///             .to_string(),
    ///         actual: 'B'
    ///     })
    /// );
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
    ///     Err(ParseError::UnexpectedChar {
    ///         message: "Expected character ':', found '='.".to_owned(),
    ///         actual: '=',
    ///     })
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = camel_case(input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((Self { name, r#type }, input))
    }
}

impl Display for Field {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self { name, r#type } = self;

        write!(f, "{name}: {type}")
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::ast::Scalar,
    };

    #[test]
    fn test_display() {
        let field = Field {
            name: "foo".to_owned(),
            r#type: Type::Scalar(Scalar::String),
        };

        assert_eq!(field.to_string(), "foo: String");
    }
}
