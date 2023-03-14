use {
    super::{
        Query,
        Type,
    },
    parser::{
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

/// A query argument.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Argument<'a> {
    /// The name of the argument. Used inside the conditions of
    /// the where clause.
    pub name: Cow<'a, str>,
    /// The type of the argument.
    pub r#type: Type<'a>,
}

impl<'a> Argument<'a> {
    /// Parse an argument from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid
    /// argument.
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = Query::parse_reference(input)?;
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

impl Display for Argument<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        let Self { name, r#type } = self;

        write!(f, "${name}: {type}")
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Scalar,
    };

    #[test]
    fn test_parse() {
        assert_eq!(
            Argument::parse("$name: String"),
            Ok((
                Argument {
                    name: "name".into(),
                    r#type: Type::Scalar(Scalar::String),
                },
                String::new()
            ))
        );
    }

    #[test]
    fn test_display_boolean() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Scalar(Scalar::Boolean),
            }
            .to_string(),
            "$name: Boolean"
        );
    }

    #[test]
    fn test_display_date_time() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Scalar(Scalar::DateTime),
            }
            .to_string(),
            "$name: DateTime"
        );
    }

    #[test]
    fn test_display_float() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Scalar(Scalar::Float),
            }
            .to_string(),
            "$name: Float"
        );
    }

    #[test]
    fn test_display_int() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Scalar(Scalar::Int),
            }
            .to_string(),
            "$name: Int"
        );
    }

    #[test]
    fn test_display_string() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Scalar(Scalar::String),
            }
            .to_string(),
            "$name: String"
        );
    }

    #[test]
    fn test_display_reference() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Scalar(Scalar::Reference("reference".into())),
            }
            .to_string(),
            "$name: reference"
        );
    }

    #[test]
    fn test_display_owned() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Scalar(Scalar::Owned("reference".into())),
            }
            .to_string(),
            "$name: @reference"
        );
    }

    #[test]
    fn test_display_booleans() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Array(Scalar::Boolean),
            }
            .to_string(),
            "$name: [Boolean]"
        );
    }

    #[test]
    fn test_display_date_times() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Array(Scalar::DateTime),
            }
            .to_string(),
            "$name: [DateTime]"
        );
    }

    #[test]
    fn test_display_floats() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Array(Scalar::Float),
            }
            .to_string(),
            "$name: [Float]"
        );
    }

    #[test]
    fn test_display_ints() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Array(Scalar::Int),
            }
            .to_string(),
            "$name: [Int]"
        );
    }

    #[test]
    fn test_display_strings() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Array(Scalar::String),
            }
            .to_string(),
            "$name: [String]"
        );
    }

    #[test]
    fn test_display_references() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Array(Scalar::Reference("reference".into())),
            }
            .to_string(),
            "$name: [reference]"
        );
    }

    #[test]
    fn test_display_owneds() {
        assert_eq!(
            Argument {
                name: "name".into(),
                r#type: Type::Array(Scalar::Owned("reference".into())),
            }
            .to_string(),
            "$name: [@reference]"
        );
    }
}
