/// A query condition operator.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Operator {
    /// Equals.
    Equals,
    /// Contains.
    Contains,
}

impl From<ast::query::r#where::Operator> for Operator {
    fn from(value: ast::query::r#where::Operator) -> Self {
        match value {
            ast::query::r#where::Operator::Equals => Self::Equals,
            ast::query::r#where::Operator::Contains => Self::Contains,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ast() {
        assert_eq!(
            Operator::from(ast::query::r#where::Operator::Equals),
            Operator::Equals
        );

        assert_eq!(
            Operator::from(ast::query::r#where::Operator::Contains),
            Operator::Contains
        );
    }
}
