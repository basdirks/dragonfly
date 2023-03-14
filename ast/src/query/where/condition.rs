use {
    super::{
        operator::Operator,
        path::Path,
    },
    std::{
        borrow::Cow,
        fmt::{
            self,
            Display,
            Formatter,
        },
    },
};

/// A condition that must be met for a query to return a result.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Condition<'a> {
    /// The path to the field that must meet the condition.
    pub path: Path<'a>,
    /// The type of the condition.
    pub operator: Operator,
    /// The right-hand side of the condition.
    pub argument_name: Cow<'a, str>,
}

impl Display for Condition<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
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
                path: Path::from_iter(["foo", "bar"]),
                operator: Operator::Contains,
                argument_name: "baz".into(),
            }
            .to_string(),
            "foo.bar contains $baz"
        );

        assert_eq!(
            Condition {
                path: Path::from_iter(["foo", "bar", "baz"]),
                operator: Operator::Equals,
                argument_name: "baz".into(),
            }
            .to_string(),
            "foo.bar.baz equals $baz"
        );
    }
}
