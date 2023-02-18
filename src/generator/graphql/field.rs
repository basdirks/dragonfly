use {
    super::{
        Argument,
        Directive,
        Selection,
    },
    crate::generator::printer::{
        comma_separated,
        indent,
        Print,
    },
};

/// A selection field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The arguments of the field.
    pub arguments: Vec<Argument>,
    /// The directives of the field.
    pub directives: Vec<Directive>,
    /// The selections of the field.
    pub selections: Vec<Selection>,
}

impl Print for Field {
    fn print(
        &self,
        level: usize,
    ) -> String {
        let mut output =
            format!("{}{}", indent::graphql(level), self.name.clone());

        if !self.arguments.is_empty() {
            output.push_str(&format!("({})", comma_separated(&self.arguments)));
        }

        for directive in &self.directives {
            output.push_str(&format!(" {directive}"));
        }

        if !self.selections.is_empty() {
            output
                .push_str(&Selection::print_multiple(&self.selections, level));
        }

        output
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::graphql::Value,
    };

    #[test]
    fn test_print_field() {
        let field = Field {
            name: "images".to_string(),
            arguments: vec![Argument {
                name: "after".to_string(),
                value: Value::Variable("endCursor".to_string()),
            }],
            directives: vec![],
            selections: vec![],
        };

        assert_eq!(field.print(0), "images(after: $endCursor)".to_string());
    }

    #[test]
    fn test_print_field_with_selections() {
        let field = Field {
            name: "images".to_string(),
            arguments: vec![Argument {
                name: "after".to_string(),
                value: Value::Variable("endCursor".to_string()),
            }],
            directives: vec![],
            selections: vec![Selection::Field(Field {
                name: "edges".to_string(),
                arguments: vec![],
                directives: vec![],
                selections: vec![Selection::Field(Field {
                    name: "node".to_string(),
                    arguments: vec![],
                    directives: vec![],
                    selections: vec![Selection::Field(Field {
                        name: "id".to_string(),
                        arguments: vec![],
                        directives: vec![Directive {
                            name: "id".to_string(),
                            arguments: vec![],
                        }],
                        selections: vec![],
                    })],
                })],
            })],
        };

        assert_eq!(
            field.print(0),
            "\
images(after: $endCursor) {
  edges {
    node {
      id @id
    }
  }
}"
            .to_string()
        );
    }
}
