use {
    super::Operator,
    std::{
        borrow::Cow,
        collections::VecDeque,
    },
};

/// A query condition.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Condition<'a> {
    /// The lhs operand (the path to the field).
    pub lhs: VecDeque<Cow<'a, str>>,
    /// The condition operator.
    pub operator: Operator,
    /// The rhs operand (the argument name).
    pub rhs: Cow<'a, str>,
}
