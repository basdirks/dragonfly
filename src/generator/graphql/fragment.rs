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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Spread;
    ///
    /// let spread = Spread::new("foo", &[]);
    ///
    /// assert_eq!(spread.name, "foo".to_owned());
    /// assert!(spread.directives.is_empty());
    /// ```
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
    ) -> String {
        let mut output = format!("{}...{}", indent::graphql(level), self.name);

        for directive in &self.directives {
            let _ = write!(output, " {directive}");
        }

        output
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
        },
    };

    #[test]
    fn test_print_spread() {
        let spread = Spread::new("foo", &[]);

        assert_eq!(spread.print(0), "...foo");

        let spread = Spread::new(
            "daboi",
            &[Directive::new("is", &[Argument::string("a", "good boy.")])],
        );

        assert_eq!(spread.print(0), "...daboi @is(a: \"good boy.\")");
    }

    #[test]
    fn test_print_inline() {
        let inline = Inline {
            type_condition: "Foo".to_owned(),
            directives: vec![Directive::new(
                "bar",
                &[Argument::string("baz", "bax")],
            )],
            selections: vec![Selection::Field(Field {
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
        };

        assert_eq!(
            inline.print(1),
            "  ... on Foo @bar(baz: \"bax\") {
    bar @foo(bar: \"baz\") {
      baz
      bax
    }
  }"
        );
    }
}
