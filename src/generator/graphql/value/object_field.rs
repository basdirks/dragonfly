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
    #[must_use]
    pub fn object(
        name: &str,
        fields: &[Self],
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: Value::Object(
                fields.iter().map(ToOwned::to_owned).collect(),
            ),
        }
    }

    /// Create a new object field with a string value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean() {
        assert_eq!(ObjectField::boolean("foo", true).to_string(), "foo: true");
    }

    #[test]
    fn test_enum() {
        assert_eq!(ObjectField::r#enum("foo", "bar").to_string(), "foo: bar");
    }

    #[test]
    fn test_float() {
        assert_eq!(ObjectField::float("foo", "1.23").to_string(), "foo: 1.23");
    }

    #[test]
    fn test_int() {
        assert_eq!(ObjectField::int("foo", "123").to_string(), "foo: 123");
    }

    #[test]
    fn test_list() {
        assert_eq!(
            ObjectField::list("foo", &[Value::Int("123".to_owned())])
                .to_string(),
            "foo: [123]"
        );
    }

    #[test]
    fn test_null() {
        assert_eq!(ObjectField::null("foo").to_string(), "foo: null");
    }

    #[test]
    fn test_object() {
        assert_eq!(
            ObjectField::object("foo", &[ObjectField::int("bar", "123")])
                .to_string(),
            "foo: {bar: 123}"
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(
            ObjectField::string("foo", "bar").to_string(),
            "foo: \"bar\""
        );
    }

    #[test]
    fn test_variable() {
        assert_eq!(
            ObjectField::variable("foo", "bar").to_string(),
            "foo: $bar"
        );
    }
}
