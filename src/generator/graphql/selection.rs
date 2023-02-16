use {
    super::{
        field::Field,
        fragment::{
            Inline,
            Spread,
        },
    },
    crate::generator::printer::print::Print,
};

/// A selection node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selection {
    /// A field.
    Field(Field),
    /// A fragment spread.
    FragmentSpread(Spread),
    /// An inline fragment.
    InlineFragment(Inline),
}

impl Print for Selection {
    fn print(
        &self,
        level: usize,
    ) -> String {
        match self {
            Self::Field(field) => field.print(level),
            Self::FragmentSpread(spread) => spread.print(level),
            Self::InlineFragment(inline) => inline.print(level),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::{
            graphql::{
                directive::Argument,
                value::Value,
            },
            printer::print::Print,
        },
    };

    #[test]
    fn test_print_selection() {
        let field = Selection::Field(Field {
            name: "images".to_string(),
            arguments: vec![Argument {
                name: "after".to_string(),
                value: Value::Variable("endCursor".to_string()),
            }],
            directives: vec![],
            selections: vec![],
        });

        assert_eq!(field.print(0), "images(after: $endCursor)");
    }

    #[test]
    fn test_print_fragment_spread() {
        let spread = Spread {
            name: "name".to_string(),
            directives: vec![],
        };

        assert_eq!(spread.print(0), "...name");
    }

    #[test]
    fn test_print_inline_fragment() {
        let inline = Inline {
            type_condition: "Type".to_string(),
            directives: vec![],
            selections: vec![],
        };

        assert_eq!(
            inline.print(0),
            "... on Type {

}"
        );
    }
}
