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
    std::io,
};

/// A selection node.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
    /// # Errors
    ///
    /// Returns an error if writing to the output stream fails.
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
    ///         name: "images".to_owned(),
    ///         arguments: vec![Argument::new(
    ///             "after",
    ///             Value::variable("endCursor"),
    ///         )],
    ///         directives: vec![],
    ///         selections: vec![Selection::Field(Field::new("id"))],
    ///     }),
    ///     Selection::FragmentSpread(FragmentSpread::new("foo", &[])),
    ///     Selection::InlineFragment(InlineFragment {
    ///         type_condition: "Image".to_owned(),
    ///         directives: vec![],
    ///         selections: vec![Selection::Field(Field::new("id"))],
    ///     }),
    /// ];
    ///
    /// let mut f = Vec::new();
    ///
    /// Selection::print_multiple(&selections, 0, &mut f).unwrap();
    ///
    /// assert_eq!(
    ///     String::from_utf8(f).unwrap(),
    ///     " {
    ///   images(after: $endCursor) {
    ///     id
    ///   }
    ///   ...foo
    ///   ... on Image {
    ///     id
    ///   }
    /// }
    /// "
    /// );
    /// ```
    pub fn print_multiple(
        selections: &[Self],
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        writeln!(f, " {{")?;

        for selection in selections {
            selection.print(level, f)?;
        }

        writeln!(f, "{}}}", indent::graphql(level))
    }
}

impl Print for Selection {
    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let level = level + 1;

        match self {
            Self::Field(field) => field.print(level, f),
            Self::FragmentSpread(spread) => spread.print(level, f),
            Self::InlineFragment(inline) => inline.print(level, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::{
            graphql::Argument,
            printer::print::Print,
        },
    };

    #[test]
    fn test_print_selection() {
        let field = Selection::Field(Field {
            name: "images".to_owned(),
            arguments: vec![Argument::variable("after", "endCursor")],
            directives: vec![],
            selections: vec![],
        });

        let mut f = Vec::new();

        field.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "  images(after: $endCursor)\n"
        );
    }

    #[test]
    fn test_print_fragment_spread() {
        let spread = FragmentSpread::new("name", &[]);
        let mut f = Vec::new();

        spread.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "...name\n");
    }

    #[test]
    fn test_print_inline_fragment() {
        let inline = InlineFragment {
            type_condition: "Type".to_owned(),
            directives: vec![],
            selections: vec![],
        };

        let mut f = Vec::new();

        inline.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "... on Type {\n}\n");
    }
}
