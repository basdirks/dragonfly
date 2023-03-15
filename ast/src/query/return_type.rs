use {
    crate::{
        r#type,
        Type,
    },
    parser::{
        spaces,
        ParseError,
        ParseResult,
    },
    std::borrow::Cow,
};

/// The return type of a query. Must be a model or an array of such a type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ReturnType<'a> {
    /// The name of a model.
    Model(Cow<'a, str>),
    /// An array of a model.
    Array(Cow<'a, str>),
}

impl<'a> ReturnType<'a> {
    /// Parse a return type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid return
    /// type.
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (r#type, input) = Type::parse(input)?;
        let (_, input) = spaces(&input)?;

        match r#type {
            Type::Scalar(r#type::Scalar::Reference(name)) => {
                Ok((Self::Model(name), input))
            }
            Type::Array(r#type::Scalar::Reference(name)) => {
                Ok((Self::Array(name), input))
            }
            _ => {
                Err(ParseError::Custom {
                    message: format!("Expected return type, found `{type}`."),
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_model() {
        assert_eq!(
            ReturnType::parse("Foo"),
            Ok((ReturnType::Model("Foo".into()), String::new()))
        );
    }

    #[test]
    fn test_parse_array() {
        assert_eq!(
            ReturnType::parse("[Foo]"),
            Ok((ReturnType::Array("Foo".into()), String::new()))
        );
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(
            ReturnType::parse("String"),
            Err(ParseError::Custom {
                message: "Expected return type, found `String`.".into(),
            })
        );
    }
}
