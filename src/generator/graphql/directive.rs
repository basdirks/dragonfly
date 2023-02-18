use {
    super::Argument,
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// A directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Directive {
    /// The name of the directive.
    pub name: String,
    /// The arguments of the directive.
    pub arguments: Vec<Argument>,
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
                name: "foo".to_string(),
                arguments: vec![],
            }
            .to_string(),
            "@foo",
        );

        assert_eq!(
            Directive {
                name: "foo".to_string(),
                arguments: vec![
                    Argument {
                        name: "bar".to_string(),
                        value: Value::String("baz".to_string()),
                    },
                    Argument {
                        name: "qux".to_string(),
                        value: Value::String("quux".to_string()),
                    },
                ],
            }
            .to_string(),
            "@foo(bar: \"baz\", qux: \"quux\")",
        );
    }
}
