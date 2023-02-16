use {
    super::{
        directive::Directive,
        selection::Selection,
    },
    crate::generator::printer::{
        indent,
        print::Print,
    },
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
            output.push_str(&format!(" {directive}"));
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
            output.push_str(&format!(" {directive}"));
        }

        output.push_str(&format!(
            " {{\n{}\n{}}}",
            self.selections
                .iter()
                .map(|selection| selection.print(level + 1))
                .collect::<Vec<_>>()
                .join("\n"),
            indent::graphql(level)
        ));

        output
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::graphql::{
            directive::Argument,
            field::Field,
            value::Value,
        },
    };

    #[test]
    fn test_print_spread() {
        let spread = Spread {
            name: "foo".to_string(),
            directives: vec![],
        };

        assert_eq!(spread.print(0), "...foo");

        let spread = Spread {
            name: "daboi".to_string(),
            directives: vec![Directive {
                name: "is".to_string(),
                arguments: vec![Argument {
                    name: "a".to_string(),
                    value: Value::String("good boy.".to_string()),
                }],
            }],
        };

        assert_eq!(spread.print(0), "...daboi @is(a: \"good boy.\")",);
    }

    #[test]
    fn test_display_inline() {
        let inline = Inline {
            type_condition: "Foo".to_string(),
            directives: vec![],
            selections: vec![Selection::Field(Field {
                name: "bar".to_string(),
                arguments: vec![],
                directives: vec![],
                selections: vec![
                    Selection::Field(Field {
                        name: "baz".to_string(),
                        arguments: vec![],
                        directives: vec![],
                        selections: vec![],
                    }),
                    Selection::Field(Field {
                        name: "bax".to_string(),
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
