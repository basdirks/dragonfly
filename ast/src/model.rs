pub use self::field::Field;
use {
    ord_str_map::OrdStrMap,
    parser::{
        brace_close,
        brace_open,
        capitalized,
        literal,
        spaces,
        ParseError,
        ParseResult,
    },
    std::borrow::Cow,
};

/// A field belonging to a model.
pub mod field;

/// A model describes an entity. It has a name and one or more fields.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Model<'a> {
    /// The name of the model. Used as query return type and inside other
    /// models to reference the model.
    pub name: Cow<'a, str>,
    /// The fields of the model.
    pub fields: OrdStrMap<Field<'a>>,
}

impl<'a> Model<'a> {
    /// Parse a model from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * Returns a `ParseError` if the input does not contain a valid model.
    /// * Returns a `ParseError` if the model contains duplicate field names.
    /// * Returns a `ParseError` if the model does not contain any fields.
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "model")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = capitalized(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, mut input) = spaces(&input)?;
        let mut fields = OrdStrMap::new();

        while let Ok((field, new_input)) = Field::parse(&input) {
            let (_, new_input) = spaces(&new_input)?;

            if fields.insert(field.name.clone(), field.clone()).is_some() {
                return Err(ParseError::custom(format!(
                    "Duplicate field name `{}` in model `{name}`.",
                    field.name
                )));
            }

            input = new_input;
        }

        if fields.is_empty() {
            return Err(ParseError::custom(format!(
                "Expected at least one field in model `{name}`."
            )));
        }

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((
            Self {
                name: name.into(),
                fields,
            },
            input,
        ))
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
    };

    #[test]
    fn test_parse() {
        let input = "
        
model Foo {
    bar: String
    baz: Int
    qux: [Bar]
    quy: @Bar
    quz: [@Bar]
}
        
        "
        .trim();

        let expected = Model {
            name: "Foo".into(),
            fields: OrdStrMap::from_iter([
                (
                    "bar",
                    Field {
                        name: "bar".into(),
                        r#type: Type::Scalar(Scalar::String),
                    },
                ),
                (
                    "baz",
                    Field {
                        name: "baz".into(),
                        r#type: Type::Scalar(Scalar::Int),
                    },
                ),
                (
                    "qux",
                    Field {
                        name: "qux".into(),
                        r#type: Type::Array(Scalar::Reference("Bar".into())),
                    },
                ),
                (
                    "quy",
                    Field {
                        name: "quy".into(),
                        r#type: Type::Scalar(Scalar::Owned("Bar".into())),
                    },
                ),
                (
                    "quz",
                    Field {
                        name: "quz".into(),
                        r#type: Type::Array(Scalar::Owned("Bar".into())),
                    },
                ),
            ]),
        };

        assert_eq!(Model::parse(input), Ok((expected, String::new())));
    }

    #[test]
    fn test_parse_duplicate_field_name() {
        let input = "
        
model Foo {
    bar: String
    bar: Int
}
        
        "
        .trim();

        assert_eq!(
            Model::parse(input),
            Err(ParseError::custom(
                "Duplicate field name `bar` in model `Foo`."
            ))
        );
    }

    #[test]
    fn test_parse_no_fields() {
        let input = "
        
model Foo {
}
        
        "
        .trim();

        assert_eq!(
            Model::parse(input),
            Err(ParseError::custom(
                "Expected at least one field in model `Foo`."
            ))
        );
    }
}
