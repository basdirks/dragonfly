use {
    crate::generator::prisma::{
        value::Function,
        Value,
    },
    std::fmt::Display,
};

/// A field type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FieldType {
    /// A name.
    Name(String),
    /// A function.
    Function(Function),
}

impl FieldType {
    /// Create a new `Name` field type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field type.
    #[must_use]
    pub fn name(name: &str) -> Self {
        Self::Name(name.to_owned())
    }

    /// Create a new `Function` field type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    /// * `parameters` - The parameters of the function.
    #[must_use]
    pub fn function(
        name: &str,
        parameters: &[Value],
    ) -> Self {
        Self::Function(Function::new(name, parameters))
    }
}

impl Display for FieldType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Name(name) => write!(f, "{name}"),
            Self::Function(function) => write!(f, "{function}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        assert_eq!(FieldType::name("foo").to_string(), "foo");
    }

    #[test]
    fn test_function() {
        assert_eq!(
            FieldType::function(
                "foo",
                &[Value::string("bar"), Value::string("baz")]
            )
            .to_string(),
            "foo(\"bar\", \"baz\")"
        );
    }
}
