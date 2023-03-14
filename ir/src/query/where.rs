use {
    super::Condition,
    std::borrow::Cow,
};

/// A query where clause.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Where<'a> {
    /// The alias of the where clause.
    pub alias: Cow<'a, str>,
    /// The conditions of the where clause.
    pub conditions: Vec<Condition<'a>>,
}
