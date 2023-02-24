use {
    super::{
        Directive,
        Selection,
    },
    crate::generator::printer::{
        indent,
        Print,
    },
    std::fmt::Write,
};

/// A fragment spread.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Spread {
    /// The name of the fragment.
    pub name: String,
    /// The directives of the fragment.
    pub directives: Vec<Directive>,
}

impl Print for Spread {
    fn print(
        &self,
        level: usize,
    ) -> String {
        let mut output = format!("{}...{}", indent::graphql(level), self.name);

        for directive in &self.directives {
            let _ = write!(output, " {directive}");
        }

        output
    }
}

/// An inline fragment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Inline {
    /// The type condition of the fragment.
    pub type_condition: String,
    /// The directives of the fragment.
    pub directives: Vec<Directive>,
    /// The selection set of the fragment.
    pub selections: Vec<Selection>,
}

impl Print for Inline {
    fn print(
        &self,
        level: usize,
    ) -> String {
        let mut output = format!(
            "{}... on {}",
            indent::graphql(level),
            self.type_condition,
        );

        for directive in &self.directives {
            let _ = write!(output, " {directive}");
        }

        output.push_str(&Selection::print_multiple(&self.selections, level));

        output
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::graphql::{
            Argument,
            Field,
            Value,
        },
    };

    #[test]
    fn test_print_spread() {
        let spread = Spread {
            name: "foo".to_owned(),
            directives: vec![],
        };

        assert_eq!(spread.print(0), "...foo");

        let spread = Spread {
            name: "daboi".to_owned(),
            directives: vec![Directive {
                name: "is".to_owned(),
                arguments: vec![Argument {
                    name: "a".to_owned(),
                    value: Value::String("good boy.".to_owned()),
                }],
            }],
        };

        assert_eq!(spread.print(0), "...daboi @is(a: \"good boy.\")");
    }

    #[test]
    fn test_print_inline() {
        let inline = Inline {
            type_condition: "Foo".to_owned(),
            directives: vec![],
            selections: vec![Selection::Field(Field {
                name: "bar".to_owned(),
                arguments: vec![],
                directives: vec![],
                selections: vec![
                    Selection::Field(Field {
                        name: "baz".to_owned(),
                        arguments: vec![],
                        directives: vec![],
                        selections: vec![],
                    }),
                    Selection::Field(Field {
                        name: "bax".to_owned(),
                        arguments: vec![],
                        directives: vec![],
                        selections: vec![],
                    }),
                ],
            })],
        };

        assert_eq!(
            inline.print(1),
            "  ... on Foo {
    bar {
      baz
      bax
    }
  }"
        );
    }
}
