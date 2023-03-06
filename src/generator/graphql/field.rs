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
    std::{
        self,
        io,
    },
};

/// A selection field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

impl Field {
    /// Create a new field with just a name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            arguments: Vec::new(),
            directives: Vec::new(),
            selections: Vec::new(),
        }
    }
}

impl Print for Field {
    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}{}", indent::graphql(level), self.name)?;

        if !self.arguments.is_empty() {
            write!(f, "({})", comma_separated(&self.arguments))?;
        }

        for directive in &self.directives {
            write!(f, " {directive}")?;
        }

        if self.selections.is_empty() {
            writeln!(f)
        } else {
            Selection::print_multiple(&self.selections, level, f)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let field = Field::new("images");

        assert_eq!(field.name, "images");
        assert!(field.arguments.is_empty());
        assert!(field.directives.is_empty());
        assert!(field.selections.is_empty());
    }

    #[test]
    fn test_print() {
        let field = Field {
            name: "images".to_owned(),
            arguments: vec![Argument::variable("after", "endCursor")],
            directives: vec![],
            selections: vec![Selection::Field(Field {
                name: "edges".to_owned(),
                arguments: vec![],
                directives: vec![],
                selections: vec![Selection::Field(Field {
                    name: "node".to_owned(),
                    arguments: vec![],
                    directives: vec![],
                    selections: vec![Selection::Field(Field {
                        name: "id".to_owned(),
                        arguments: vec![],
                        directives: vec![Directive::new("id", &[])],
                        selections: vec![],
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
