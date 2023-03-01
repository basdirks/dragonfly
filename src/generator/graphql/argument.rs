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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::new("foo", Value::string("bar"));
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Value::string("bar"));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::boolean("foo", true);
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Value::Boolean(true));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::r#enum("foo", "Foo.BAR");
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Value::Enum("Foo.BAR".to_owned()));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::float("foo", "1.0");
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Value::Float("1.0"));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::integer("foo", "1");
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Value::int("1"));
    /// ```
    #[must_use]
    pub fn integer(
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::list("foo", &[Value::string("bar")]);
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Value::List(vec![Value::string("bar")]));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     Argument,
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::object(
    ///     "foo",
    ///     &[ObjectField::new("bar", Value::string("baz"))],
    /// );
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(
    ///     argument.value,
    ///     Value::Object(vec![ObjectField::new("bar", Value::string("baz"))]),
    /// );
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::string("foo", "bar");
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Value::string("bar"));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     Argument,
    ///     Value,
    /// };
    ///
    /// let argument = Argument::variable("foo", "bar");
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Value::variable("bar"));
    /// ```
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
    fn test_display() {
        assert_eq!(
            Argument::new("foo", Value::string("bar")).to_string(),
            "foo: \"bar\"",
        );
    }
}
