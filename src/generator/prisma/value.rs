use {
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// A function.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Function {
    /// The name of the function.
    pub name: String,
    /// The parameters of the function.
    pub parameters: Vec<Value>,
}

impl Display for Function {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}({})", self.name, comma_separated(&self.parameters))
    }
}

/// A value.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Value {
    /// An array of values.
    Array(Vec<Value>),
    /// A boolean value.
    Boolean(bool),
    /// A keyword.
    Keyword(String),
    /// A function.
    Function(Function),
    /// A number.
    Number(String),
    /// A string.
    String(String),
}

impl Display for Value {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Array(values) => write!(f, "[{}]", comma_separated(values)),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Function(function) => write!(f, "{function}"),
            Self::Number(value) | Self::Keyword(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "\"{value}\""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_array() {
        assert_eq!(
            Value::Array(vec![
                Value::String("foo".to_owned()),
                Value::String("bar".to_owned()),
            ])
            .to_string(),
            "[\"foo\", \"bar\"]"
        );
    }

    #[test]
    fn test_display_boolean() {
        assert_eq!(Value::Boolean(true).to_string(), "true");
    }

    #[test]
    fn test_display_function() {
        assert_eq!(
            Value::Function(Function {
                name: "foo".to_owned(),
                parameters: vec![],
            })
            .to_string(),
            "foo()"
        );
    }

    #[test]
    fn test_display_number() {
        assert_eq!(Value::Number("1".to_owned()).to_string(), "1");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Value::String("foo".to_owned()).to_string(), "\"foo\"");
    }

    #[test]
    fn test_display_keyword() {
        assert_eq!(Value::Keyword("foo".to_owned()).to_string(), "foo");
    }
}
