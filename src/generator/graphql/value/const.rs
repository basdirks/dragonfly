use {
    super::const_object_field::ConstObjectField,
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// A constant value.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

impl Const {
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
    pub fn object(fields: &[ConstObjectField]) -> Self {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_boolean() {
        assert_eq!(Const::Boolean(true).to_string(), "true");
    }

    #[test]
    fn test_display_enum() {
        assert_eq!(Const::r#enum("Foo.BAR").to_string(), "Foo.BAR");
    }

    #[test]
    fn test_display_float() {
        assert_eq!(Const::float("1.0").to_string(), "1.0");
    }

    #[test]
    fn test_display_int() {
        assert_eq!(Const::int("1").to_string(), "1");
    }

    #[test]
    fn test_display_list() {
        assert_eq!(
            Const::list(&[Const::int("1"), Const::int("2")]).to_string(),
            "[1, 2]"
        );
    }

    #[test]
    fn test_display_null() {
        assert_eq!(Const::Null.to_string(), "null");
    }

    #[test]
    fn test_display_object() {
        assert_eq!(
            Const::object(&[ConstObjectField::new("foo", Const::int("1"))])
                .to_string(),
            "{foo: 1}"
        );
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Const::string("foo").to_string(), "\"foo\"");
    }
}
