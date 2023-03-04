/// A query schema node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Node {
    /// A field.
    Field(String),
    /// A relation.
    Relation(String, Vec<Node>),
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
    /// * `name` - The name of the relation.
    /// * `nodes` - The nodes of the relation.
    #[must_use]
    pub fn relation(
        name: &str,
        nodes: &[Self],
    ) -> Self {
        Self::Relation(name.to_owned(), nodes.to_owned())
    }
}

/// A query schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schema {
    /// The alias of the schema.
    pub alias: String,
    /// The nodes of the schema.
    pub nodes: Vec<Node>,
}

impl Schema {
    /// Create a new query schema.
    ///
    /// # Arguments
    ///
    /// * `alias` - The alias of the schema.
    /// * `nodes` - The nodes of the schema.
    #[must_use]
    pub fn new(
        alias: &str,
        nodes: &[Node],
    ) -> Self {
        Self {
            alias: alias.to_owned(),
            nodes: nodes.to_owned(),
        }
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
    fn test_relation_node() {
        assert_eq!(
            Node::relation("user", &[]),
            Node::Relation("user".to_owned(), vec![])
        );
    }

    #[test]
    fn test_new() {
        assert_eq!(
            Schema::new("user", &[]),
            Schema {
                alias: "user".to_owned(),
                nodes: vec![],
            }
        );
    }
}
