pub use self::{
    const_object_field::ConstObjectField,
    object_field::ObjectField,
    r#const::Const,
};
use {
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// Const values.
pub mod r#const;
/// Const object fields.
pub mod const_object_field;
/// Object fields.
pub mod object_field;

/// A constant directive argument.
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
}

impl Display for Argument {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

/// A value.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Value {
    /// A boolean value.
    Boolean(bool),
    /// An enum value.
    Enum(String),
    /// A floating point value.
    Float(String),
    /// An integer value.
    Int(String),
    /// A list of values.
    List(Vec<Value>),
    /// Null.
    Null,
    /// An object of fields.
    Object(Vec<ObjectField>),
    /// A string value.
    String(String),
    /// A variable.
    Variable(String),
}

impl Value {
    /// Create a new enum value.
    ///
    /// # Arguments
    ///
    /// * `name` - The enum name.
    #[must_use]
    pub fn r#enum(name: &str) -> Self {
        Self::Enum(name.to_owned())
    }

    /// Create a new float value.
    ///
    /// # Arguments
    ///
    /// * `value` - The float value.
    #[must_use]
    pub fn float(value: &str) -> Self {
        Self::Float(value.to_owned())
    }

    /// Create a new int value.
    ///
    /// # Arguments
    ///
    /// * `value` - The int value.
    #[must_use]
    pub fn int(value: &str) -> Self {
        Self::Int(value.to_owned())
    }

    // list object string variable

    /// Create a new list value.
    ///
    /// # Arguments
    ///
    /// * `values` - The list of values.
    #[must_use]
    pub fn list(values: &[Self]) -> Self {
        Self::List(values.to_owned())
    }

    /// Create a new object value.
    ///
    /// # Arguments
    ///
    /// * `fields` - The list of fields.
    #[must_use]
    pub fn object(fields: &[ObjectField]) -> Self {
        Self::Object(fields.to_owned())
    }

    /// Create a new string value.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value.
    #[must_use]
    pub fn string(value: &str) -> Self {
        Self::String(value.to_owned())
    }

    /// Create a new variable value.
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name.
    #[must_use]
    pub fn variable(name: &str) -> Self {
        Self::Variable(name.to_owned())
    }
}

impl Display for Value {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Enum(value) | Self::Float(value) | Self::Int(value) => {
                write!(f, "{value}")
            }
            Self::Variable(value) => {
                write!(f, "${value}")
            }
            Self::List(values) => {
                write!(f, "[{}]", comma_separated(values))
            }
            Self::Null => write!(f, "null"),
            Self::Object(fields) => {
                write!(f, "{{{}}}", comma_separated(fields))
            }
            Self::String(value) => write!(f, "\"{value}\""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_argument() {
        assert_eq!(Argument::new("foo", Const::int("1")).to_string(), "foo: 1");
    }

    #[test]
    fn test_enum() {
        assert_eq!(Value::r#enum("Foo.BAR").to_string(), "Foo.BAR");
    }

    #[test]
    fn test_float() {
        assert_eq!(Value::float("1.0").to_string(), "1.0");
    }

    #[test]
    fn test_int() {
        assert_eq!(Value::int("1").to_string(), "1");
    }

    #[test]
    fn test_list() {
        assert_eq!(
            Value::list(&[Value::int("1"), Value::int("2")]).to_string(),
            "[1, 2]".to_owned()
        );
    }

    #[test]
    fn test_object() {
        assert_eq!(
            Value::object(&[
                ObjectField::int("foo", "1"),
                ObjectField::string("bar", "barrr")
            ])
            .to_string(),
            "{foo: 1, bar: \"barrr\"}".to_owned()
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(Value::string("foo").to_string(), "\"foo\"".to_owned());
    }

    #[test]
    fn test_variable() {
        assert_eq!(Value::variable("foo").to_string(), "$foo".to_owned());
    }
}
