use {
    crate::Cardinality,
    std::borrow::Cow,
};

/// An enum relation.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct EnumRelation<'a> {
    /// The name of the enum.
    pub name: Cow<'a, str>,
    /// The cardinality of the relation.
    pub cardinality: Cardinality,
}
