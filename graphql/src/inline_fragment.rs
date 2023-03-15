use {
    super::{
        Directive,
        Selection,
    },
    print::{
        Print,
        PrintInline,
    },
    std::{
        self,
        borrow::Cow,
        io,
    },
};

/// An inline fragment.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct InlineFragment<'a> {
    /// The type condition of the fragment.
    pub type_condition: Cow<'a, str>,
    /// The directives of the fragment.
    pub directives: Vec<Directive<'a>>,
    /// The selection set of the fragment.
    pub selections: Vec<Selection<'a>>,
}

impl Print for InlineFragment<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}... on {}", Self::indent(level), self.type_condition)?;

        for directive in &self.directives {
            directive.print(f)?;
        }

        Selection::print_multiple(self.selections.iter(), level, f)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Argument,
            Field,
            Value,
        },
    };

    #[test]
    fn test_print() {
        let inline = InlineFragment {
            type_condition: "Foo".into(),
            directives: vec![Directive {
                name: "bar".into(),
                arguments: vec![Argument {
                    name: "baz".into(),
                    value: Value::String("bax".into()),
                }],
            }],
            selections: vec![Selection::Field(Field {
                name: "bar".into(),
                arguments: Vec::new(),
                directives: vec![Directive {
                    name: "foo".into(),
                    arguments: vec![Argument {
                        name: "bar".into(),
                        value: Value::String("baz".into()),
                    }],
                }],
                selections: vec![
                    Selection::Field(Field {
                        name: "baz".into(),
                        arguments: Vec::new(),
                        directives: Vec::new(),
                        selections: Vec::new(),
                    }),
                    Selection::Field(Field {
                        name: "bax".into(),
                        arguments: Vec::new(),
                        directives: Vec::new(),
                        selections: Vec::new(),
                    }),
                ],
            })],
        };

        let mut f = Vec::new();

        inline.print(1, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "  ... on Foo @bar(baz: \"bax\") {
    bar @foo(bar: \"baz\") {
      baz
      bax
    }
  }
"
        );
    }
}
