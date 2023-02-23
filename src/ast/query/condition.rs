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

/// A path to a field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FieldPath(VecDeque<String>);

impl FieldPath {
    /// Create a new path from the given segments.
    ///
    /// # Arguments
    ///
    /// * `segments` - The segments of the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::FieldPath;
    ///
    /// let path = FieldPath::new(&["foo", "bar"]);
    ///
    /// assert_eq!(path.to_string(), "foo { bar }");
    /// ```
    #[must_use]
    pub fn new(segments: &[&str]) -> Self {
        Self(segments.iter().map(ToString::to_string).collect())
    }

    /// Pop the first segment off the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::FieldPath;
    ///
    /// let mut path = FieldPath::new(&["foo", "bar"]);
    ///
    /// assert_eq!(path.pop_front(), Some("foo".to_string()));
    /// assert_eq!(path.pop_front(), Some("bar".to_string()));
    /// assert_eq!(path.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<String> {
        self.0.pop_front()
    }

    /// Pop the last segment off the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::FieldPath;
    ///
    /// let mut path = FieldPath::new(&["foo", "bar"]);
    ///
    /// assert_eq!(path.pop_back(), Some("bar".to_string()));
    /// assert_eq!(path.pop_back(), Some("foo".to_string()));
    /// assert_eq!(path.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<String> {
        self.0.pop_back()
    }

    /// Push a segment onto the path.
    ///
    /// # Arguments
    ///
    /// * `segment` - The segment to push onto the path.
    pub fn push(
        &mut self,
        segment: String,
    ) {
        self.0.push_back(segment);
    }

    /// Check if the path is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::FieldPath;
    ///
    /// assert!(!FieldPath::new(&["foo", "bar"]).is_empty());
    /// assert!(FieldPath::new(&[]).is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for FieldPath {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self(fields) = self;
        let path_length = fields.len().saturating_sub(1);

        for (index, field) in fields.iter().enumerate() {
            if index > 0 {
                write!(f, " {{ {field}")?;
            } else {
                write!(f, "{field}")?;
            }
        }

        for _ in 0..path_length {
            write!(f, " }}")?;
        }

        Ok(())
    }
}

/// A condition that must be met for a query to return a result.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Condition {
    /// The path to the field that must meet the condition.
    pub field_path: FieldPath,
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

        write!(f, "{field_path} {operator} ${argument}")
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

    #[test]
    fn test_display_condition() {
        assert_eq!(
            Condition {
                field_path: FieldPath::new(&["foo", "bar"]),
                operator: Operator::Contains,
                argument: "baz".to_string(),
            }
            .to_string(),
            "foo { bar } contains $baz"
        );

        assert_eq!(
            Condition {
                field_path: FieldPath::new(&["foo", "bar", "baz",]),
                operator: Operator::Equals,
                argument: "baz".to_string(),
            }
            .to_string(),
            "foo { bar { baz } } equals $baz"
        );
    }
}
