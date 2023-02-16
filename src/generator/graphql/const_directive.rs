use {
    super::value::Const,
    std::fmt::Display,
};

/// A constant directive argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argument {
    /// The name of the argument.
    pub name: String,
    /// The value of the argument.
    pub value: Const,
}

impl Display for Argument {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

/// A constant directive.
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
            write!(f, "(")?;

            for (i, argument) in self.arguments.iter().enumerate() {
                if i > 0 {
                    write!(f, ", ")?;
                }

                write!(f, "{argument}")?;
            }

            write!(f, ")")?;
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
                        value: Const::String("baz".to_string()),
                    },
                    Argument {
                        name: "qux".to_string(),
                        value: Const::String("quux".to_string()),
                    },
                ],
            }
            .to_string(),
            "@foo(bar: \"baz\", qux: \"quux\")",
        );
    }
}
