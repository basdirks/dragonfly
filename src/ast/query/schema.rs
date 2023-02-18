use crate::parser::{
    alphabetics,
    brace_close,
    brace_open,
    choice,
    many1,
    map,
    spaces,
    ParseResult,
};

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
    /// Parse a schema node from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid schema
    /// node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchema;
    ///
    /// assert_eq!(
    ///     QuerySchema::parse("foo { bar baz }"),
    ///     Ok((
    ///         QuerySchema::Model {
    ///             name: "foo".to_string(),
    ///             nodes: vec![
    ///                 QuerySchema::Field("bar".to_string()),
    ///                 QuerySchema::Field("baz".to_string()),
    ///             ],
    ///         },
    ///         "".to_string(),
    ///     )),
    /// );
    /// ```
    pub fn parse_node(input: &str) -> ParseResult<Self> {
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
