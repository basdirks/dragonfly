use {
    super::{
        r#type::{
            Scalar,
            Type,
        },
        TypeError,
    },
    crate::parser::{
        alphabetics,
        brace_close,
        brace_open,
        colon,
        comma,
        dollar,
        literal,
        option,
        paren_close,
        paren_open,
        spaces,
        ParseResult,
    },
    std::collections::HashSet,
};
pub use {
    argument::Argument,
    r#where::Where,
    return_type::ReturnType,
    schema::Schema,
};

/// Query arguments.
pub mod argument;
/// The return type of a query.
pub mod return_type;
/// The structure of the data that the query should return.
pub mod schema;
/// Sets of conditions that queried data must meet.
pub mod r#where;

/// A query.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Query {
    /// The name of the query. Used as the name of the generated function.
    pub name: String,
    /// The arguments of the query.
    pub arguments: Vec<Argument>,
    /// The schema of the query.
    pub schema: Schema,
    /// The return type of the query. Must be a model reference or an array of
    /// a model reference.
    pub r#type: ReturnType,
    /// The where clause of the query.
    pub r#where: Option<Where>,
}

impl Query {
    /// Parse query arguments from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid query
    /// argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Query::parse_arguments("($id: Int)"),
    ///     Ok((
    ///         vec![QueryArgument {
    ///             name: "id".to_owned(),
    ///             r#type: Type::Scalar(Scalar::Int),
    ///         }],
    ///         "".to_owned()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Query::parse_arguments("($id: Int, $name: [String])"),
    ///     Ok((
    ///         vec![
    ///             QueryArgument {
    ///                 name: "id".to_owned(),
    ///                 r#type: Type::Scalar(Scalar::Int),
    ///             },
    ///             QueryArgument {
    ///                 name: "name".to_owned(),
    ///                 r#type: Type::Array(Scalar::String)
    ///             }
    ///         ],
    ///         "".to_owned()
    ///     ))
    /// );
    /// ```
    pub fn parse_arguments(input: &str) -> ParseResult<Vec<Argument>> {
        if let Ok((_, input)) = paren_open(input) {
            let (argument, mut input) = Argument::parse(&input)?;
            let mut arguments = vec![argument];

            while let Ok((_, new_input)) = comma(&input) {
                let (_, new_input) = spaces(&new_input)?;
                let (argument, new_input) = Argument::parse(&new_input)?;

                arguments.push(argument);
                input = new_input;
            }

            let (_, input) = paren_close(&input)?;

            return Ok((arguments, input));
        }

        Ok((vec![], input.to_owned()))
    }

    /// Parse a reference from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid
    /// reference.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// assert_eq!(
    ///     Query::parse_reference("$name"),
    ///     Ok(("name".to_owned(), "".to_owned()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// assert!(Query::parse_reference("name").is_err());
    /// ```
    pub fn parse_reference(input: &str) -> ParseResult<String> {
        let (_, input) = dollar(input)?;

        alphabetics(&input)
    }

