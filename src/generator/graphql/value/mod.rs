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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Value;
    ///
    /// assert_eq!(Value::r#enum("Foo.BAR"), Value::Enum("Foo.BAR".to_owned()));
    /// ```
    #[must_use]
    pub fn r#enum(name: &str) -> Self {
        Self::Enum(name.to_owned())
    }

    /// Create a new float value.
    ///
    /// # Arguments
    ///
    /// * `value` - The float value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Value;
    ///
    /// assert_eq!(Value::float("1.0"), Value::Float("1.0".to_owned()));
    /// ```
    #[must_use]
    pub fn float(value: &str) -> Self {
        Self::Float(value.to_owned())
    }

    /// Create a new int value.
    ///
    /// # Arguments
    ///
    /// * `value` - The int value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Value;
    ///
    /// assert_eq!(Value::int("1"), Value::Int("1".to_owned()));
    /// ```
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Value;
    ///
    /// assert_eq!(
    ///     Value::list(&[Value::int("1"), Value::int("2")]),
    ///     Value::List(&[Value::int("1"), Value::int("2")])
    /// );
    /// ```
    #[must_use]
    pub fn list(values: &[Self]) -> Self {
        Self::List(values.to_owned())
    }

    /// Create a new object value.
    ///
    /// # Arguments
    ///
    /// * `fields` - The list of fields.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::{
    ///     ObjectField,
    ///     Value,
    /// };
    ///
    /// assert_eq!(
    ///     Value::object(&[ObjectField::new("foo", Value::int("1"))]),
    ///     Value::Object(&[ObjectField::new("foo", Value::int("1"))])
    /// );
    /// ```
    #[must_use]
    pub fn object(fields: &[ObjectField]) -> Self {
        Self::Object(fields.to_owned())
    }

    /// Create a new string value.
    ///
    /// # Arguments
    ///
    /// * `value` - The string value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Value;
    ///
    /// assert_eq!(Value::string("foo"), Value::String("foo".to_owned()));
    /// ```
    #[must_use]
    pub fn string(value: &str) -> Self {
        Self::String(value.to_owned())
    }

    /// Create a new variable value.
    ///
    /// # Arguments
    ///
    /// * `name` - The variable name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Value;
    ///
    /// assert_eq!(Value::variable("foo"), Value::Variable("foo".to_owned()));
    /// ```
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
