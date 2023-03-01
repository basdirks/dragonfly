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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::{
    ///     argument::Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::unnamed("foo");
    ///
    /// assert!(argument.name.is_none());
    /// assert_eq!(argument.value, Value::string("foo"));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::{
    ///     argument::Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::array("foo", &["bar", "baz"]);
    ///
    /// assert_eq!(argument.name, Some("foo".to_owned()));
    /// assert_eq!(argument.value, Value::array(&["bar", "baz"]));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::{
    ///     argument::Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::boolean("foo", true);
    ///
    /// assert_eq!(argument.name, Some("foo".to_owned()));
    /// assert_eq!(argument.value, Value::Boolean(true));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::{
    ///     argument::Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::keyword("foo", "bar");
    ///
    /// assert_eq!(argument.name, Some("foo".to_owned()));
    /// assert_eq!(argument.value, Value::keyword("bar"));
    /// ```
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

    /// Create a number argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::{
    ///     argument::Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::number("foo", "42");
    ///
    /// assert_eq!(argument.name, Some("foo".to_owned()));
    /// assert_eq!(argument.value, Value::number("42"));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::{
    ///     argument::Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::string("foo", "bar");
    ///
    /// assert_eq!(argument.name, Some("foo".to_owned()));
    /// assert_eq!(argument.value, Value::string("bar"));
    /// ```
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
    fn test_display_argument() {
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