    /// Parse a query from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid query.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryReturnType,
    ///     QuerySchema,
    ///     QuerySchemaNode,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "
    ///
    /// query images: [Image] {
    ///   image {
    ///     title
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Query {
    ///     name: "images".to_owned(),
    ///     arguments: vec![],
    ///     schema: QuerySchema {
    ///         name: "image".to_owned(),
    ///         nodes: vec![QuerySchemaNode::Field("title".to_owned())],
    ///     },
    ///     r#type: QueryReturnType::Array("Image".to_owned()),
    ///     r#where: None,
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, "".to_owned())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     FieldPath,
    ///     Query,
    ///     QueryArgument,
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryReturnType,
    ///     QuerySchema,
    ///     QuerySchemaNode,
    ///     QueryWhere,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "
    ///
    /// query images($tag: String, $title: String): [Image] {
    ///   image {
    ///     title
    ///   }
    ///   where {
    ///     image {
    ///       title {
    ///         equals: $title
    ///         tags {
    ///           contains: $tag
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Query {
    ///     name: "images".to_owned(),
    ///     arguments: vec![
    ///         QueryArgument {
    ///             name: "tag".to_owned(),
    ///             r#type: Type::Scalar(Scalar::String),
    ///         },
    ///         QueryArgument {
    ///             name: "title".to_owned(),
    ///             r#type: Type::Scalar(Scalar::String),
    ///         },
    ///     ],
    ///     schema: QuerySchema {
    ///         name: "image".to_owned(),
    ///         nodes: vec![QuerySchemaNode::Field("title".to_owned())],
    ///     },
    ///     r#type: QueryReturnType::Array("Image".to_owned()),
    ///     r#where: Some(QueryWhere {
    ///         name: "image".to_owned(),
    ///         conditions: vec![
    ///             QueryCondition {
    ///                 path: FieldPath::new(&["title"]),
    ///                 operator: QueryOperator::Equals,
    ///                 argument: "title".to_owned(),
    ///             },
    ///             QueryCondition {
    ///                 path: FieldPath::new(&["title", "tags"]),
    ///                 operator: QueryOperator::Contains,
    ///                 argument: "tag".to_owned(),
    ///             },
    ///         ],
    ///     }),
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, "".to_owned())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     FieldPath,
    ///     Query,
    ///     QueryArgument,
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryReturnType,
    ///     QuerySchema,
    ///     QuerySchemaNode,
    ///     QueryWhere,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "
    ///
    /// query imagesByCountryName($name: CountryName): [Image] {
    ///   image {
    ///     title
    ///     category
    ///   }
    ///   where {
    ///     image {
    ///       country {
    ///         name {
    ///           equals: $name
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Query {
    ///     name: "imagesByCountryName".to_owned(),
    ///     arguments: vec![QueryArgument {
    ///         name: "name".to_owned(),
    ///         r#type: Type::Scalar(Scalar::Reference("CountryName".to_owned())),
    ///     }],
    ///     schema: QuerySchema {
    ///         name: "image".to_owned(),
    ///         nodes: vec![
    ///             QuerySchemaNode::Field("title".to_owned()),
    ///             QuerySchemaNode::Field("category".to_owned()),
    ///         ],
    ///     },
    ///     r#type: QueryReturnType::Array("Image".to_owned()),
    ///     r#where: Some(QueryWhere {
    ///         name: "image".to_owned(),
    ///         conditions: vec![QueryCondition {
    ///             path: FieldPath::new(&["country", "name"]),
    ///             operator: QueryOperator::Equals,
    ///             argument: "name".to_owned(),
    ///         }],
    ///     }),
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, "".to_owned())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "query")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = alphabetics(&input)?;
        let (_, input) = spaces(&input)?;
        let (arguments, input) = Self::parse_arguments(&input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = ReturnType::parse(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (schema, input) = Schema::parse(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#where, input) = option(&input, Where::parse)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((
            Self {
                name,
                arguments,
                schema,
                r#type,
                r#where,
            },
            input,
        ))
    }

    /// Check whether the schema is empty.
    ///
    /// # Errors
    ///
    /// Returns `TypeError::EmptyQuerySchema` if the schema is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// let input = "
    ///
    /// query images: [Image] {
    ///     image {
    ///         title
    ///     }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let query = Query::parse(input).unwrap().0;
    ///
    /// assert!(query.check_empty_schema().is_ok());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryReturnType,
    ///     QuerySchema,
    ///     TypeError,
    /// };
    ///
    /// let query = Query {
    ///     name: "images".to_owned(),
    ///     arguments: vec![],
    ///     schema: QuerySchema {
    ///         name: "image".to_owned(),
    ///         nodes: vec![],
    ///     },
    ///     r#type: QueryReturnType::Array("Image".to_owned()),
    ///     r#where: None,
    /// };
    ///
    /// assert_eq!(
    ///     query.check_empty_schema(),
    ///     Err(TypeError::EmptyQuerySchema {
    ///         query_name: "images".to_owned(),
    ///     })
    /// );
    /// ```
    pub fn check_empty_schema(&self) -> Result<(), TypeError> {
        if self.schema.is_empty() {
            Err(TypeError::EmptyQuerySchema {
                query_name: self.name.clone(),
            })
        } else {
            Ok(())
        }
    }

    /// Check whether all arguments are used in the where clause.
    ///
    /// # Errors
    ///
    /// Returns `TypeError::UnusedQueryArgument` if any argument is not used
    /// in the where clause.
    ///
    /// # Panics
    ///
    /// Panics only if there is a bug in `alloc::vec::Vec::is_empty()` or
    /// `core::slice::first()`.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// let input = "
    ///
    /// query images($name: CountryName): [Image] {
    ///   image {
    ///     title
    ///   }
    ///   where {
    ///     image {
    ///       country {
    ///         name {
    ///           equals: $name
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert!(Query::parse(input)
    ///     .unwrap()
    ///     .0
    ///     .check_unused_arguments()
    ///     .is_ok());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    ///     TypeError,
    /// };
    ///
    /// let input = "
    ///
    /// query images($name: CountryName): [Image] {
    ///     image {
    ///         title
    ///     }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Query::parse(input).unwrap().0.check_unused_arguments(),
    ///     Err(TypeError::UnusedQueryArgument {
    ///         query_name: "images".to_owned(),
    ///         argument: QueryArgument {
    ///             name: "name".to_owned(),
    ///             r#type: Type::Scalar(Scalar::Reference(
    ///                 "CountryName".to_owned()
    ///             )),
    ///         },
    ///     }),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    ///     TypeError,
    /// };
    ///
    /// let input = "
    ///
    /// query images($name: CountryName, $tag: String): [Image] {
    ///   image {
    ///     title
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Query::parse(input).unwrap().0.check_unused_arguments(),
    ///     Err(TypeError::UnusedQueryArgument {
    ///         query_name: "images".to_owned(),
    ///         argument: QueryArgument {
    ///             name: "name".to_owned(),
    ///             r#type: Type::Scalar(Scalar::Reference(
    ///                 "CountryName".to_owned()
    ///             )),
    ///         },
    ///     }),
    /// );
    /// ```
    ///
    /// This check always passes if there are no arguments.
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// let input = "
    ///
    /// query images: [Image] {
    ///   image {
    ///     title
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert!(Query::parse(input)
    ///     .unwrap()
    ///     .0
    ///     .check_unused_arguments()
    ///     .is_ok(),);
    /// ```
    pub fn check_unused_arguments(&self) -> Result<(), TypeError> {
        if self.arguments.is_empty() {
            return Ok(());
        }

        match &self.r#where {
            Some(r#where) => {
                let used_arguments = r#where
                    .conditions
                    .iter()
                    .map(|condition| condition.argument_name.clone())
                    .collect::<HashSet<_>>();

                for argument in &self.arguments {
                    if !used_arguments.contains(&argument.name) {
                        return Err(TypeError::UnusedQueryArgument {
                            query_name: self.name.clone(),
                            argument: argument.clone(),
                        });
                    }
                }
            }
            None => {
                return Err(TypeError::UnusedQueryArgument {
                    query_name: self.name.clone(),
                    argument: self.arguments.first().map_or_else(
                        || unreachable!(),
                        std::clone::Clone::clone,
                    ),
                });
            }
        }

        Ok(())
    }
}
