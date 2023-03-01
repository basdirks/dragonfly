use {
    super::Argument,
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// A directive.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Directive {
    /// The name of the directive.
    pub name: String,
    /// The arguments of the directive.
    pub arguments: Vec<Argument>,
}

impl Directive {
    /// Create a new directive.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the directive.
    /// * `arguments` - The arguments of the directive.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Directive;
    ///
    /// let directive = Directive::new("foo", &[]);
    ///
    /// assert_eq!(directive.name, "foo");
    /// assert!(directive.arguments.is_empty());
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        arguments: &[Argument],
    ) -> Self {
        Self {
            name: name.to_owned(),
            arguments: arguments.iter().map(ToOwned::to_owned).collect(),
        }
    }
}

impl Display for Directive {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "@{}", self.name)?;

        if !self.arguments.is_empty() {
            write!(f, "({})", comma_separated(&self.arguments))?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::graphql::Value,
    };

    #[test]
    fn test_display() {
        assert_eq!(
            Directive {
                name: "foo".to_owned(),
                arguments: vec![],
            }
            .to_string(),
            "@foo",
        );

        assert_eq!(
            Directive {
                name: "foo".to_owned(),
                arguments: vec![
                    Argument {
                        name: "bar".to_owned(),
                        value: Value::String("baz".to_owned()),
                    },
                    Argument {
                        name: "qux".to_owned(),
                        value: Value::String("quux".to_owned()),
                    },
                ],
            }
            .to_string(),
            "@foo(bar: \"baz\", qux: \"quux\")",
        );
    }
}
