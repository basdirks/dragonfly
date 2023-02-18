use {
    super::{
        r#type::Type,
        TypeError,
    },
    crate::parser::{
        alphabetics,
        brace_close,
        brace_open,
        choice,
        colon,
        comma,
        dollar,
        literal,
        many1,
        map,
        maybe,
        paren_close,
        paren_open,
        spaces,
        ParseResult,
    },
    std::collections::HashSet,
};

/// A condition that must be met for a query to return a result.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Condition {
    /// The value of the given field must contain the given value.
    Contains(String),
    /// The value of the given field must equal the given value.
    Equals(String),
}

impl Condition {
    fn parse_contains(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "contains")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (value, input) = Query::parse_reference(&input)?;

        Ok((Self::Contains(value), input))
    }

    fn parse_equals(input: &str) -> ParseResult<Self> {
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
}

/// The conditions that queried data must meet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Where {
    /// A condition that must be met for a query to return a result.
    Condition(Condition),
    /// A field.
    Node {
        /// The name of the field.
        name: String,
        /// In the case of a relation, the fields and their conditions. This is
        /// empty if the field does not refer to another model.
        nodes: Vec<Where>,
    },
}

impl Where {
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
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "contains: $foo";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse_condition(input),
    ///     Ok((
    ///         QueryWhere::Condition(QueryCondition::Contains("foo".to_string())),
    ///         "".to_string(),
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "equals: $bar";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse_condition(input),
    ///     Ok((
    ///         QueryWhere::Condition(QueryCondition::Equals("bar".to_string())),
    ///         "".to_string(),
    ///     ))
    /// );
    /// ```
    pub fn parse_condition(input: &str) -> ParseResult<Self> {
        let (condition, input) = Condition::parse(input)?;

        Ok((Self::Condition(condition), input))
    }

    /// Parse a node from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "foo {
    ///     contains: $foo
    /// }";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse_node(input),
    ///     Ok((
    ///         QueryWhere::Node {
    ///             name: "foo".to_string(),
    ///             nodes: vec![QueryWhere::Condition(QueryCondition::Contains(
    ///                 "foo".to_string()
    ///             ))]
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "foo {
    ///     bar {
    ///         contains: $foo
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse_node(input),
    ///     Ok((
    ///         QueryWhere::Node {
    ///             name: "foo".to_string(),
    ///             nodes: vec![QueryWhere::Node {
    ///                 name: "bar".to_string(),
    ///                 nodes: vec![QueryWhere::Condition(
    ///                     QueryCondition::Contains("foo".to_string())
    ///                 )]
    ///             }]
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    pub fn parse_node(input: &str) -> ParseResult<Self> {
        let (name, input) = alphabetics(input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;

        let (structure, input) = many1(&input, |input| {
            let (_, input) = spaces(input)?;
            let (where_clause, input) =
                choice(&input, vec![Self::parse_condition, Self::parse_node])?;
            let (_, input) = spaces(&input)?;

            Ok((where_clause, input))
        })?;

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((
            Self::Node {
                name,
                nodes: structure,
            },
            input,
        ))
    }

    /// Parse a where clause from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid where
    /// clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "where {
    ///     foo {
    ///         contains: $foo
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse(input),
    ///     Ok((
    ///         QueryWhere::Node {
    ///             name: "foo".to_string(),
    ///             nodes: vec![QueryWhere::Condition(QueryCondition::Contains(
    ///                 "foo".to_string()
    ///             ))]
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "where")?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (where_clause, input) =
            choice(&input, vec![Self::parse_condition, Self::parse_node])?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((where_clause, input))
    }

    /// Return all references used in the conditions of this where clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     dragonfly::ast::{
    ///         QueryCondition,
    ///         QueryWhere,
    ///     },
    ///     std::collections::HashSet,
    /// };
    ///
    /// let where_clause = QueryWhere::Node {
    ///     name: "foo".to_string(),
    ///     nodes: vec![
    ///         QueryWhere::Node {
    ///             name: "bar".to_string(),
    ///             nodes: vec![
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "bar".to_string(),
    ///                 )),
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "baz".to_string(),
    ///                 )),
    ///             ],
    ///         },
    ///         QueryWhere::Node {
    ///             name: "baz".to_string(),
    ///             nodes: vec![
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "bar".to_string(),
    ///                 )),
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "foo".to_string(),
    ///                 )),
    ///             ],
    ///         },
    ///     ],
    /// };
    ///
    /// let mut expected = HashSet::new();
    ///
    /// expected.insert("foo".to_string());
    /// expected.insert("bar".to_string());
    /// expected.insert("baz".to_string());
    ///
    /// assert_eq!(where_clause.references(), expected);
    /// ```
    pub fn references(&self) -> HashSet<String> {
        let mut references = HashSet::new();

        match self {
            Self::Condition(
                Condition::Contains(reference) | Condition::Equals(reference),
            ) => {
                references.insert(reference.clone());
            }
            Self::Node { nodes, .. } => {
                references.extend(
                    nodes
                        .iter()
                        .flat_map(Self::references)
                        .collect::<HashSet<String>>(),
                );
            }
        }

        references
    }
}

