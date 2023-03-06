use {
    super::{
        Directive,
        Selection,
    },
    crate::generator::printer::{
        indent,
        Print,
    },
    std::{
        self,
        io,
    },
};

/// A fragment spread.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Spread {
    /// The name of the fragment.
    pub name: String,
    /// The directives of the fragment.
    pub directives: Vec<Directive>,
}

impl Spread {
    /// Create a new fragment spread.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the fragment.
    /// * `directives` - The directives of the fragment.
    #[must_use]
    pub fn new(
        name: &str,
        directives: &[Directive],
    ) -> Self {
        Self {
            name: name.to_owned(),
            directives: directives.iter().map(ToOwned::to_owned).collect(),
        }
    }
}

impl Print for Spread {
    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}...{}", indent::graphql(level), self.name)?;

        for directive in &self.directives {
            write!(f, " {directive}")?;
        }

        writeln!(f)
    }
}

/// An inline fragment.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Inline {
    /// The type condition of the fragment.
    pub type_condition: String,
    /// The directives of the fragment.
    pub directives: Vec<Directive>,
    /// The selection set of the fragment.
    pub selections: Vec<Selection>,
}

impl Inline {
    /// Create a new inline fragment.
    ///
    /// # Arguments
    ///
    /// * `type_condition` - The type condition of the fragment.
    /// * `directives` - The directives of the fragment.
    /// * `selections` - The selection set of the fragment.
    #[must_use]
    pub fn new(
        type_condition: &str,
        directives: &[Directive],
        selections: &[Selection],
    ) -> Self {
        Self {
            type_condition: type_condition.to_owned(),
            directives: directives.iter().map(ToOwned::to_owned).collect(),
            selections: selections.iter().map(ToOwned::to_owned).collect(),
        }
    }
}

impl Print for Inline {
    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(
            f,
            "{}... on {}",
            indent::graphql(level),
            self.type_condition
        )?;

        for directive in &self.directives {
            write!(f, " {directive}")?;
        }

        Selection::print_multiple(&self.selections, level, f)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::graphql::{
            Argument,
            Field,
        },
    };

    #[test]
    fn test_spread() {
        let spread = Spread::new("foo", &[]);
        let mut f = Vec::new();

        spread.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "...foo\n");

        let spread = Spread::new(
            "daboi",
            &[Directive::new("is", &[Argument::string("a", "good boy.")])],
        );

        let mut f = Vec::new();

        spread.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "...daboi @is(a: \"good boy.\")\n"
        );
    }

    #[test]
    fn test_inline() {
        let inline = Inline::new(
            "Foo",
            &[Directive::new("bar", &[Argument::string("baz", "bax")])],
            &[Selection::Field(Field {
                name: "bar".to_owned(),
                arguments: vec![],
                directives: vec![Directive::new(
                    "foo",
                    &[Argument::string("bar", "baz")],
                )],
                selections: vec![
                    Selection::Field(Field::new("baz")),
                    Selection::Field(Field::new("bax")),
                ],
            })],
        );

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
