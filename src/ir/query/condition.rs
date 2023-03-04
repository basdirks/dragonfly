use {
    super::Operator,
    std::collections::VecDeque,
};

/// A query condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Condition {
    /// The lhs operand (the path to the field).
    pub lhs: VecDeque<String>,
    /// The condition operator.
    pub operator: Operator,
    /// The rhs operand (the argument name).
    pub rhs: String,
}
