use {
    parser::{
        brace_close,
        brace_open,
        literal,
        pascal_case,
        spaces,
        ParseError,
        ParseResult,
    },
    std::borrow::Cow,
    token_set::TokenSet,
};

/// An enumerated type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Enum<'a> {
    /// The name of the enum. Used inside models to reference the enum.
    pub name: Cow<'a, str>,
    /// The values of the enum.
    pub values: TokenSet,
}

impl<'a> Enum<'a> {
    /// Parse an enum value from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid enum
    /// value.
    fn parse_value(input: &str) -> ParseResult<String> {
        let (value, input) = pascal_case(input)?;
        let (_, input) = spaces(&input)?;

        Ok((value, input))
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
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "enum")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = pascal_case(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, mut input) = spaces(&input)?;
        let mut values = TokenSet::new();

        while let Ok((value, new_input)) = Self::parse_value(&input) {
            if !values.insert(value) {
                return Err(ParseError::custom("Duplicate enum value."));
            }

            input = new_input;
        }

        if values.is_empty() {
            return Err(ParseError::custom(format!(
                "Enum `{name}` has no values."
            )));
        }

        let (_, input) = brace_close(&input)?;

        Ok((
            Self {
                name: name.into(),
                values,
            },
            input,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "
        
        enum Foo {
            Bar
            Baz
        }
        
        "
        .trim();

        let expected = Enum {
            name: "Foo".into(),
            values: TokenSet::from_iter(["Bar", "Baz"]),
        };

        assert_eq!(Enum::parse(input), Ok((expected, String::new())));
    }

    #[test]
    fn test_parse_duplicate_value() {
        let input = "
        
        enum Foo {
            Bar
            Bar
        }
        
        "
        .trim();

        assert_eq!(
            Enum::parse(input),
            Err(ParseError::custom("Duplicate enum value."))
        );
    }

    #[test]
    fn test_parse_no_fields() {
        let input = "
        
        enum Foo {
        }
        
        "
        .trim();

        assert_eq!(
            Enum::parse(input),
            Err(ParseError::custom("Enum `Foo` has no values."))
        );
    }
}
