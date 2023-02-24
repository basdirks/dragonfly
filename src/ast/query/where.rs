use {
    super::{
        condition::{
            FieldPath,
            Operator,
        },
        Condition,
    },
    crate::parser::{
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
};

/// The conditions that queried data must meet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Where {
    /// The name of the root node.
    pub name: String,
    /// The conditions that must be met.
    pub conditions: Vec<Condition>,
}

impl Where {
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
    ///     FieldPath,
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "foo {
    ///     contains: $foo
    ///     bar {
    ///       equals: $bar
    ///     }
    ///     baz {
    ///       contains: $baz
    ///     }
    ///     qux {
    ///       contains: $qux
    ///     }
    ///   }
    /// }";
    ///
    /// let (conditions, input) = QueryWhere::parse_conditions(input).unwrap();
    ///
    /// assert_eq!(
    ///     conditions,
    ///     vec![
    ///         QueryCondition {
    ///             field_path: FieldPath::new(&["foo"]),
    ///             operator: QueryOperator::Contains,
    ///             argument: "foo".to_owned(),
    ///         },
    ///         QueryCondition {
    ///             field_path: FieldPath::new(&["foo", "bar",]),
    ///             operator: QueryOperator::Equals,
    ///             argument: "bar".to_owned(),
    ///         },
    ///         QueryCondition {
    ///             field_path: FieldPath::new(&["foo", "baz",]),
    ///             operator: QueryOperator::Contains,
    ///             argument: "baz".to_owned(),
    ///         },
    ///         QueryCondition {
    ///             field_path: FieldPath::new(&["foo", "qux",]),
    ///             operator: QueryOperator::Contains,
    ///             argument: "qux".to_owned(),
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
    ///     Err(ParseError::CustomError {
    ///         message: "a condition must refer to a field".to_owned(),
    ///         input: "contains: $foo".to_owned(),
    ///     })
    /// );
    /// ```
    pub fn parse_conditions(input: &str) -> ParseResult<Vec<Condition>> {
        let mut input = input.to_owned();
        let mut field_path = FieldPath::new(&[]);
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
                field_path.push(segment);

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
                if field_path.is_empty() {
                    return Err(ParseError::CustomError {
                        message: "a condition must refer to a field".to_owned(),
                        input: input.clone(),
                    });
                }

                conditions.push(Condition {
                    field_path: field_path.clone(),
                    operator,
                    argument,
                });

                input = new_input;

                continue;
            }

            // Parse `}`.
            if !field_path.is_empty() {
                if let Ok((_, new_input)) = (|input: &str| {
                    let (_, input) = brace_close(input)?;
                    let (_, input) = spaces(&input)?;

                    Ok::<((), String), ParseError>(((), input))
                })(&input)
                {
                    input = new_input;

                    let _ = field_path.pop_back();

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
    ///     FieldPath,
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "where {
    ///     foo {
    ///         bar {
    ///             contains: $foo
    ///         }
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse(input),
    ///     Ok((
    ///         QueryWhere {
    ///             name: "foo".to_owned(),
    ///             conditions: vec![QueryCondition {
    ///                 field_path: FieldPath::new(&["bar"]),
    ///                 operator: QueryOperator::Contains,
    ///                 argument: "foo".to_owned(),
    ///             }]
    ///         },
    ///         "".to_owned()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     FieldPath,
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "where {
    ///   image {
    ///     title {
    ///       equals: $title
    ///       tags {
    ///         contains: $tag
    ///       }
    ///     }
    ///   }
    /// }";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse(input),
    ///     Ok((
    ///         QueryWhere {
    ///             name: "image".to_owned(),
    ///             conditions: vec![
    ///                 QueryCondition {
    ///                     field_path: FieldPath::new(&["title"]),
    ///                     operator: QueryOperator::Equals,
    ///                     argument: "title".to_owned(),
    ///                 },
    ///                 QueryCondition {
    ///                     field_path: FieldPath::new(&["title", "tags",]),
    ///                     operator: QueryOperator::Contains,
    ///                     argument: "tag".to_owned(),
    ///                 }
    ///             ]
    ///         },
    ///         "".to_owned()
    ///     ))
    /// );
    /// ```
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
                    Err(ParseError::CustomError {
                        message: format!("expected closing brace for {name}."),
                        input: input.to_owned(),
                    })
                }
            }
        };

        let (_, input) = check_closing_brace(&input, "root node")?;
        let (_, input) = spaces(&input)?;
        let (_, input) = check_closing_brace(&input, "where clause")?;

        Ok((Self { name, conditions }, input))
    }
}
