use {
    parser::{
        at,
        capitalized,
        choice,
        literal,
        map,
        tag,
        ParseError,
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

/// Scalar types.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Scalar<'a> {
    /// A boolean.
    Boolean,
    /// A date and time.
    DateTime,
    /// A 64-bit floating point number.
    Float,
    /// A 64-bit integer.
    Int,
    /// A one-to-one or one-to-many reference.
    Owned(Cow<'a, str>),
    /// A many-to-one or many-to-many reference.
    Reference(Cow<'a, str>),
    /// A UTF-8 string.
    String,
}

impl<'a> Scalar<'a> {
    // Parse a scalar type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid scalar
    /// type.
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(
            input,
            vec![
                tag!(literal!("Boolean"), Self::Boolean),
                tag!(literal!("DateTime"), Self::DateTime),
                tag!(literal!("Float"), Self::Float),
                tag!(literal!("Int"), Self::Int),
                tag!(literal!("String"), Self::String),
                map!(
                    |input| {
                        let (_, input) = at(input)?;
                        let (name, input) = capitalized(&input)?;

                        Ok((name.into(), input))
                    },
                    Self::Owned
                ),
                map!(capitalized, |name| Self::Reference(name.into())),
            ],
        )
        .map_err(|_| {
            ParseError::custom(
                "expected one of: Boolean, DateTime, Float, Int, String, \
                 @<capitalized>, <capitalized>",
            )
        })
    }
}

impl Display for Scalar<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Boolean => write!(f, "Boolean"),
            Self::DateTime => write!(f, "DateTime"),
            Self::Float => write!(f, "Float"),
            Self::Int => write!(f, "Int"),
            Self::Owned(name) => write!(f, "@{name}"),
            Self::Reference(name) => write!(f, "{name}"),
            Self::String => write!(f, "String"),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        parser::ParseError,
    };

    #[test]
    fn test_display_boolean() {
        assert_eq!(Scalar::Boolean.to_string(), "Boolean");
    }

    #[test]
    fn test_display_date_time() {
        assert_eq!(Scalar::DateTime.to_string(), "DateTime");
    }

    #[test]
    fn test_display_float() {
        assert_eq!(Scalar::Float.to_string(), "Float");
    }

    #[test]
    fn test_display_int() {
        assert_eq!(Scalar::Int.to_string(), "Int");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Scalar::String.to_string(), "String");
    }

    #[test]
    fn test_display_reference() {
        assert_eq!(Scalar::Reference("Foo".into()).to_string(), "Foo");
    }

    #[test]
    fn test_display_owned() {
        assert_eq!(Scalar::Owned("Foo".into()).to_string(), "@Foo");
    }

    #[test]
    fn test_parse_boolean() {
        assert_eq!(
            Scalar::parse("Boolean"),
            Ok((Scalar::Boolean, String::new()))
        );
    }

    #[test]
    fn test_parse_date_time() {
        assert_eq!(
            Scalar::parse("DateTime"),
            Ok((Scalar::DateTime, String::new()))
        );
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(Scalar::parse("Float"), Ok((Scalar::Float, String::new())));
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(Scalar::parse("Int"), Ok((Scalar::Int, String::new())));
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            Scalar::parse("String"),
            Ok((Scalar::String, String::new()))
        );
    }

    #[test]
    fn test_parse_reference() {
        assert_eq!(
            Scalar::parse("Foo"),
            Ok((Scalar::Reference("Foo".into()), String::new()))
        );
    }

    #[test]
    fn test_parse_owned() {
        assert_eq!(
            Scalar::parse("@Foo"),
            Ok((Scalar::Owned("Foo".into()), String::new()))
        );
    }

    #[test]
    fn test_parse_error() {
        assert_eq!(
            Scalar::parse("!FooBar"),
            Err(ParseError::custom(
                "expected one of: Boolean, DateTime, Float, Int, String, \
                 @<capitalized>, <capitalized>",
            ))
        );
    }
}
