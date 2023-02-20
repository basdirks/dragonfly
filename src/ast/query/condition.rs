use crate::parser::{
    camel_case,
    choice,
    colon,
    dollar,
    literal,
    spaces,
    ParseResult,
};

/// The type of a condition.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    /// The value of the field must contain the value of the argument.
    Contains {
        /// The name of the referenced argument.
        argument: String,
    },
    /// The value of the field must equal the value of the argument.
    Equals {
        /// The name of the referenced argument.
        argument: String,
    },
}

impl Type {
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryConditionType;
    ///
    /// assert_eq!(
    ///     QueryConditionType::parse("contains: $foo"),
    ///     Ok((
    ///         QueryConditionType::Contains {
    ///             argument: "foo".to_string(),
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QueryConditionType;
    ///
    /// assert_eq!(
    ///     QueryConditionType::parse("equals: $bar"),
    ///     Ok((
    ///         QueryConditionType::Equals {
    ///             argument: "bar".to_string(),
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QueryConditionType;
    ///
    /// assert!(QueryConditionType::parse("starts_with: $foo").is_err());
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice(
            input,
            vec![
                |input| {
                    let (_, input) = literal(input, "contains")?;
                    let (_, input) = spaces(&input)?;
                    let (_, input) = colon(&input)?;
                    let (_, input) = spaces(&input)?;
                    let (_, input) = dollar(&input)?;
                    let (argument, input) = camel_case(&input)?;

                    Ok((Self::Contains { argument }, input))
                },
                |input| {
                    let (_, input) = literal(input, "equals")?;
                    let (_, input) = spaces(&input)?;
                    let (_, input) = colon(&input)?;
                    let (_, input) = spaces(&input)?;
                    let (_, input) = dollar(&input)?;
                    let (argument, input) = camel_case(&input)?;

                    Ok((Self::Equals { argument }, input))
                },
            ],
        )
    }
}

/// A condition that must be met for a query to return a result.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Condition {
    /// The path to the field that must meet the condition.
    pub field: Vec<String>,
    /// The type of the condition.
    pub r#type: Type,
}

impl Condition {
    /// Return the name of the argument referenced by the condition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryConditionType,
    /// };
    ///
    /// let condition = QueryCondition {
    ///     field: vec!["foo".to_string()],
    ///     r#type: QueryConditionType::Contains {
    ///         argument: "bar".to_string(),
    ///     },
    /// };
    ///
    /// assert_eq!(condition.argument(), "bar");
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryConditionType,
    /// };
    ///
    /// let condition = QueryCondition {
    ///     field: vec!["foo".to_string()],
    ///     r#type: QueryConditionType::Equals {
    ///         argument: "baz".to_string(),
    ///     },
    /// };
    ///
    /// assert_eq!(condition.argument(), "baz");
    /// ```
    #[must_use]
    pub fn argument(&self) -> &str {
        match &self.r#type {
            Type::Contains { argument } | Type::Equals { argument } => argument,
        }
    }
}
