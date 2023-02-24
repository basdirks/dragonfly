use {
    super::Value,
    std::fmt::Display,
};

/// A field argument.
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

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::graphql::Value,
    };

    #[test]
    fn test_display() {
        assert_eq!(
            Argument {
                name: "foo".to_owned(),
                value: Value::String("bar".to_owned()),
            }
            .to_string(),
            "foo: \"bar\"",
        );
    }
}
