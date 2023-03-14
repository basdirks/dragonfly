pub use self::{
    condition::Condition,
    operator::Operator,
    path::Path,
};
use {
    parser::{
        brace_close,
        brace_open,
        camel_case,
        colon,
        dollar,
        literal,
        spaces,
        ParseError,
        ParseResult,
    },
    std::borrow::Cow,
};

/// A condition.
pub mod condition;
/// A condition operator.
pub mod operator;
/// A path to a field.
pub mod path;

/// A where clause.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Where<'a> {
    /// The name of the root node.
    pub name: Cow<'a, str>,
    /// The conditions that must be met.
    pub conditions: Vec<Condition<'a>>,
}

impl<'a> Where<'a> {
    /// Parse conditions from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not contain valid conditions.
    fn parse_conditions(input: &str) -> ParseResult<Vec<Condition<'a>>> {
        let mut input = input.to_owned();
        let mut path = Path::default();
        let mut conditions: Vec<Condition> = Vec::new();

        loop {
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
                        message: "A condition must refer to a field.".into(),
                    });
                }

                conditions.push(Condition {
                    path: path.clone(),
                    operator,
                    argument_name: argument.into(),
                });

                input = new_input;

                continue;
            }

            if !path.is_empty() {
                if let Ok((_, new_input)) = (|input: &str| {
                    let (_, input) = brace_close(input)?;
                    let (_, input) = spaces(&input)?;

                    Ok::<((), String), ParseError>(((), input))
                })(&input)
                {
                    input = new_input;

                    path.drop_back();

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

        Ok((
            Self {
                name: name.into(),
                conditions,
            },
            input,
        ))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            QueryCondition,
            QueryOperator,
            QueryPath,
        },
    };

    #[test]
    fn test_parse_where() {
        let input = "

where {
  foo {
    bar {
      contains: $foo
    }
  }
}

        "
        .trim();

        assert_eq!(
            Where::parse(input),
            Ok((
                Where {
                    name: "foo".into(),
                    conditions: vec![QueryCondition {
                        path: QueryPath::from_iter(["bar"]),
                        operator: QueryOperator::Contains,
                        argument_name: "foo".into(),
                    }]
                },
                String::new()
            ))
        );
    }

    #[test]
    fn test_parse_where_with_multiple_conditions() {
        let input = "

where {
  image {
    title {
      equals: $title
      tags {
        contains: $tag
      }
    }
  }
}

        "
        .trim();

        assert_eq!(
            Where::parse(input),
            Ok((
                Where {
                    name: "image".into(),
                    conditions: vec![
                        QueryCondition {
                            path: QueryPath::from_iter(["title"]),
                            operator: QueryOperator::Equals,
                            argument_name: "title".into(),
                        },
                        QueryCondition {
                            path: QueryPath::from_iter(["title", "tags"]),
                            operator: QueryOperator::Contains,
                            argument_name: "tag".into(),
                        }
                    ]
                },
                String::new()
            ))
        );
    }

    #[test]
    fn test_parse_where_with_missing_closing_brace() {
        let input = "

where {
  foo {
    bar {
      contains: $foo
    }

        "
        .trim();

        assert_eq!(
            Where::parse(input),
            Err(ParseError::Custom {
                message: "Expected closing brace for root node `foo`.".into(),
            })
        );
    }

    #[test]
    fn test_parse_where_with_stray_condition() {
        let input = "

where {
  foo {
    contains: $baz
  }
}

        "
        .trim();

        assert_eq!(
            Where::parse(input),
            Err(ParseError::Custom {
                message: "A condition must refer to a field.".into(),
            })
        );
    }

    #[test]
    fn test_parse_where_with_subsequent_conditions() {
        let input = "
    
where {
  foo {
    bar {
      contains: $baz
      contains: $bar
    }
    baz {
      equals: $baz
    }
  }
}
    
            "
        .trim();

        assert_eq!(
            Where::parse(input),
            Ok((
                Where {
                    name: "foo".into(),
                    conditions: vec![
                        QueryCondition {
                            path: QueryPath::from_iter(["bar"]),
                            operator: QueryOperator::Contains,
                            argument_name: "baz".into(),
                        },
                        QueryCondition {
                            path: QueryPath::from_iter(["bar"]),
                            operator: QueryOperator::Contains,
                            argument_name: "bar".into(),
                        },
                        QueryCondition {
                            path: QueryPath::from_iter(["baz"]),
                            operator: QueryOperator::Equals,
                            argument_name: "baz".into(),
                        }
                    ]
                },
                String::new()
            ))
        );
    }
}
