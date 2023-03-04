use {
    super::Const,
    std::fmt::Display,
};
/// A constant object field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ConstObjectField {
    /// The name of the field.
    pub name: String,
    /// The value of the field.
    pub value: Const,
}

impl ConstObjectField {
    /// Create a new constant object field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `value` - The value of the field.
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

impl Display for ConstObjectField {
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
    fn test_new() {
        let field = ConstObjectField::new("foo", Const::string("bar"));

        assert_eq!(field.name, "foo".to_owned());
        assert_eq!(field.value, Const::string("bar"));
    }

    #[test]
    fn test_display() {
        let field = ConstObjectField::new("foo", Const::string("bar"));

        assert_eq!(field.to_string(), "foo: \"bar\"");
    }
}
