use {
    super::{
        value::Const,
        ConstObjectField,
    },
    std::fmt::Display,
};

/// A const argument.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Argument {
    /// The name of the argument.
    pub name: String,
    /// The value of the argument.
    pub value: Const,
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
    ///     Const,
    /// };
    ///
    /// let argument = Argument::new("foo", Const::String("bar".to_owned()));
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Const::String("bar".to_owned()));
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        value: Const,
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
    ///     Const,
    /// };
    ///
    /// let argument = Argument::boolean("foo", true);
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Const::Boolean(true));
    /// ```
    #[must_use]
    pub fn boolean(
        name: &str,
        value: bool,
    ) -> Self {
        Self::new(name, Const::Boolean(value))
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
    ///     Const,
    /// };
    ///
    /// let argument = Argument::r#enum("foo", "Foo.BAR");
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Const::Enum("Foo.BAR".to_owned()));
    /// ```
    #[must_use]
    pub fn r#enum(
        name: &str,
        value: &str,
    ) -> Self {
        Self::new(name, Const::Enum(value.to_owned()))
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
    ///     Const,
    /// };
    ///
    /// let argument = Argument::float("foo", "1.0");
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Const::Float("1.0"));
    /// ```
    #[must_use]
    pub fn float(
        name: &str,
        value: &str,
    ) -> Self {
        Self::new(name, Const::Float(value.to_owned()))
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
    ///     Const,
    /// };
    ///
    /// let argument = Argument::integer("foo", "1");
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Const::int("1"));
    /// ```
    #[must_use]
    pub fn integer(
        name: &str,
        value: &str,
    ) -> Self {
        Self::new(name, Const::int(value))
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
    ///     Const,
    /// };
    ///
    /// let argument = Argument::list("foo", &[Const::string("bar")]);
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Const::List(vec![Const::string("bar")]));
    /// ```
    #[must_use]
    pub fn list(
        name: &str,
        value: &[Const],
    ) -> Self {
        Self::new(name, Const::List(value.to_vec()))
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
    ///     Const,
    ///     ConstObjectField,
    /// };
    ///
    /// let argument = Argument::object(
    ///     "foo",
    ///     &[ConstObjectField::new("bar", Const::string("baz"))],
    /// );
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(
    ///     argument.value,
    ///     Const::Object(vec![ConstObjectField::new("bar", Const::string("baz"))]),
    /// );
    /// ```
    #[must_use]
    pub fn object(
        name: &str,
        value: &[ConstObjectField],
    ) -> Self {
        Self::new(name, Const::object(value))
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
    ///     Const,
    /// };
    ///
    /// let argument = Argument::string("foo", "bar");
    ///
    /// assert_eq!(argument.name, "foo");
    /// assert_eq!(argument.value, Const::string("bar"));
    /// ```
    #[must_use]
    pub fn string(
        name: &str,
        value: &str,
    ) -> Self {
        Self::new(name, Const::string(value))
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
            Argument::new("foo", Const::string("bar"),).to_string(),
            "foo: \"bar\"",
        );
    }
}
