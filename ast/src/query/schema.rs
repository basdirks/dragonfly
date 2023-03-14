use {
    parser::{
        alphabetics,
        brace_close,
        brace_open,
        camel_case,
        choice,
        many1,
        map,
        spaces,
        ParseResult,
    },
    std::borrow::Cow,
};

/// A schema node.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Node<'a> {
    /// A leaf node: a field.
    Field {
        /// The name of the field.
        name: Cow<'a, str>,
    },
    /// A node with children. Either the root node or a relation.
    Relation {
        /// The name of the model.
        name: Cow<'a, str>,
        /// The fields of the model.
        nodes: Vec<Self>,
    },
}

impl<'a> Node<'a> {
    /// Parse a schema node from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid node.
    fn parse_model(input: &str) -> ParseResult<Self> {
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

        Ok((
            Self::Relation {
                name: name.into(),
                nodes,
            },
            input,
        ))
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
    fn parse_field(input: &str) -> ParseResult<Self> {
        map(input, camel_case, |name| Self::Field { name: name.into() })
    }

    /// Parse a schema node from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid node.
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice(input, vec![Self::parse_model, Self::parse_field])
    }

    /// Check if the schema node is empty. The schema node is empty if it is a
    /// field or if the relation node has no children.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        match self {
            Self::Field { .. } => true,
            Self::Relation { nodes, .. } => nodes.is_empty(),
        }
    }
}

/// The structure of the data that the query should return.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Schema<'a> {
    /// The name of the root node.
    pub name: Cow<'a, str>,
    /// The children of the root node; fields or relations.
    pub nodes: Vec<Node<'a>>,
}

impl<'a> Schema<'a> {
    /// Parse a schema from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid schema.
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

        Ok((
            Self {
                name: name.into(),
                nodes,
            },
            input,
        ))
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
    use {
        super::*,
        parser::ParseError,
    };

    #[test]
    fn test_schema_node_is_empty() {
        assert!(Node::Field {
            name: "name".into()
        }
        .is_empty());

        assert!(!Node::Relation {
            name: "user".into(),
            nodes: vec![Node::Field {
                name: "name".into()
            }],
        }
        .is_empty());
    }

    #[test]
    fn test_schema_is_empty() {
        assert!(Schema {
            name: "user".into(),
            nodes: Vec::new(),
        }
        .is_empty());

        assert!(!Schema {
            name: "user".into(),
            nodes: vec![Node::Field {
                name: "name".into()
            }],
        }
        .is_empty());

        assert!(!Schema {
            name: "user".into(),
            nodes: vec![Node::Relation {
                name: "user".into(),
                nodes: vec![Node::Field {
                    name: "name".into()
                }],
            }],
        }
        .is_empty());
    }

    #[test]
    fn test_parse_field_node() {
        assert_eq!(
            Node::parse("name"),
            Ok((
                Node::Field {
                    name: "name".into()
                },
                String::new()
            ))
        );
    }

    #[test]
    fn test_parse_model_node() {
        assert_eq!(
            Node::parse("user { name }"),
            Ok((
                Node::Relation {
                    name: "user".into(),
                    nodes: vec![Node::Field {
                        name: "name".into()
                    }],
                },
                String::new()
            ))
        );
    }

    #[test]
    fn test_parse_schema() {
        assert_eq!(
            Schema::parse("user { name }"),
            Ok((
                Schema {
                    name: "user".into(),
                    nodes: vec![Node::Field {
                        name: "name".into()
                    }],
                },
                String::new()
            ))
        );
    }

    #[test]
    fn test_parse_schema_with_empty_node() {
        assert_eq!(
            Schema::parse("user { }"),
            Err(ParseError::unmatched_choice([
                ParseError::unexpected_char(
                    '}',
                    "Expected alphabetic character."
                ),
                ParseError::unexpected_char(
                    '}',
                    "Expected camelCase identifier to start with lowercase \
                     character, found '}'."
                )
            ]))
        );
    }
}
