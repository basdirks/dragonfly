use {
    super::Value,
    std::fmt::Display,
};

/// A model attribute argument.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Argument {
    /// The name of the argument.
    pub name: Option<String>,
    /// The value of the argument.
    pub value: Value,
}

impl Display for Argument {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self { name, value } = self;

        match &name {
            Some(name) => {
                write!(f, "{name}: {value}")
            }
            None => write!(f, "{value}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::prisma::Function,
    };

    #[test]
    fn test_display_argument() {
        assert_eq!(
            Argument {
                name: Some("foo".to_owned()),
                value: Value::Keyword("bar".to_owned()),
            }
            .to_string(),
            "foo: bar"
        );

        assert_eq!(
            Argument {
                name: None,
                value: Value::String("bar".to_owned()),
            }
            .to_string(),
            "\"bar\""
        );

        assert_eq!(
            Argument {
                name: Some("foo".to_owned()),
                value: Value::Function(Function {
                    name: "bar".to_owned(),
                    parameters: vec![],
                }),
            }
            .to_string(),
            "foo: bar()"
        );
    }
}