/// A query argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argument {
    /// The name of the argument. Used inside the conditions of
    /// the where clause.
    pub name: String,
    /// The type of the argument.
    pub r#type: Type,
}

impl Argument {
    /// Parse an argument from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     QueryArgument::parse("$name: String"),
    ///     Ok((
    ///         QueryArgument {
    ///             name: "name".to_string(),
    ///             r#type: Type::Scalar(Scalar::String),
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = Query::parse_reference(input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((Self { name, r#type }, input))
    }
}

/// The structure of the data that the query should return.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Schema {
    /// A leaf node: a field.
    Field(String),
    /// A node with children. Either the root node or a relation.
    Model {
        /// The name of the node.
        name: String,
        /// The children of the node; fields or relations.
        nodes: Vec<Self>,
    },
}

impl Schema {
    fn parse_node(input: &str) -> ParseResult<Self> {
        let (name, input) = alphabetics(input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;

        let (structure, input) = many1(&input, |input| {
            let (_, input) = spaces(input)?;
            let (schema, input) = Self::parse(&input)?;
            let (_, input) = spaces(&input)?;

            Ok((schema, input))
        })?;

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((
            Self::Model {
                name,
                nodes: structure,
            },
            input,
        ))
    }

    fn parse_identifier(input: &str) -> ParseResult<Self> {
        map(input, alphabetics, Self::Field)
    }

    /// Parse a schema from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid schema.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchema;
    ///
    /// assert_eq!(
    ///     QuerySchema::parse("user"),
    ///     Ok((QuerySchema::Field("user".to_string()), "".to_string())),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchema;
    ///
    /// let input = "user {
    ///   name
    /// }";
    ///
    /// assert_eq!(
    ///     QuerySchema::parse(input),
    ///     Ok((
    ///         QuerySchema::Model {
    ///             name: "user".to_string(),
    ///             nodes: vec![QuerySchema::Field("name".to_string())],
    ///         },
    ///         "".to_string()
    ///     )),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchema;
    ///
    /// let input = "user {
    ///   name {
    ///     first
    ///     last
    ///   }
    /// }";
    ///
    /// assert_eq!(
    ///     QuerySchema::parse(input),
    ///     Ok((
    ///         QuerySchema::Model {
    ///             name: "user".to_string(),
    ///             nodes: vec![QuerySchema::Model {
    ///                 name: "name".to_string(),
    ///                 nodes: vec![
    ///                     QuerySchema::Field("first".to_string()),
    ///                     QuerySchema::Field("last".to_string()),
    ///                 ]
    ///             }]
    ///         },
    ///         "".to_string()
    ///     )),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchema;
    ///
    /// let input = "user";
    ///
    /// assert_eq!(
    ///     QuerySchema::parse(input),
    ///     Ok((QuerySchema::Field("user".to_string()), "".to_string())),
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice(input, vec![Self::parse_node, Self::parse_identifier])
    }

    /// Check if the schema is empty. The schema is empty if the root node has
    /// no children.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchema;
    ///
    /// let schema = QuerySchema::Field("user".to_string());
    ///
    /// assert_eq!(schema.is_empty(), true);
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchema;
    ///
    /// let schema = QuerySchema::Model {
    ///     name: "user".to_string(),
    ///     nodes: vec![QuerySchema::Field("name".to_string())],
    /// };
    ///
    /// assert_eq!(schema.is_empty(), false);
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Field(_) => true,
            Self::Model { nodes: schema, .. } => schema.is_empty(),
        }
    }
}

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
    ///         nodes: vec![Schema::Field("title".to_string())],
    ///     },
    ///     r#type: Type::Array(Scalar::Reference("Image".to_string())),
    ///     r#where: Some(QueryWhere::Node {
    ///         name: "image".to_string(),
    ///         nodes: vec![QueryWhere::Node {
    ///             name: "title".to_string(),
    ///             nodes: vec![
    ///                 QueryWhere::Condition(Condition::Equals(
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
    pub fn check_non_empty_schema(&self) -> Result<(), TypeError> {
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
}
