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
    std::{
        collections::VecDeque,
        fmt::Display,
    },
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

impl Display for Condition {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            field_path,
            operator,
            argument,
        } = self;

        let mut path = String::new();

        for (index, field) in field_path.iter().enumerate() {
            if index > 0 {
                path.push_str(&format!(" {{ {field} }}"));
            } else {
                path.push_str(field);
            }
        }

        write!(f, "{path} {operator} {argument}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn display_condition_operator() {
        assert_eq!(Operator::Contains.to_string(), "contains");
        assert_eq!(Operator::Equals.to_string(), "equals");
    }

    #[test]
    fn display_condition() {
        assert_eq!(
            Condition {
                field_path: vec!["foo".to_string(), "bar".to_string()]
                    .into_iter()
                    .collect(),
                operator: Operator::Contains,
                argument: "$baz".to_string(),
            }
            .to_string(),
            "foo { bar } contains $baz"
        );

        assert_eq!(
            Condition {
                field_path: vec![
                    "foo".to_string(),
                    "bar".to_string(),
                    "baz".to_string()
                ]
                .into_iter()
                .collect(),
                operator: Operator::Equals,
                argument: "$baz".to_string(),
            }
            .to_string(),
            "foo { bar { baz } } equals $baz"
        );
    }
}
