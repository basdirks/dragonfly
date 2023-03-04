use {
    crate::ast::Field,
    std::fmt::{
        self,
        Display,
        Formatter,
    },
};

/// Errors that can occur when type checking a model.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ModelError {
    /// The name of a model must be unique within the application. This model
    /// has the same name as another model.
    Duplicate,
    /// The name of a field must be unique within the model. This model
    /// contains a field with the same name as another field.
    DuplicateField(String),
    /// A model should contain at least one field, but this model is empty.
    Empty,
    /// The type of a field must be defined within the application.
    /// This model contains a field with an undefined type.
    UnknownFieldType(Field),
}

impl Display for ModelError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Duplicate => write!(f, "duplicate model"),
            Self::DuplicateField(name) => {
                write!(f, "duplicate field \"{name}\"")
            }
            Self::Empty => write!(f, "empty model"),
            Self::UnknownFieldType(Field { name, r#type }) => {
                write!(f, "field \"{name}\" has unknown type \"{type}\"")
            }
        }
    }
}
