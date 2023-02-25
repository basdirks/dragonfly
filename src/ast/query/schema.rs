use crate::parser::{
    alphabetics,
    brace_close,
    brace_open,
    camel_case,
    choice,
    many1,
    map,
    spaces,
    ParseResult,
};

/// A schema node.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Node {
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

impl Node {
    /// Parse a schema node from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse("foo { bar baz }"),
    ///     Ok((
    ///         QuerySchemaNode::Model {
    ///             name: "foo".to_owned(),
    ///             nodes: vec![
    ///                 QuerySchemaNode::Field("bar".to_owned()),
    ///                 QuerySchemaNode::Field("baz".to_owned()),
    ///             ],
    ///         },
    ///         "".to_owned(),
    ///     )),
    /// );
    /// ```
    pub fn parse_model(input: &str) -> ParseResult<Self> {
        let (name, input) = alphabetics(input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;

        let (nodes, input) = many1(&input, |input| {
            let (_, input) = spaces(input)?;
            let (schema, input) = Self::parse(&input)?;
            let (_, input) = spaces(&input)?;

            Ok((schema, input))
        })?;

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self::Model { name, nodes }, input))
    }

    /// Parse a schema field from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::QuerySchemaNode,
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse_field("foo"),
    ///     Ok((QuerySchemaNode::Field("foo".to_owned()), "".to_owned())),
    /// );
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse_field("foo { bar }"),
    ///     Ok((
    ///         QuerySchemaNode::Field("foo".to_owned()),
    ///         " { bar }".to_owned()
    ///     )),
    /// );
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse_field("Foo { bar }"),
    ///     Err(ParseError::UnexpectedChar {
    ///         message: "Expected camelCase identifier to start with lowercase \
    ///                   character, found 'F'."
    ///             .to_string(),
    ///         actual: 'F',
    ///     }),
    /// );
    /// ```
    pub fn parse_field(input: &str) -> ParseResult<Self> {
        map(input, camel_case, Self::Field)
    }

    /// Parse a schema node from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse("user"),
    ///     Ok((QuerySchemaNode::Field("user".to_owned()), "".to_owned())),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let input = "user {
    ///   name
    /// }";
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse(input),
    ///     Ok((
    ///         QuerySchemaNode::Model {
    ///             name: "user".to_owned(),
    ///             nodes: vec![QuerySchemaNode::Field("name".to_owned())],
    ///         },
    ///         "".to_owned()
    ///     )),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let input = "user {
    ///   name {
    ///     first
    ///     last
    ///   }
    /// }";
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse(input),
    ///     Ok((
    ///         QuerySchemaNode::Model {
    ///             name: "user".to_owned(),
    ///             nodes: vec![QuerySchemaNode::Model {
    ///                 name: "name".to_owned(),
    ///                 nodes: vec![
    ///                     QuerySchemaNode::Field("first".to_owned()),
    ///                     QuerySchemaNode::Field("last".to_owned()),
    ///                 ]
    ///             }]
    ///         },
    ///         "".to_owned()
    ///     )),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let input = "user";
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse(input),
    ///     Ok((QuerySchemaNode::Field("user".to_owned()), "".to_owned())),
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice(input, vec![Self::parse_model, Self::parse_field])
    }

    /// Check if the schema is empty. The schema is empty if the root node has
    /// no children.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let schema = QuerySchemaNode::Field("user".to_owned());
    ///
    /// assert!(schema.is_empty());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let schema = QuerySchemaNode::Model {
    ///     name: "user".to_owned(),
    ///     nodes: vec![],
    /// };
    ///
    /// assert!(schema.is_empty());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let schema = QuerySchemaNode::Model {
    ///     name: "user".to_owned(),
    ///     nodes: vec![QuerySchemaNode::Field("name".to_owned())],
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

/// The structure of the data that the query should return.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Schema {
    /// The name of the root node.
    pub name: String,
    /// The children of the root node; fields or relations.
    pub nodes: Vec<Node>,
}

impl Schema {
    /// Parse a schema from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid schema.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::QuerySchema,
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(QuerySchema::parse("user"), Err(ParseError::UnexpectedEof));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QuerySchema,
    ///     QuerySchemaNode,
    /// };
    ///
    /// let input = "user {
    ///   name
    /// }";
    ///
    /// assert_eq!(
    ///     QuerySchema::parse(input),
    ///     Ok((
    ///         QuerySchema {
    ///             name: "user".to_owned(),
    ///             nodes: vec![QuerySchemaNode::Field("name".to_owned())],
    ///         },
    ///         "".to_owned()
    ///     )),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QuerySchema,
    ///     QuerySchemaNode,
    /// };
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
    ///         QuerySchema {
    ///             name: "user".to_owned(),
    ///             nodes: vec![QuerySchemaNode::Model {
    ///                 name: "name".to_owned(),
    ///                 nodes: vec![
    ///                     QuerySchemaNode::Field("first".to_owned()),
    ///                     QuerySchemaNode::Field("last".to_owned()),
    ///                 ]
    ///             }]
    ///         },
    ///         "".to_owned()
    ///     )),
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = alphabetics(input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;

        let (nodes, input) = many1(&input, |input| {
            let (schema, input) = Node::parse(input)?;
            let (_, input) = spaces(&input)?;

            Ok((schema, input))
        })?;

        let (_, input) = brace_close(&input)?;

        Ok((Self { name, nodes }, input))
    }

    /// Check if the schema is empty. The schema is empty if the root node has
    /// no children.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchema;
    ///
    /// let schema = QuerySchema {
    ///     name: "user".to_owned(),
    ///     nodes: vec![],
    /// };
    ///
    /// assert!(schema.is_empty());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QuerySchema,
    ///     QuerySchemaNode,
    /// };
    ///
    /// let schema = QuerySchema {
    ///     name: "user".to_owned(),
    ///     nodes: vec![QuerySchemaNode::Field("name".to_owned())],
    /// };
    ///
    /// assert!(!schema.is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}
