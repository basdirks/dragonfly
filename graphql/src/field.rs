use {
    super::{
        Argument,
        Directive,
        Selection,
    },
    printer::{
        Print,
        PrintInline,
    },
    std::{
        self,
        borrow::Cow,
        io,
    },
};

/// A selection field.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Field<'a> {
    /// The name of the field.
    pub name: Cow<'a, str>,
    /// The arguments of the field.
    pub arguments: Vec<Argument<'a>>,
    /// The directives of the field.
    pub directives: Vec<Directive<'a>>,
    /// The selections of the field.
    pub selections: Vec<Selection<'a>>,
}

impl Print for Field<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}{}", Self::indent(level), self.name)?;

        if !self.arguments.is_empty() {
            write!(f, "(")?;
            PrintInline::intercalate(self.arguments.clone(), f, ", ")?;
            write!(f, ")")?;
        }

        for directive in &self.directives {
            directive.print(f)?;
        }

        if self.selections.is_empty() {
            writeln!(f)
        } else {
            Selection::print_multiple(self.selections.iter(), level, f)
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Value,
    };

    #[test]
    fn test_print() {
        let field = Field {
            name: "images".into(),
            arguments: vec![Argument {
                name: "after".into(),
                value: Value::Variable("endCursor".into()),
            }],
            directives: Vec::new(),
            selections: vec![Selection::Field(Field {
                name: "edges".into(),
                arguments: Vec::new(),
                directives: Vec::new(),
                selections: vec![Selection::Field(Field {
                    name: "node".into(),
                    arguments: Vec::new(),
                    directives: Vec::new(),
                    selections: vec![Selection::Field(Field {
                        name: "id".into(),
                        arguments: Vec::new(),
                        directives: vec![Directive {
                            name: "id".into(),
                            arguments: Vec::new(),
                        }],
                        selections: Vec::new(),
                    })],
                })],
            })],
        };

        let mut f = Vec::new();

        field.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "images(after: $endCursor) {
  edges {
    node {
      id @id
    }
  }
}
"
        );
    }
}
