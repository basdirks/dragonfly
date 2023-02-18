use {
    super::{
        Field,
        FragmentSpread,
        InlineFragment,
    },
    crate::generator::printer::{
        indent,
        Print,
    },
};

/// A selection node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selection {
    /// A field.
    Field(Field),
    /// A fragment spread.
    FragmentSpread(FragmentSpread),
    /// An inline fragment.
    InlineFragment(InlineFragment),
}

impl Selection {
    /// Print multiple selections.
    ///
    /// # Arguments
    ///
    /// * `selections` - The selections to print.
    /// * `level` - The indentation level.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::{
    ///     graphql::{
    ///         Argument,
    ///         Field,
    ///         FragmentSpread,
    ///         InlineFragment,
    ///         Selection,
    ///         Value,
    ///     },
    ///     printer::Print,
    /// };
    ///
    /// let selections = vec![
    ///     Selection::Field(Field {
    ///         name: "images".to_string(),
    ///         arguments: vec![Argument {
    ///             name: "after".to_string(),
    ///             value: Value::Variable("endCursor".to_string()),
    ///         }],
    ///         directives: vec![],
    ///         selections: vec![Selection::Field(Field {
    ///             name: "id".to_string(),
    ///             arguments: vec![],
    ///             directives: vec![],
    ///             selections: vec![],
    ///         })],
    ///     }),
    ///     Selection::FragmentSpread(Spread {
    ///         name: "foo".to_string(),
    ///         directives: vec![],
    ///     }),
    ///     Selection::InlineFragment(Inline {
    ///         type_condition: "Image".to_string(),
    ///         directives: vec![],
    ///         selections: vec![Selection::Field(Field {
    ///             name: "id".to_string(),
    ///             arguments: vec![],
    ///             directives: vec![],
    ///             selections: vec![],
    ///         })],
    ///     }),
    /// ];
    ///
    /// assert_eq!(
    ///     Selection::print_multiple(&selections, 0),
    ///     " {
    ///   images(after: $endCursor) {
    ///     id
    ///   }
    ///   ...foo
    ///   ... on Image {
    ///     id
    ///   }
    /// }"
    /// );
    /// ```
    #[must_use]
    pub fn print_multiple(
        selections: &[Self],
        level: usize,
    ) -> String {
        format!(
            " {{\n{}\n{}}}",
            selections
                .iter()
                .map(|selection| selection.print(level + 1))
                .collect::<Vec<_>>()
                .join("\n"),
            indent::graphql(level)
        )
    }
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
                Argument,
                Value,
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
        let spread = FragmentSpread {
            name: "name".to_string(),
            directives: vec![],
        };

        assert_eq!(spread.print(0), "...name");
    }

    #[test]
    fn test_print_inline_fragment() {
        let inline = InlineFragment {
            type_condition: "Type".to_string(),
            directives: vec![],
            selections: vec![],
        };

        assert_eq!(
            inline.print(0),
            "\
... on Type {

}"
        );
    }
}
