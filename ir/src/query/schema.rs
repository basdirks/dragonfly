use std::borrow::Cow;

/// A query schema node.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum Node<'a> {
    /// A field.
    Field {
        /// The name of the field.
        name: Cow<'a, str>,
    },
    /// A relation.
    Relation {
        /// The name of the relation.
        name: Cow<'a, str>,
        /// The nodes of the relation.
        nodes: Vec<Node<'a>>,
    },
}

/// A query schema.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Schema<'a> {
    /// The alias of the schema.
    pub alias: Cow<'a, str>,
    /// The nodes of the schema.
    pub nodes: Vec<Node<'a>>,
}
