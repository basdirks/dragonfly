use {
    crate::{
        Cardinality,
        Type,
    },
    std::borrow::Cow,
};

/// A data field.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Field<'a> {
    /// The name of the field.
    pub name: Cow<'a, str>,
    /// The type of the field.
    pub r#type: Type,
    /// The cardinality of the field.
    pub cardinality: Cardinality,
}
