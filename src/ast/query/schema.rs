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
    Relation(String, Vec<Self>),
}

impl Node {
    /// Create a new field node.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn field(name: &str) -> Self {
        Self::Field(name.to_owned())
    }

    /// Create a new relation node.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    /// * `nodes` - The children of the model.
    #[must_use]
    pub fn relation(
        name: &str,
        nodes: &[Self],
    ) -> Self {
        Self::Relation(name.to_owned(), nodes.to_owned())
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
    ///     QuerySchemaNode::parse("foo { bar baz }"),
    ///     Ok((
    ///         QuerySchemaNode::relation(
    ///             "foo",
    ///             &[QuerySchemaNode::field("bar"), QuerySchemaNode::field("baz"),]
    ///         ),
    ///         String::new(),
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

        Ok((Self::Relation(name, nodes), input))
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
    ///     Ok((QuerySchemaNode::Field("foo".to_owned()), String::new())),
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
    ///     Ok((QuerySchemaNode::field("user"), String::new())),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let input = "
    ///
    /// user {
    ///   name
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse(input),
    ///     Ok((
    ///         QuerySchemaNode::relation(
    ///             "user",
    ///             &[QuerySchemaNode::field("name")]
    ///         ),
    ///         String::new()
    ///     )),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let input = "
    ///
    /// user {
    ///   name {
    ///     first
    ///     last
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     QuerySchemaNode::parse(input),
    ///     Ok((
    ///         QuerySchemaNode::relation(
    ///             "user",
    ///             &[QuerySchemaNode::relation(
    ///                 "name",
    ///                 &[
    ///                     QuerySchemaNode::Field("first".to_owned()),
    ///                     QuerySchemaNode::Field("last".to_owned()),
    ///                 ]
    ///             )]
    ///         ),
    ///         String::new()
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
    ///     Ok((QuerySchemaNode::Field("user".to_owned()), String::new())),
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
    /// let schema = QuerySchemaNode::field("user");
    ///
    /// assert!(schema.is_empty());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let schema = QuerySchemaNode::relation("user", &[]);
    ///
    /// assert!(schema.is_empty());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::QuerySchemaNode;
    ///
    /// let schema =
    ///     QuerySchemaNode::relation("user", &[QuerySchemaNode::field("name")]);
    ///
    /// assert_eq!(schema.is_empty(), false);
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Field(_) => true,
            Self::Relation(_, schema) => schema.is_empty(),
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
    /// Create a new schema.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the root node.
    /// * `nodes` - The children of the root node; fields or relations.
    #[must_use]
    pub fn new(
        name: &str,
        nodes: &[Node],
    ) -> Self {
        Self {
            name: name.to_owned(),
            nodes: nodes.to_owned(),
        }
    }

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
    /// let input = "
    ///
    /// user {
    ///   name
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     QuerySchema::parse(input),
    ///     Ok((
    ///         QuerySchema {
    ///             name: "user".to_owned(),
    ///             nodes: vec![QuerySchemaNode::Field("name".to_owned())],
    ///         },
    ///         String::new()
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
    /// let input = "
    ///
    /// user {
    ///   name {
    ///     first
    ///     last
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     QuerySchema::parse(input),
    ///     Ok((
    ///         QuerySchema::new(
    ///             "user",
    ///             &[QuerySchemaNode::relation(
    ///                 "name",
    ///                 &[
    ///                     QuerySchemaNode::field("first"),
    ///                     QuerySchemaNode::field("last"),
    ///                 ]
    ///             )]
    ///         ),
    ///         String::new()
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
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field_node() {
        assert_eq!(Node::field("name"), Node::Field("name".to_owned()));
    }

    #[test]
    fn test_model_node() {
        assert_eq!(
            Node::relation("user", &[Node::field("name")]),
            Node::Relation("user".to_owned(), vec![Node::field("name")])
        );
    }

    #[test]
    fn test_schema_is_empty() {
        assert!(Schema::new("user", &[]).is_empty());
        assert!(!Schema::new("user", &[Node::field("name")]).is_empty());
        assert!(!Schema::new("user", &[Node::relation("name", &[])]).is_empty());
    }
}
