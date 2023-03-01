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

impl Display for ConstObjectField {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.value)
    }
}
