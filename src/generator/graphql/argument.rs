use {
    super::{
        value::Value,
        ObjectField,
    },
    std::fmt::Display,
};

/// A field argument.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Argument {
    /// The name of the argument.
    pub name: String,
    /// The value of the argument.
    pub value: Value,
}

impl Argument {
    /// Create a new argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn new(
        name: &str,
        value: Value,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value,
        }
    }

    /// Create a new argument with a boolean value.
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
        Self::new(name, Value::Boolean(value))
    }

    /// Create a new argument with an enum value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn r#enum(
        name: &str,
        value: &str,
    ) -> Self {
        Self::new(name, Value::Enum(value.to_owned()))
    }

    /// Create a new argument with a float value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn float(
        name: &str,
        value: &str,
    ) -> Self {
        Self::new(name, Value::Float(value.to_owned()))
    }

    /// Create a new argument with an integer value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn int(
        name: &str,
        value: &str,
    ) -> Self {
        Self::new(name, Value::int(value))
    }

    /// Create a new argument with a list value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn list(
        name: &str,
        value: &[Value],
    ) -> Self {
        Self::new(name, Value::List(value.to_vec()))
    }

    /// Create a new argument with an object value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn object(
        name: &str,
        value: &[ObjectField],
    ) -> Self {
        Self::new(name, Value::object(value))
    }

    /// Create a new argument with a string value.
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
        Self::new(name, Value::string(value))
    }

    /// Create a new argument with a variable value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `value` - The value of the argument.
    #[must_use]
    pub fn variable(
        name: &str,
        value: &str,
    ) -> Self {
        Self::new(name, Value::variable(value))
    }
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
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Argument::new("foo", Value::string("bar")).to_string(),
            "foo: \"bar\""
        );
    }

    #[test]
    fn test_boolean() {
        assert_eq!(Argument::boolean("foo", true).to_string(), "foo: true");
    }

    #[test]
    fn test_enum() {
        assert_eq!(Argument::r#enum("foo", "bar").to_string(), "foo: bar");
    }

    #[test]
    fn test_float() {
        assert_eq!(Argument::float("foo", "1.23").to_string(), "foo: 1.23");
    }

    #[test]
    fn test_int() {
        assert_eq!(Argument::int("foo", "123").to_string(), "foo: 123");
    }

    #[test]
    fn test_list() {
        assert_eq!(
            Argument::list(
                "foo",
                &[Value::string("bar"), Value::string("baz")]
            )
            .to_string(),
            "foo: [\"bar\", \"baz\"]"
        );
    }

    #[test]
    fn test_object() {
        assert_eq!(
            Argument::object(
                "foo",
                &[
                    ObjectField::string("bar", "baz"),
                    ObjectField::list(
                        "qux",
                        &[Value::string("baz"), Value::string("qux")]
                    )
                ],
            )
            .to_string(),
            "foo: {bar: \"baz\", qux: [\"baz\", \"qux\"]}"
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(Argument::string("foo", "bar").to_string(), "foo: \"bar\"");
    }

    #[test]
    fn test_variable() {
        assert_eq!(Argument::variable("foo", "bar").to_string(), "foo: $bar");
    }
}
