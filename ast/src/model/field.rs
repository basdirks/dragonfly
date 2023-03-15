use {
    crate::Type,
    parser::{
        camel_case,
        colon,
        spaces,
        ParseResult,
    },
    std::{
        borrow::Cow,
        fmt::{
            self,
            Display,
            Formatter,
        },
    },
};

/// A field belonging to a model.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Field<'a> {
    /// The name of the field. Used inside query schemas and where clauses.
    pub name: Cow<'a, str>,
    /// The type of the field.
    pub r#type: Type<'a>,
}

impl<'a> Field<'a> {
    /// Parse a field from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid field.
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = camel_case(input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((
            Self {
                name: name.into(),
                r#type,
            },
            input,
        ))
    }
}

impl Display for Field<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        let Self { name, r#type } = self;

        write!(f, "{name}: {type}")
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            r#type::Scalar,
            Type,
        },
        parser::ParseError,
    };

    #[test]
    fn test_display() {
        let field = Field {
            name: "foo".into(),
            r#type: Type::Scalar(Scalar::String),
        };

        assert_eq!(field.to_string(), "foo: String");
    }

    #[test]
    fn test_parse() {
        let input = "foo: String";

        let expected = Field {
            name: "foo".into(),
            r#type: Type::Scalar(Scalar::String),
        };

        assert_eq!(Field::parse(input), Ok((expected, String::new())));
    }

    #[test]
    fn test_parse_camel_case() {
        let input = "Baz: Int";

        assert_eq!(
            Field::parse(input),
            Err(ParseError::unexpected_char(
                'B',
                "Expected camelCase identifier to start with lowercase \
                 character, found 'B'."
            ))
        );
    }

    #[test]
    fn test_parse_colon() {
        let input = "baz= Int";

        assert_eq!(
            Field::parse(input),
            Err(ParseError::UnexpectedChar {
                message: "Expected character ':', found '='.".into(),
                actual: '=',
            })
        );
    }

    #[test]
    fn test_parse_owned() {
        assert_eq!(
            Field::parse("foo: @Bar"),
            Ok((
                Field {
                    name: "foo".into(),
                    r#type: Type::Scalar(Scalar::Owned("Bar".into()))
                },
                String::new(),
            ))
        );
    }
}
