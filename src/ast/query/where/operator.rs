use {
    crate::{
        ast::Type,
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

    /// Check compatibility between an operator and two operand types.
    ///
    /// # Arguments
    ///
    /// * `lhs` - The type of the left-hand side of the operator.
    /// * `rhs` - The type of the right-hand side of the operator.
    ///
    /// # Examples
    ///
    /// An array cannot contain a scalar of a different type.
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryOperator,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert!(!QueryOperator::Contains.check_operands(
    ///     &Type::Array(Scalar::String),
    ///     &Type::Scalar(Scalar::Int),
    /// ));
    ///
    /// assert!(QueryOperator::Contains.check_operands(
    ///     &Type::Array(Scalar::String),
    ///     &Type::Scalar(Scalar::String),
    /// ));
    /// ```
    ///
    /// A value can only equal a value of the same type.
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryOperator,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert!(!QueryOperator::Equals.check_operands(
    ///     &Type::Scalar(Scalar::String),
    ///     &Type::Scalar(Scalar::Int),
    /// ));
    ///
    /// assert!(QueryOperator::Equals.check_operands(
    ///     &Type::Scalar(Scalar::String),
    ///     &Type::Scalar(Scalar::String),
    /// ));
    ///
    /// assert!(QueryOperator::Equals.check_operands(
    ///     &Type::Array(Scalar::String),
    ///     &Type::Array(Scalar::String),
    /// ));
    /// ```
    #[must_use]
    pub fn check_operands(
        self,
        lhs: &Type,
        rhs: &Type,
    ) -> bool {
        match self {
            Self::Contains => {
                match (lhs, rhs) {
                    (Type::Array(a), Type::Scalar(b)) => a == b,
                    _ => false,
                }
            }
            Self::Equals => lhs == rhs,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_condition_operator() {
        assert_eq!(Operator::Contains.to_string(), "contains");
        assert_eq!(Operator::Equals.to_string(), "equals");
    }
}
