use {
    crate::generator::printer::common::comma_separated,
    std::fmt::Display,
};

/// A constant object field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConstObjectField {
    /// The name of the field.
    pub name: String,
    /// The value of the field.
    pub value: Const,
}

impl Display for ConstObjectField {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

/// A constant value.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Const {
    /// A boolean constant.
    Boolean(bool),
    /// An enum constant.
    Enum(String),
    /// A floating point constant.
    Float(String),
    /// An integer constant.
    Int(String),
    /// A list of constants.
    List(Vec<Const>),
    /// Null.
    Null,
    /// An object constant.
    Object(Vec<ConstObjectField>),
    /// A string constant.
    String(String),
}

impl Display for Const {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Enum(value) | Self::Float(value) | Self::Int(value) => {
                write!(f, "{value}")
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

/// An object field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectField {
    /// The name of the field.
    pub name: String,
    /// The value of the field.
    pub value: Value,
}

impl Display for ObjectField {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}

/// A value.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    fn test_display_value_boolean() {
        assert_eq!(Value::Boolean(true).to_string(), "true");
        assert_eq!(Value::Boolean(false).to_string(), "false");
    }

    #[test]
    fn test_display_value_enum() {
        assert_eq!(Value::Enum("Foo.BAR".to_string()).to_string(), "Foo.BAR");
    }

    #[test]
    fn test_display_value_float() {
        assert_eq!(Value::Float("1.0".to_string()).to_string(), "1.0");
    }

    #[test]
    fn test_display_value_int() {
        assert_eq!(Value::Int("1".to_string()).to_string(), "1");
    }

    #[test]
    fn test_display_value_list() {
        assert_eq!(
            Value::List(vec![
                Value::Int("1".to_string()),
                Value::Int("2".to_string()),
                Value::Int("3".to_string()),
            ])
            .to_string(),
            "[1, 2, 3]"
        );
    }

    #[test]
    fn test_display_value_null() {
        assert_eq!(Value::Null.to_string(), "null");
    }

    #[test]
    fn test_display_value_object() {
        assert_eq!(
            Value::Object(vec![
                ObjectField {
                    name: "foo".to_string(),
                    value: Value::Int("1".to_string()),
                },
                ObjectField {
                    name: "bar".to_string(),
                    value: Value::Int("2".to_string()),
                },
                ObjectField {
                    name: "baz".to_string(),
                    value: Value::Int("3".to_string()),
                },
            ])
            .to_string(),
            "{foo: 1, bar: 2, baz: 3}"
        );
    }

    #[test]
    fn test_display_value_string() {
        assert_eq!(Value::String("foo".to_string()).to_string(), "\"foo\"");
    }

    #[test]
    fn test_display_value_variable() {
        assert_eq!(Value::Variable("foo".to_string()).to_string(), "$foo");
    }

    #[test]
    fn test_display_const_boolean() {
        assert_eq!(Const::Boolean(true).to_string(), "true");
        assert_eq!(Const::Boolean(false).to_string(), "false");
    }

    #[test]
    fn test_display_const_enum() {
        assert_eq!(Const::Enum("Foo.BAR".to_string()).to_string(), "Foo.BAR");
    }

    #[test]
    fn test_display_const_float() {
        assert_eq!(Const::Float("1.0".to_string()).to_string(), "1.0");
    }

    #[test]
    fn test_display_const_int() {
        assert_eq!(Const::Int("1".to_string()).to_string(), "1");
    }

    #[test]
    fn test_display_const_list() {
        assert_eq!(
            Const::List(vec![
                Const::Int("1".to_string()),
                Const::Int("2".to_string()),
                Const::Int("3".to_string()),
            ])
            .to_string(),
            "[1, 2, 3]"
        );
    }

    #[test]
    fn test_display_const_null() {
        assert_eq!(Const::Null.to_string(), "null");
    }

    #[test]
    fn test_display_const_object() {
        assert_eq!(
            Const::Object(vec![
                ConstObjectField {
                    name: "foo".to_string(),
                    value: Const::Int("1".to_string()),
                },
                ConstObjectField {
                    name: "bar".to_string(),
                    value: Const::Int("2".to_string()),
                },
                ConstObjectField {
                    name: "baz".to_string(),
                    value: Const::Int("3".to_string()),
                },
            ])
            .to_string(),
            "{foo: 1, bar: 2, baz: 3}"
        );
    }

    #[test]
    fn test_display_const_string() {
        assert_eq!(Const::String("foo".to_string()).to_string(), "\"foo\"");
    }
}
