pub use self::{
    condition::Condition,
    operator::Operator,
    path::Path,
};
use crate::parser::{
    brace_close,
    brace_open,
    camel_case,
    colon,
    dollar,
    literal,
    spaces,
    ParseError,
    ParseResult,
};

/// A condition.
pub mod condition;
/// A condition operator.
pub mod operator;
/// A path to a field.
pub mod path;

/// A where clause.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Where {
    /// The name of the root node.
    pub name: String,
    /// The conditions that must be met.
    pub conditions: Vec<Condition>,
}

impl Where {
    /// Create a new where clause.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the root node.
    /// * `conditions` - The conditions that must be met.
    #[must_use]
    pub fn new(
        name: &str,
        conditions: &[Condition],
    ) -> Self {
        Self {
            name: name.to_owned(),
            conditions: conditions.to_owned(),
        }
    }

    /// Parse conditions from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not contain valid conditions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryPath,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "
    ///
    /// foo {
    ///   contains: $foo
    ///   bar {
    ///     equals: $bar
    ///   }
    ///   baz {
    ///     contains: $baz
    ///   }
    ///   qux {
    ///     contains: $qux
    ///   }
    /// }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let (conditions, input) = QueryWhere::parse_conditions(input).unwrap();
    ///
    /// assert_eq!(
    ///     conditions,
    ///     vec![
    ///         QueryCondition {
    ///             path: QueryPath::new(&["foo"]),
    ///             operator: QueryOperator::Contains,
    ///             argument_name: "foo".to_owned(),
    ///         },
    ///         QueryCondition {
    ///             path: QueryPath::new(&["foo", "bar"]),
    ///             operator: QueryOperator::Equals,
    ///             argument_name: "bar".to_owned(),
    ///         },
    ///         QueryCondition {
    ///             path: QueryPath::new(&["foo", "baz"]),
    ///             operator: QueryOperator::Contains,
    ///             argument_name: "baz".to_owned(),
    ///         },
    ///         QueryCondition {
    ///             path: QueryPath::new(&["foo", "qux"]),
    ///             operator: QueryOperator::Contains,
    ///             argument_name: "qux".to_owned(),
    ///         }
    ///     ]
    /// );
    ///
    /// assert_eq!(input, "}");
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::QueryWhere,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "contains: $foo";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse_conditions(input),
    ///     Err(ParseError::Custom {
    ///         message: "A condition must refer to a field.".to_owned(),
    ///     })
    /// );
    /// ```
    pub fn parse_conditions(input: &str) -> ParseResult<Vec<Condition>> {
        let mut input = input.to_owned();
        let mut path = Path::new(&[]);
        let mut conditions: Vec<Condition> = vec![];

        loop {
            // Parse `segment {`.
            if let Ok((segment, new_input)) = (|input: &str| {
                let (segment, input) = camel_case(input)?;
                let (_, input) = spaces(&input)?;
                let (_, input) = brace_open(&input)?;
                let (_, input) = spaces(&input)?;

                Ok::<(String, String), ParseError>((segment, input))
            })(&input)
            {
                path.push(segment);

                input = new_input;

                continue;
            }

            // Parse `condition_type: $argument`.
            if let Ok((operator, argument, new_input)) = (|input: &str| {
                let (operator, input) = Operator::parse(input)?;
                let (_, input) = spaces(&input)?;
                let (_, input) = colon(&input)?;
                let (_, input) = spaces(&input)?;
                let (_, input) = dollar(&input)?;
                let (argument, input) = camel_case(&input)?;
                let (_, input) = spaces(&input)?;

                Ok::<(Operator, String, String), ParseError>((
                    operator, argument, input,
                ))
            })(&input)
            {
                if path.is_empty() {
                    return Err(ParseError::Custom {
                        message: "A condition must refer to a field."
                            .to_owned(),
                    });
                }

                conditions.push(Condition {
                    path: path.clone(),
                    operator,
                    argument_name: argument,
                });

                input = new_input;

                continue;
            }

            // Parse `}`.
            if !path.is_empty() {
                if let Ok((_, new_input)) = (|input: &str| {
                    let (_, input) = brace_close(input)?;
                    let (_, input) = spaces(&input)?;

                    Ok::<((), String), ParseError>(((), input))
                })(&input)
                {
                    input = new_input;

                    let _ = path.pop_back();

                    continue;
                }
            }

            return Ok((conditions, input));
        }
    }

    /// Parse a where clause from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid where
    /// clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryPath,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "
    ///
    /// where {
    ///     foo {
    ///         bar {
    ///             contains: $foo
    ///         }
    ///     }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     QueryWhere::parse(input),
    ///     Ok((
    ///         QueryWhere {
    ///             name: "foo".to_owned(),
    ///             conditions: vec![QueryCondition {
    ///                 path: QueryPath::new(&["bar"]),
    ///                 operator: QueryOperator::Contains,
    ///                 argument_name: "foo".to_owned(),
    ///             }]
    ///         },
    ///         String::new()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryPath,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "
    ///
    /// where {
    ///   image {
    ///     title {
    ///       equals: $title
    ///       tags {
    ///         contains: $tag
    ///       }
    ///     }
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     QueryWhere::parse(input),
    ///     Ok((
    ///         QueryWhere {
    ///             name: "image".to_owned(),
    ///             conditions: vec![
    ///                 QueryCondition {
    ///                     path: QueryPath::new(&["title"]),
    ///                     operator: QueryOperator::Equals,
    ///                     argument_name: "title".to_owned(),
    ///                 },
    ///                 QueryCondition {
    ///                     path: QueryPath::new(&["title", "tags"]),
    ///                     operator: QueryOperator::Contains,
    ///                     argument_name: "tag".to_owned(),
    ///                 }
    ///             ]
    ///         },
    ///         String::new()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::QueryWhere,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// where {
    ///    foo {
    ///       bar {
    ///         contains: $foo
    ///       }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     QueryWhere::parse(input),
    ///     Err(ParseError::Custom {
    ///         message: "Expected closing brace for root node `foo`.".to_owned(),
    ///     })
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///    ast::QueryWhere,
    ///   parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// where {
    ///   foo {
    ///     bar {
    ///       contains: $foo
    ///     }
    ///   }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///    QueryWhere::parse(input),
    ///   Err(ParseError::Custom {
    ///     message: "Expected closing brace for where clause.".to_owned(),
    ///  })
    /// );
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "where")?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (name, input) = camel_case(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (conditions, input) = Self::parse_conditions(&input)?;
        let (_, input) = spaces(&input)?;

        let check_closing_brace = |input: &str, name: &str| {
            match brace_close(input) {
                Ok((_, input)) => Ok(((), input)),
                _ => {
                    Err(ParseError::Custom {
                        message: format!("Expected closing brace for {name}."),
                    })
                }
            }
        };

        let (_, input) =
            check_closing_brace(&input, &format!("root node `{name}`"))?;
        let (_, input) = spaces(&input)?;
        let (_, input) = check_closing_brace(&input, "where clause")?;

        Ok((Self { name, conditions }, input))
    }
}