pub use self::scalar::Scalar;
use {
    parser::{
        between,
        choice,
        map,
        ParseResult,
    },
    std::fmt::{
        self,
        Display,
        Formatter,
    },
};

/// A scalar type.
pub mod scalar;

/// A type: a scalar, a reference to a model or enum, or an array.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Type<'a> {
    /// An array of scalars.
    Array(Scalar<'a>),
    /// A basic type.
    Scalar(Scalar<'a>),
}

impl Display for Type<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Array(scalar) => write!(f, "[{scalar}]"),
            Self::Scalar(scalar) => write!(f, "{scalar}"),
        }
    }
}

impl<'a> Type<'a> {
    /// Parse a scalar type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid scalar
    /// type.
    fn parse_scalar(input: &str) -> ParseResult<Self> {
        map(input, Scalar::parse, Self::Scalar)
    }

    /// Parse an array type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid array
    /// type.
    fn parse_array(input: &str) -> ParseResult<Self> {
        let (scalar, input) = between(input, "[", Scalar::parse, "]")?;

        Ok((Self::Array(scalar), input))
    }

    /// Parse a type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid type.
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(input, vec![Self::parse_scalar, Self::parse_array])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_boolean() {
        assert_eq!(
            Type::parse("Boolean"),
            Ok((Type::Scalar(Scalar::Boolean), String::new()))
        );
    }

    #[test]
    fn test_parse_date_time() {
        assert_eq!(
            Type::parse("DateTime"),
            Ok((Type::Scalar(Scalar::DateTime), String::new()))
        );
    }

    #[test]
    fn test_parse_float() {
        assert_eq!(
            Type::parse("Float"),
            Ok((Type::Scalar(Scalar::Float), String::new()))
        );
    }

    #[test]
    fn test_parse_int() {
        assert_eq!(
            Type::parse("Int"),
            Ok((Type::Scalar(Scalar::Int), String::new()))
        );
    }

    #[test]
    fn test_parse_string() {
        assert_eq!(
            Type::parse("String"),
            Ok((Type::Scalar(Scalar::String), String::new()))
        );
    }

    #[test]
    fn test_parse_reference() {
        assert_eq!(
            Type::parse("Foo"),
            Ok((Type::Scalar(Scalar::Reference("Foo".into())), String::new()))
        );
    }

    #[test]
    fn test_parse_owned() {
        assert_eq!(
            Type::parse("@Foo"),
            Ok((Type::Scalar(Scalar::Owned("Foo".into())), String::new()))
        );
    }

    #[test]
    fn test_parse_boolean_array() {
        assert_eq!(
            Type::parse("[Boolean]"),
            Ok((Type::Array(Scalar::Boolean), String::new()))
        );
    }

    #[test]
    fn test_parse_date_time_array() {
        assert_eq!(
            Type::parse("[DateTime]"),
            Ok((Type::Array(Scalar::DateTime), String::new()))
        );
    }

    #[test]
    fn test_parse_float_array() {
        assert_eq!(
            Type::parse("[Float]"),
            Ok((Type::Array(Scalar::Float), String::new()))
        );
    }

    #[test]
    fn test_parse_int_array() {
        assert_eq!(
            Type::parse("[Int]"),
            Ok((Type::Array(Scalar::Int), String::new()))
        );
    }

    #[test]
    fn test_parse_string_array() {
        assert_eq!(
            Type::parse("[String]"),
            Ok((Type::Array(Scalar::String), String::new()))
        );
    }

    #[test]
    fn test_parse_reference_array() {
        assert_eq!(
            Type::parse("[Foo]"),
            Ok((Type::Array(Scalar::Reference("Foo".into())), String::new()))
        );
    }

    #[test]
    fn test_parse_owned_array() {
        assert_eq!(
            Type::parse("[@Foo]"),
            Ok((Type::Array(Scalar::Owned("Foo".into())), String::new()))
        );
    }

    #[test]
    fn test_display_boolean() {
        assert_eq!(Type::Scalar(Scalar::Boolean).to_string(), "Boolean");
    }

    #[test]
    fn test_display_date_time() {
        assert_eq!(Type::Scalar(Scalar::DateTime).to_string(), "DateTime");
    }

    #[test]
    fn test_display_int() {
        assert_eq!(Type::Scalar(Scalar::Int).to_string(), "Int");
    }

    #[test]
    fn test_display_float() {
        assert_eq!(Type::Scalar(Scalar::Float).to_string(), "Float");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Type::Scalar(Scalar::String).to_string(), "String");
    }

    #[test]
    fn test_display_reference() {
        assert_eq!(
            Type::Scalar(Scalar::Reference("Foo".into())).to_string(),
            "Foo"
        );
    }

    #[test]
    fn test_display_owned() {
        assert_eq!(
            Type::Scalar(Scalar::Owned("Foo".into())).to_string(),
            "@Foo"
        );
    }

    #[test]
    fn test_display_boolean_array() {
        assert_eq!(Type::Array(Scalar::Boolean).to_string(), "[Boolean]");
    }

    #[test]
    fn test_display_date_time_array() {
        assert_eq!(Type::Array(Scalar::DateTime).to_string(), "[DateTime]");
    }

    #[test]
    fn test_display_int_array() {
        assert_eq!(Type::Array(Scalar::Int).to_string(), "[Int]");
    }

    #[test]
    fn test_display_float_array() {
        assert_eq!(Type::Array(Scalar::Float).to_string(), "[Float]");
    }

    #[test]
    fn test_display_string_array() {
        assert_eq!(Type::Array(Scalar::String).to_string(), "[String]");
    }

    #[test]
    fn test_display_reference_array() {
        assert_eq!(
            Type::Array(Scalar::Reference("Foo".into())).to_string(),
            "[Foo]"
        );
    }

    #[test]
    fn test_display_owned_array() {
        assert_eq!(
            Type::Array(Scalar::Owned("Foo".into())).to_string(),
            "[@Foo]"
        );
    }
}
