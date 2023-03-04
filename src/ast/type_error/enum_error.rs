use std::fmt::{
    self,
    Display,
    Formatter,
};

/// Errors that can occur when type checking an enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EnumError {
    /// The name of an enum must be unique within the application. This enum
    /// has the same name as another enum.
    Duplicate,
    /// The name of an enum variant must be unique within the enum. This enum
    /// contains a variant with the same name as another variant.
    DuplicateVariant(String),
    /// An enum should contain at least one variant, but this enum is empty.
    Empty,
}

impl Display for EnumError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Duplicate => write!(f, "duplicate enum"),
            Self::DuplicateVariant(name) => {
                write!(f, "duplicate variant \"{name}\"")
            }
            Self::Empty => write!(f, "empty enum"),
        }
    }
}
