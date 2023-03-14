use {
    parser::{
        choice,
        literal,
        tag,
        ParseResult,
    },
    std::fmt::{
        self,
        Display,
        Formatter,
    },
};

/// The type of a condition.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Operator {
    /// The value of the field must contain the value of the argument.
    Contains,
    /// The value of the field must equal the value of the argument.
    Equals,
}

impl Display for Operator {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Contains => write!(f, "contains"),
            Self::Equals => write!(f, "equals"),
        }
    }
}

impl Operator {
    /// Parse a condition type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// condition type.
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice(
            input,
            vec![
                tag!(literal!("contains"), Self::Contains),
                tag!(literal!("equals"), Self::Equals),
            ],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(Operator::Contains.to_string(), "contains");
        assert_eq!(Operator::Equals.to_string(), "equals");
    }

    #[test]
    fn test_parse_contains() {
        assert_eq!(
            Operator::parse("contains: $foo"),
            Ok((Operator::Contains, ": $foo".to_owned()))
        );
    }

    #[test]
    fn test_parse_equals() {
        assert_eq!(
            Operator::parse("equals: $bar"),
            Ok((Operator::Equals, ": $bar".to_owned()))
        );
    }

    #[test]
    fn test_parse_error() {
        assert!(Operator::parse("starts_with: $foo").is_err());
    }
}
