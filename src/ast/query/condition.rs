use {
    super::Query,
    crate::parser::{
        choice,
        colon,
        literal,
        spaces,
        ParseResult,
    },
};

/// A condition that must be met for a query to return a result.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Condition {
    /// The value of the given field must contain value provided by the
    /// referenced argument.
    Contains(String),
    /// The value of the given field must equal the value provided by the
    /// referenced argument.
    Equals(String),
}

impl Condition {
    /// Parse a `Condition::Contains` from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// `Condition::Contains`.
    pub fn parse_contains(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "contains")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (value, input) = Query::parse_reference(&input)?;

        Ok((Self::Contains(value), input))
    }

    /// Parse a `Condition::Equals` from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// `Condition::Equals`.
    pub fn parse_equals(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "equals")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (value, input) = Query::parse_reference(&input)?;

        Ok((Self::Equals(value), input))
    }

    /// Parse a condition from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// condition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryCondition;
    ///
    /// let input = "contains: $foo";
    ///
    /// assert_eq!(
    ///     QueryCondition::parse(input),
    ///     Ok((QueryCondition::Contains("foo".to_string()), "".to_string()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QueryCondition;
    ///
    /// let input = "equals: $bar";
    ///
    /// assert_eq!(
    ///     QueryCondition::parse(input),
    ///     Ok((QueryCondition::Equals("bar".to_string()), "".to_string()))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(input, vec![Self::parse_contains, Self::parse_equals])
    }

    /// Return the name of the referenced argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryCondition;
    ///
    /// assert_eq!(
    ///     QueryCondition::Contains("foo".to_string()).reference(),
    ///     "foo"
    /// );
    ///
    /// assert_eq!(QueryCondition::Equals("bar".to_string()).reference(), "bar");
    /// ```
    #[must_use]
    pub fn reference(&self) -> &str {
        match self {
            Self::Contains(reference) | Self::Equals(reference) => reference,
        }
    }
}
