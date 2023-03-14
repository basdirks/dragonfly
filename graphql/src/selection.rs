use {
    super::{
        Field,
        FragmentSpread,
        InlineFragment,
    },
    printer::Print,
    std::io,
};

/// A selection node.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Selection<'a> {
    /// A field.
    Field(Field<'a>),
    /// A fragment spread.
    FragmentSpread(FragmentSpread<'a>),
    /// An inline fragment.
    InlineFragment(InlineFragment<'a>),
}

impl<'a> Selection<'a> {
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
    /// use {
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
    /// let selections = vec![Selection::Field(Field {
    ///     name: "images".into(),
    ///     arguments: vec![Argument {
    ///         name: "after".into(),
    ///         value: Value::Variable("endCursor".into()),
    ///     }],
    ///     directives: Vec::new(),
    ///     selections: vec![
    ///         Selection::FragmentSpread(FragmentSpread {
    ///             name: "foo".into(),
    ///             directives: Vec::new(),
    ///         }),
    ///         Selection::InlineFragment(InlineFragment {
    ///             type_condition: "Image".into(),
    ///             directives: Vec::new(),
    ///             selections: vec![Selection::Field(Field {
    ///                 name: "id".into(),
    ///                 arguments: Vec::new(),
    ///                 directives: Vec::new(),
    ///                 selections: Vec::new(),
    ///             })],
    ///         }),
    ///     ],
    /// })];
    ///
    /// let mut f = Vec::new();
    ///
    /// Selection::print_multiple(selections.iter(), 0, &mut f).unwrap();
    ///
    /// assert_eq!(
    ///     String::from_utf8(f).unwrap(),
    ///     " {
    ///   images(after: $endCursor) {
    ///     ...foo
    ///     ... on Image {
    ///       id
    ///     }
    ///   }
    /// }
    /// "
    /// );
    /// ```
    pub fn print_multiple<T>(
        selections: T,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()>
    where
        T: Iterator<Item = &'a Self>,
    {
        writeln!(f, " {{")?;

        for selection in selections {
            selection.print(level + 1, f)?;
        }

        writeln!(f, "{}}}", Self::indent(level))
    }
}

impl Print for Selection<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
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
        crate::{
            Argument,
            Value,
        },
        printer::Print,
    };

    #[test]
    fn test_print_selection() {
        let field = Selection::Field(Field {
            name: "images".into(),
            arguments: vec![Argument {
                name: "after".into(),
                value: Value::Variable("endCursor".into()),
            }],
            directives: Vec::new(),
            selections: Vec::new(),
        });

        let mut f = Vec::new();

        field.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "images(after: $endCursor)\n"
        );
    }

    #[test]
    fn test_print_fragment_spread() {
        let spread = Selection::FragmentSpread(FragmentSpread {
            name: "name".into(),
            directives: Vec::new(),
        });

        let mut f = Vec::new();

        spread.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "...name\n");
    }

    #[test]
    fn test_print_inline_fragment() {
        let inline = Selection::InlineFragment(InlineFragment {
            type_condition: "Type".into(),
            directives: Vec::new(),
            selections: Vec::new(),
        });

        let mut f = Vec::new();

        inline.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "... on Type {\n}\n");
    }
}
