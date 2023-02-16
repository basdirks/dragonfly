use {
    super::value::Value,
    std::fmt::Display,
};

/// A directive argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argument {
    /// The name of the argument.
    pub name: String,
    /// The value of the argument.
    pub value: Value,
}

impl Display for Argument {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

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
            write!(
                f,
                "({})",
                self.arguments
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
