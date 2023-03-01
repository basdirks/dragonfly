use {
    super::ConstArgument,
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// A constant directive.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Directive {
    /// The name of the directive.
    pub name: String,
    /// The arguments of the directive.
    pub arguments: Vec<ConstArgument>,
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
    /// use dragonfly::generator::graphql::ConstDirective;
    ///
    /// let directive = ConstDirective::new("foo", &[]);
    ///
    /// assert_eq!(directive.name, "foo");
    /// assert!(directive.arguments.is_empty());
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        arguments: &[ConstArgument],
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
        crate::generator::graphql::ConstArgument,
    };

    #[test]
    fn test_display() {
        assert_eq!(Directive::new("foo", &[]).to_string(), "@foo",);

        assert_eq!(
            Directive::new(
                "foo",
                &[
                    ConstArgument::string("bar", "baz"),
                    ConstArgument::string("qux", "quux"),
                ],
            )
            .to_string(),
            "@foo(bar: \"baz\", qux: \"quux\")",
        );
    }
}
