use {
    super::Value,
    std::fmt::Display,
};
/// An object field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ObjectField {
    /// The name of the field.
    pub name: String,
    /// The value of the field.
    pub value: Value,
}

impl ObjectField {
    /// Create a new object field with a boolean value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let field = ObjectField::boolean("foo", true);
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.value, Value::Boolean(true));
    /// ```
    #[must_use]
    pub fn boolean(
        name: &str,
        value: bool,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::Boolean(value),
        }
    }

    /// Create a new object field with an enum value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let field = ObjectField::r#enum("foo", "bar");
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.value, Value::Enum("bar".to_owned()));
    /// ```
    #[must_use]
    pub fn r#enum(
        name: &str,
        value: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::Enum(value.to_owned()),
        }
    }

    /// Create a new object field with a float value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let field = ObjectField::float("foo", "1.0");
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.value, Value::Float("1.0".to_owned()));
    /// ```
    #[must_use]
    pub fn float(
        name: &str,
        value: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::Float(value.to_owned()),
        }
    }

    /// Create a new object field with an integer value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let field = ObjectField::int("foo", "1");
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.value, Value::Int("1".to_owned()));
    /// ```
    #[must_use]
    pub fn int(
        name: &str,
        value: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::Int(value.to_owned()),
        }
    }

    /// Create a new object field with a list value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let field = ObjectField::list("foo", vec![Value::Int("1".to_owned())]);
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.value, Value::List(vec![Value::Int("1".to_owned())]));
    /// ```
    #[must_use]
    pub fn list(
        name: &str,
        value: &[Value],
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::list(value),
        }
    }

    /// Create a new object field with a null value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let field = ObjectField::null("foo");
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.value, Value::Null);
    /// ```
    #[must_use]
    pub fn null(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::Null,
        }
    }

    /// Create a new object field with an object value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let field = ObjectField::object("foo", vec![ObjectField::int("bar", 1)]);
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.value, Value::Object(vec![ObjectField::int("bar", 1)]));
    /// ```
    #[must_use]
    pub fn object(
        name: &str,
        value: Vec<Self>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::Object(value),
        }
    }

    /// Create a new object field with a string value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let field = ObjectField::string("foo", "bar");
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.value, Value::String("bar".to_owned()));
    /// ```
    #[must_use]
    pub fn string(
        name: &str,
        value: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::String(value.to_owned()),
        }
    }

    /// Create a new object field with a variable value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// let field = ObjectField::variable("foo", "bar");
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.value, Value::Variable("bar".to_owned()));
    /// ```
    #[must_use]
    pub fn variable(
        name: &str,
        value: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::Variable(value.to_owned()),
        }
    }
}

impl Display for ObjectField {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
