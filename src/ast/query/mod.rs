use {
    super::{
        r#type::Type,
        Enum,
        Scalar,
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
        maybe,
        paren_close,
        paren_open,
        spaces,
        ParseResult,
    },
    std::collections::{
        HashMap,
        HashSet,
    },
};
pub use {
    argument::Argument,
    condition::Condition,
    r#where::Where,
    schema::Schema,
};

/// Query arguments.
pub mod argument;
/// Conditions that queried data must meet.
pub mod condition;
/// The structure of the data that the query should return.
pub mod schema;
/// Sets of conditions that queried data must meet.
pub mod r#where;

/// A query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query {
    /// The name of the query. Used as the name of the generated function.
    pub name: String,
    /// The arguments of the query.
    pub arguments: Vec<Argument>,
    /// The schema of the query.
    pub schema: Schema,
    /// The return type of the query.
    pub r#type: Type,
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
    /// Returns a `ParseError` if the input does not start with a valid query
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
    ///     Query::parse_arguments("($id: UUID)"),
    ///     Ok((
    ///         vec![QueryArgument {
    ///             name: "id".to_string(),
    ///             r#type: Type::Scalar(Scalar::Reference("UUID".to_string()))
    ///         }],
    ///         "".to_string()
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
    ///     Query::parse_arguments("($id: UUID, $name: [String])"),
    ///     Ok((
    ///         vec![
    ///             QueryArgument {
    ///                 name: "id".to_string(),
    ///                 r#type: Type::Scalar(Scalar::Reference("UUID".to_string()))
    ///             },
    ///             QueryArgument {
    ///                 name: "name".to_string(),
    ///                 r#type: Type::Array(Scalar::String)
    ///             }
    ///         ],
    ///         "".to_string()
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

        Ok((vec![], input.to_string()))
    }

    /// Parse a reference from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// reference.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// assert_eq!(
    ///     Query::parse_reference("$name"),
    ///     Ok(("name".to_string(), "".to_string()))
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
    /// Returns a `ParseError` if the input does not start with a valid query.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QuerySchema,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "query images: [Image] {
    ///   image {
    ///     title
    ///   }
    /// }";
    ///
    /// let expected = Query {
    ///     name: "images".to_string(),
    ///     arguments: vec![],
    ///     schema: QuerySchema::Model {
    ///         name: "image".to_string(),
    ///         nodes: vec![QuerySchema::Field("title".to_string())],
    ///     },
    ///     r#type: Type::Array(Scalar::Reference("Image".to_string())),
    ///     r#where: None,
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, "".to_string())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     QueryCondition,
    ///     QuerySchema,
    ///     QueryWhere,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "query images($tag: String, $title: String): [Image] {
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
    /// }";
    ///
    /// let expected = Query {
    ///     name: "images".to_string(),
    ///     arguments: vec![
    ///         QueryArgument {
    ///             name: "tag".to_string(),
    ///             r#type: Type::Scalar(Scalar::String),
    ///         },
    ///         QueryArgument {
    ///             name: "title".to_string(),
    ///             r#type: Type::Scalar(Scalar::String),
    ///         },
    ///     ],
    ///     schema: QuerySchema::Model {
    ///         name: "image".to_string(),
    ///         nodes: vec![QuerySchema::Field("title".to_string())],
    ///     },
    ///     r#type: Type::Array(Scalar::Reference("Image".to_string())),
    ///     r#where: Some(QueryWhere::Node {
    ///         name: "image".to_string(),
    ///         nodes: vec![QueryWhere::Node {
    ///             name: "title".to_string(),
    ///             nodes: vec![
    ///                 QueryWhere::Condition(QueryCondition::Equals(
    ///                     "title".to_string(),
    ///                 )),
    ///                 QueryWhere::Node {
    ///                     name: "tags".to_string(),
    ///                     nodes: vec![QueryWhere::Condition(
    ///                         QueryCondition::Contains("tag".to_string()),
    ///                     )],
    ///                 },
    ///             ],
    ///         }],
    ///     }),
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, "".to_string())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     QueryCondition,
    ///     QuerySchema,
    ///     QueryWhere,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "query imagesByCountryName($name: CountryName): [Image] {
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
    /// }";
    ///
    /// let expected = Query {
    ///     name: "imagesByCountryName".to_string(),
    ///     arguments: vec![QueryArgument {
    ///         name: "name".to_string(),
    ///         r#type: Type::Scalar(Scalar::Reference("CountryName".to_string())),
    ///     }],
    ///     schema: QuerySchema::Model {
    ///         name: "image".to_string(),
    ///         nodes: vec![
    ///             QuerySchema::Field("title".to_string()),
    ///             QuerySchema::Field("category".to_string()),
    ///         ],
    ///     },
    ///     r#type: Type::Array(Scalar::Reference("Image".to_string())),
    ///     r#where: Some(QueryWhere::Node {
    ///         name: "image".to_string(),
    ///         nodes: vec![QueryWhere::Node {
    ///             name: "country".to_string(),
    ///             nodes: vec![QueryWhere::Node {
    ///                 name: "name".to_string(),
    ///                 nodes: vec![QueryWhere::Condition(QueryCondition::Equals(
    ///                     "name".to_string(),
    ///                 ))],
    ///             }],
    ///         }],
    ///     }),
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "query")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = alphabetics(&input)?;
        let (_, input) = spaces(&input)?;
        let (arguments, input) = Self::parse_arguments(&input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (schema, input) = Schema::parse(&input)?;

        let (r#where, input) = maybe(&input, |input| {
            let (_, input) = spaces(input)?;
            Where::parse(&input)
        })?;

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

    /// Check whether the root node of the schema has the same name as the root
    /// node of the where clause.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError::IncompatibleQueryRootNodes` if the names of the
    /// root nodes of the schema and the where clause are not the same.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// let input = "query images: [Image] {
    ///     image {
    ///         title
    ///     }
    ///     where {
    ///         image {
    ///             title {
    ///                 equals: $title
    ///             }
    ///         }
    ///     }
    /// }";
    ///
    /// assert!(Query::parse(input).unwrap().0.check_root_nodes().is_ok());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     TypeError,
    /// };
    ///
    /// let input = "query images: [Image] {
    ///     image {
    ///         title
    ///     }
    ///     where {
    ///         images {
    ///             title {
    ///                 equals: $title
    ///             }
    ///         }
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     Query::parse(input).unwrap().0.check_root_nodes(),
    ///     Err(TypeError::IncompatibleQueryRootNodes {
    ///         query_name: "images".to_string(),
    ///         schema_root: "image".to_string(),
    ///         where_root: "images".to_string(),
    ///     })
    /// );
    /// ```
    pub fn check_root_nodes(&self) -> Result<(), TypeError> {
        if let Some(Where::Node {
            name: where_root, ..
        }) = &self.r#where
        {
            if let Schema::Model {
                name: schema_root, ..
            } = &self.schema
            {
                if where_root != schema_root {
                    return Err(TypeError::IncompatibleQueryRootNodes {
                        query_name: self.name.clone(),
                        schema_root: schema_root.clone(),
                        where_root: where_root.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Check whether the schema is empty.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError::EmptyQuerySchema` if the schema is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// let input = "query images: [Image] {
    ///     image {
    ///         title
    ///     }
    /// }";
    ///
    /// assert!(Query::parse(input).is_ok());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// let input = "query images: [Image] {}";
    ///
    /// assert!(Query::parse(input).is_err());
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
    /// Returns a `TypeError::UnusedQueryArgument` if any argument is not used
    /// in the where clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// let input = "query images($name: CountryName): [Image] {
    ///     image {
    ///         title
    ///     }
    ///     where {
    ///         image {
    ///             country {
    ///                 name {
    ///                     equals: $name
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }";
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
    /// let input = "query images($name: CountryName): [Image] {
    ///     image {
    ///         title
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     Query::parse(input).unwrap().0.check_unused_arguments(),
    ///     Err(TypeError::UnusedQueryArgument {
    ///         query_name: "images".to_string(),
    ///         argument: QueryArgument {
    ///             name: "name".to_string(),
    ///             r#type: Type::Scalar(Scalar::Reference(
    ///                 "CountryName".to_string()
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
    /// let input = "query images($name: CountryName, $tag: String): [Image] {
    ///     image {
    ///         title
    ///     }
    ///     where {
    ///         image {
    ///             country {
    ///                 name {
    ///                     equals: $name
    ///                 }
    ///             }
    ///         }
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     Query::parse(input).unwrap().0.check_unused_arguments(),
    ///     Err(TypeError::UnusedQueryArgument {
    ///         query_name: "images".to_string(),
    ///         argument: QueryArgument {
    ///             name: "tag".to_string(),
    ///             r#type: Type::Scalar(Scalar::String),
    ///         },
    ///     }),
    /// );
    /// ```
    pub fn check_unused_arguments(&self) -> Result<(), TypeError> {
        let condition_references = self
            .r#where
            .as_ref()
            .map(Where::references)
            .unwrap_or_default();

        for argument in &self.arguments {
            if !condition_references.contains(&argument.name) {
                return Err(TypeError::UnusedQueryArgument {
                    query_name: self.name.clone(),
                    argument: argument.clone(),
                });
            }
        }

        Ok(())
    }

    /// Check whether each condition references an existing argument.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError::UnknownQueryConditionReference` if any condition
    /// references an argument that does not exist.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// let input = "query images($name: CountryName): [Image] {
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
    /// }";
    ///
    /// assert!(Query::parse(input)
    ///     .unwrap()
    ///     .0
    ///     .check_condition_references()
    ///     .is_ok());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryCondition,
    ///     TypeError,
    /// };
    ///
    /// let input = "query images($name: CountryName): [Image] {
    ///   image {
    ///     title
    ///   }
    ///   where {
    ///     image {
    ///       country {
    ///         name {
    ///           equals: $tag
    ///         }
    ///       }
    ///     }
    ///   }
    /// }";
    ///
    /// assert_eq!(
    ///     Query::parse(input).unwrap().0.check_condition_references(),
    ///     Err(TypeError::UnknownQueryConditionReference {
    ///         query_name: "images".to_string(),
    ///         condition: QueryCondition::Equals("tag".to_string()),
    ///     }),
    /// );
    /// ```
    pub fn check_condition_references(&self) -> Result<(), TypeError> {
        if let Some(r#where) = &self.r#where {
            let argument_names = self
                .arguments
                .iter()
                .map(|argument| argument.name.clone())
                .collect::<HashSet<String>>();

            for condition in &r#where.conditions() {
                if !argument_names.contains(condition.reference()) {
                    return Err(TypeError::UnknownQueryConditionReference {
                        query_name: self.name.clone(),
                        condition: condition.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Check whether each query arguments is a primitive type or a reference
    /// to an existing enum.
    ///
    /// # Arguments
    ///
    /// * `enums` - A list of enums.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError::InvalidQueryArgumentType` if any argument type is
    /// not a primitive type or a reference to an existing enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Enum,
    ///     Query,
    /// };
    ///
    /// let input = "query images($name: CountryName): [Image] {
    ///   image {
    ///     title
    ///   }
    /// }";
    ///
    /// let query = Query::parse(input).unwrap().0;
    ///
    /// let enums = vec![(
    ///     "CountryName".to_string(),
    ///     Enum {
    ///         name: "CountryName".to_string(),
    ///         variants: vec!["France".to_string(), "Germany".to_string()]
    ///             .into_iter()
    ///             .collect(),
    ///     },
    /// )]
    /// .into_iter()
    /// .collect();
    ///
    /// assert!(query.check_argument_types(&enums).is_ok());
    /// ```
    ///
    /// ```rust
    /// use {
    ///     dragonfly::ast::Query,
    ///     std::collections::HashMap,
    /// };
    ///
    /// let input = "query foo($p: Boolean, $date: DateTime, $rate: Float, \
    ///              $population: Int, $name: String): [Image] {
    ///   image {
    ///     title
    ///   }
    /// }";
    ///
    /// let query = Query::parse(input).unwrap().0;
    ///
    /// assert!(query.check_argument_types(&HashMap::new()).is_ok());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Enum,
    ///     Query,
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    ///     TypeError,
    /// };
    ///
    /// let input = "query images($name: CountryName): [Image] {
    ///   image {
    ///     title
    ///   }
    /// }";
    ///
    /// let query = Query::parse(input).unwrap().0;
    ///
    /// let enums = vec![(
    ///     "ContinentName".to_string(),
    ///     Enum {
    ///         name: "ContinentName".to_string(),
    ///         variants: vec!["Europe".to_string(), "Asia".to_string()]
    ///             .into_iter()
    ///             .collect(),
    ///     },
    /// )]
    /// .into_iter()
    /// .collect();
    ///
    /// assert_eq!(
    ///     query.check_argument_types(&enums),
    ///     Err(TypeError::InvalidQueryArgumentType {
    ///         query_name: "images".to_string(),
    ///         argument: QueryArgument {
    ///             name: "name".to_string(),
    ///             r#type: Type::Scalar(Scalar::Reference(
    ///                 "CountryName".to_string()
    ///             )),
    ///         },
    ///     }),
    /// );
    /// ```
    pub fn check_argument_types(
        &self,
        enums: &HashMap<String, Enum>,
    ) -> Result<(), TypeError> {
        for argument in &self.arguments {
            if let Scalar::Reference(name) = &argument.scalar() {
                if !enums.contains_key(name) {
                    return Err(TypeError::InvalidQueryArgumentType {
                        query_name: self.name.clone(),
                        argument: argument.clone(),
                    });
                }
            }
        }

        Ok(())
    }

    /// Check that the return type of the query is a reference to an existing
    /// model, or an array of such a type.
    ///
    /// # Arguments
    ///
    /// * `models` - An list of models.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError::InvalidQueryReturnType` if the return type is
    /// not a reference to an existing model, or an array of such a type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Model,
    ///     Query,
    /// };
    ///
    /// let input = "query images: [Image] {
    ///   image {
    ///     title
    ///   }
    /// }";
    ///
    /// let query = Query::parse(input).unwrap().0;
    /// let models = vec!["Image".to_string()].into_iter().collect();
    ///
    /// assert!(query.check_return_type(&models).is_ok());
    /// ```
    ///
    /// ```rust
    /// use {
    ///     dragonfly::ast::{
    ///         Query,
    ///         Scalar,
    ///         Type,
    ///         TypeError,
    ///     },
    ///     std::collections::HashSet,
    /// };
    ///
    /// let input = "query images: [Image] {
    ///   image {
    ///     title
    ///   }
    /// }";
    ///
    /// let query = Query::parse(input).unwrap().0;
    ///
    /// assert_eq!(
    ///     query.check_return_type(&HashSet::new()),
    ///     Err(TypeError::InvalidQueryReturnType {
    ///         query_name: "images".to_string(),
    ///         r#type: Type::Array(Scalar::Reference("Image".to_string())),
    ///     })
    /// );
    /// ```
    pub fn check_return_type(
        &self,
        model_names: &HashSet<String>,
    ) -> Result<(), TypeError> {
        if let Scalar::Reference(name) = &self.r#type.scalar() {
            if !model_names.contains(name) {
                return Err(TypeError::InvalidQueryReturnType {
                    query_name: self.name.clone(),
                    r#type: self.r#type.clone(),
                });
            }
        }

        Ok(())
    }
}
