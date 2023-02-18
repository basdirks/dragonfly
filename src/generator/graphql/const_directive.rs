use {
    super::ConstArgument,
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// A constant directive.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Directive {
    /// The name of the directive.
    pub name: String,
    /// The arguments of the directive.
    pub arguments: Vec<ConstArgument>,
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
        crate::generator::graphql::{
            ConstArgument,
            ConstValue,
        },
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
                    ConstArgument {
                        name: "bar".to_string(),
                        value: ConstValue::String("baz".to_string()),
                    },
                    ConstArgument {
                        name: "qux".to_string(),
                        value: ConstValue::String("quux".to_string()),
                    },
                ],
            }
            .to_string(),
            "@foo(bar: \"baz\", qux: \"quux\")",
        );
    }
}
