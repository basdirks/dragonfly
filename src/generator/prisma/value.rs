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

impl Function {
    /// Create a new function.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    /// * `parameters` - The parameters of the function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::Function;
    ///
    /// let function = Function::new("foo", &[]);
    ///
    /// assert_eq!(function.name, "foo");
    /// assert!(function.parameters.is_empty());
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        parameters: &[Value],
    ) -> Self {
        Self {
            name: name.to_owned(),
            parameters: parameters.iter().map(ToOwned::to_owned).collect(),
        }
    }
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

impl Value {
    /// Create an array value.
    ///
    /// # Arguments
    ///
    /// * `values` - The values of the array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::Value;
    ///
    /// let value = Value::array(&[Value::string("foo"), Value::string("bar")]);
    ///
    /// assert_eq!(
    ///     value,
    ///     Value::Array(vec![Value::string("foo"), Value::string("bar")])
    /// );
    /// ```
    #[must_use]
    pub fn array(values: &[Self]) -> Self {
        Self::Array(values.iter().map(ToOwned::to_owned).collect())
    }

    /// Create a string value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the string.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::Value;
    ///
    /// let value = Value::string("foo");
    ///
    /// assert_eq!(value, Value::String("foo".to_owned()));
    /// ```
    #[must_use]
    pub fn string(value: &str) -> Self {
        Self::String(value.to_owned())
    }

    /// Create a function value.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    /// * `parameters` - The parameters of the function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::{
    ///     Function,
    ///     Value,
    /// };
    ///
    /// let value = Value::function("bar", &[Value::string("baz")]);
    ///
    /// assert_eq!(
    ///     value,
    ///     Value::Function(Function::new("bar", &[Value::string("baz")]))
    /// );
    /// ```
    #[must_use]
    pub fn function(
        name: &str,
        parameters: &[Self],
    ) -> Self {
        Self::Function(Function::new(name, parameters))
    }

    /// Create a number value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::Value;
    ///
    /// let value = Value::number("1");
    ///
    /// assert_eq!(value, Value::Number("1".to_owned()));
    /// ```
    #[must_use]
    pub fn number(value: &str) -> Self {
        Self::Number(value.to_owned())
    }

    /// Create a keyword value.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the keyword.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::Value;
    ///
    /// let value = Value::keyword("foo");
    ///
    /// assert_eq!(value, Value::Keyword("foo".to_owned()));
    /// ```
    #[must_use]
    pub fn keyword(value: &str) -> Self {
        Self::Keyword(value.to_owned())
    }
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
            Value::array(&[Value::string("foo"), Value::string("bar")])
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
            Value::Function(Function::new("foo", &[])).to_string(),
            "foo()"
        );
    }

    #[test]
    fn test_display_number() {
        assert_eq!(Value::number("1").to_string(), "1");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Value::string("foo").to_string(), "\"foo\"");
    }

    #[test]
    fn test_display_keyword() {
        assert_eq!(Value::keyword("foo").to_string(), "foo");
    }
}
