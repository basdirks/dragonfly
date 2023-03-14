use std::{
    borrow::Cow,
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

/// Errors that can occur when type checking a model.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ModelError<'a> {
    /// The name of a model must be unique within the application. This model
    /// has the same name as another model.
    Duplicate,
    /// The name of a field must be unique within the model. This model
    /// contains a field with the same name as another field.
    DuplicateField {
        /// The name of the field.
        field_name: Cow<'a, str>,
    },
    /// A model should contain at least one field, but this model is empty.
    Empty,
    /// The type of a field must be defined within the application.
    /// This model contains a field with an undefined type.
    UnknownFieldType {
        /// The name of the field.
        field_name: Cow<'a, str>,
        /// The type of the field.
        field_type: Cow<'a, str>,
    },
}

impl Display for ModelError<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Duplicate => write!(f, "model already exists"),
            Self::DuplicateField { field_name } => {
                write!(f, "field `{field_name}` already exists")
            }
            Self::Empty => write!(f, "model has no fields"),
            Self::UnknownFieldType {
                field_name,
                field_type,
            } => {
                write!(
                    f,
                    "field `{field_name}` has unknown type `{field_type}`"
                )
            }
        }
    }
}

impl Error for ModelError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_duplicate() {
        assert!(ModelError::Duplicate.source().is_none());
    }

    #[test]
    fn test_source_duplicate_field() {
        assert!(ModelError::DuplicateField {
            field_name: "foo".into(),
        }
        .source()
        .is_none());
    }

    #[test]
    fn test_source_empty() {
        assert!(ModelError::Empty.source().is_none());
    }

    #[test]
    fn test_source_unknown_field_type() {
        assert!(ModelError::UnknownFieldType {
            field_name: "foo".into(),
            field_type: "bar".into(),
        }
        .source()
        .is_none());
    }
}
