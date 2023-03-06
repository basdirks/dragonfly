use {
    super::{
        operator::Operator,
        path::Path,
    },
    std::fmt::Display,
};

/// A condition that must be met for a query to return a result.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Condition {
    /// The path to the field that must meet the condition.
    pub path: Path,
    /// The type of the condition.
    pub operator: Operator,
    /// The right-hand side of the condition.
    pub argument_name: String,
}

impl Display for Condition {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            path,
            operator,
            argument_name,
        } = self;

        write!(f, "{path} {operator} ${argument_name}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_condition() {
        assert_eq!(
            Condition {
                path: Path::new(&["foo", "bar"]),
                operator: Operator::Contains,
                argument_name: "baz".to_owned(),
            }
            .to_string(),
            "foo.bar contains $baz"
        );

        assert_eq!(
            Condition {
                path: Path::new(&["foo", "bar", "baz"]),
                operator: Operator::Equals,
                argument_name: "baz".to_owned(),
            }
            .to_string(),
            "foo.bar.baz equals $baz"
        );
    }
}
