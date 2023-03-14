use std::borrow::Cow;

/// A model relation type.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Type {
    /// A one-to-one relation.
    OneToOne,
    /// A one-to-many relation.
    OneToMany,
    /// A many-to-one relation.
    ManyToOne,
    /// A many-to-many relation.
    ManyToMany,
}

/// A model relation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Relation<'a> {
    /// The name of the model.
    pub model_name: Cow<'a, str>,
    /// The cardinality of the relation.
    pub r#type: Type,
}
