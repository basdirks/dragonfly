use {
    crate::{
        literal,
        parser::{
            choice,
            literal,
            tag,
            ParseResult,
        },
        tag,
    },
    std::fmt::Display,
};

/// The type of a condition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Operator {
    /// The value of the field must contain the value of the argument.
    Contains,
    /// The value of the field must equal the value of the argument.
    Equals,
}

impl Display for Operator {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
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
    /// Returns `ParseError` if the input does not start with a valid
    /// condition type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryOperator;
    ///
    /// assert_eq!(
    ///     QueryOperator::parse("contains: $foo"),
    ///     Ok((QueryOperator::Contains, ": $foo".to_owned()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QueryOperator;
    ///
    /// assert_eq!(
    ///     QueryOperator::parse("equals: $bar"),
    ///     Ok((QueryOperator::Equals, ": $bar".to_owned()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QueryOperator;
    ///
    /// assert!(QueryOperator::parse("starts_with: $foo").is_err());
    /// ```
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
}
