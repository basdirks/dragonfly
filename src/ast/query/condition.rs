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
    std::collections::VecDeque,
};

/// The type of a condition.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Operator {
    /// The value of the field must contain the value of the argument.
    Contains,
    /// The value of the field must equal the value of the argument.
    Equals,
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
    ///     Ok((QueryOperator::Contains, ": $foo".to_string()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QueryOperator;
    ///
    /// assert_eq!(
    ///     QueryOperator::parse("equals: $bar"),
    ///     Ok((QueryOperator::Equals, ": $bar".to_string()))
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

/// A condition that must be met for a query to return a result.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Condition {
    /// The path to the field that must meet the condition.
    pub field_path: VecDeque<String>,
    /// The type of the condition.
    pub operator: Operator,
    /// The right-hand side of the condition.
    pub argument: String,
}
