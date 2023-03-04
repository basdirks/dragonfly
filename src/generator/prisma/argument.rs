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

impl Argument {
    /// Create an argument without a name.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn unnamed(value: &Value) -> Self {
        Self {
            name: None,
            value: value.clone(),
        }
    }

    /// Create an array argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `values` - The values of the argument.
    #[must_use]
    pub fn array(
        name: &str,
        values: &[Value],
    ) -> Self {
        Self {
            name: Some(name.to_owned()),
            value: Value::Array(values.iter().map(ToOwned::to_owned).collect()),
        }
    }

    /// Create a boolean argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn boolean(
        name: &str,
        value: bool,
    ) -> Self {
        Self {
            name: Some(name.to_owned()),
            value: Value::Boolean(value),
        }
    }

    /// Create a keyword argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn keyword(
        name: &str,
        value: &str,
    ) -> Self {
        Self {
            name: Some(name.to_owned()),
            value: Value::Keyword(value.to_owned()),
        }
    }

    /// Create a function argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `function_name` - The name of the function.
    /// * `arguments` - The arguments of the function.
    #[must_use]
    pub fn function(
        name: &str,
        function_name: &str,
        arguments: &[Value],
    ) -> Self {
        Self {
            name: Some(name.to_owned()),
            value: Value::function(function_name, arguments),
        }
    }

    /// Create a number argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn number(
        name: &str,
        value: &str,
    ) -> Self {
        Self {
            name: Some(name.to_owned()),
            value: Value::Number(value.to_owned()),
        }
    }

    /// Create a string argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn string(
        name: &str,
        value: &str,
    ) -> Self {
        Self {
            name: Some(name.to_owned()),
            value: Value::String(value.to_owned()),
        }
    }
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
    fn test_unnamed() {
        assert_eq!(
            Argument {
                name: None,
                value: Value::String("bar".to_owned()),
            }
            .to_string(),
            "\"bar\""
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(
            Argument::array(
                "foo",
                &[Value::string("bar"), Value::string("baz")],
            )
            .to_string(),
            "foo: [\"bar\", \"baz\"]"
        );
    }

    #[test]
    fn test_boolean() {
        assert_eq!(Argument::boolean("foo", true).to_string(), "foo: true");
    }

    #[test]
    fn test_function() {
        assert_eq!(
            Argument::function(
                "foo",
                "bar",
                &[Value::string("baz"), Value::string("qux")],
            )
            .to_string(),
            "foo: bar(\"baz\", \"qux\")"
        );
    }

    #[test]
    fn test_keyword() {
        assert_eq!(Argument::keyword("foo", "bar").to_string(), "foo: bar");
    }

    #[test]
    fn test_number() {
        assert_eq!(Argument::number("foo", "42").to_string(), "foo: 42");
    }

    #[test]
    fn test_string() {
        assert_eq!(Argument::string("foo", "bar").to_string(), "foo: \"bar\"");
    }

    #[test]
    fn test_display() {
        assert_eq!(Argument::keyword("foo", "bar").to_string(), "foo: bar");

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
