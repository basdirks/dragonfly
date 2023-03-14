use std::{
    borrow::Cow,
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

/// An error that can occur when working with a Prisma schema.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum SchemaError<'a> {
    /// The name of an enum must be unique within the schema. This enum has
    /// the same name as another enum.
    DuplicateEnum {
        /// The name of the enum.
        enum_name: Cow<'a, str>,
    },
    /// The name of a model must be unique within the schema. This model has
    /// the same name as another model.
    DuplicateModel {
        /// The name of the model.
        model_name: Cow<'a, str>,
    },
    /// The name of a model field must be unique within the model. This model
    /// contains a field with the same name as another field.
    DuplicateModelField {
        /// The name of the model.
        model_name: Cow<'a, str>,
        /// The name of the field.
        field_name: Cow<'a, str>,
    },
    /// A model can only have a relation to an existing model. This model
    /// contains a relation to a model that does not exist.
    UnknownModel {
        /// The name of the model.
        model_name: Cow<'a, str>,
    },
}

impl<'a> SchemaError<'a> {
    /// Create a new `DuplicateEnum` error.
    ///
    /// # Arguments
    ///
    /// * `enum_name` - The name of the enum.
    #[must_use]
    pub fn duplicate_enum<S>(enum_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::DuplicateEnum {
            enum_name: enum_name.into(),
        }
    }

    /// Create a new `DuplicateModel` error.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    #[must_use]
    pub fn duplicate_model<S>(model_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::DuplicateModel {
            model_name: model_name.into(),
        }
    }

    /// Create a new `DuplicateModelField` error.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    /// * `field_name` - The name of the field.
    #[must_use]
    pub fn duplicate_model_field<S, T>(
        model_name: S,
        field_name: T,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::DuplicateModelField {
            model_name: model_name.into(),
            field_name: field_name.into(),
        }
    }

    /// Create a new `UnknownModel` error.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    #[must_use]
    pub fn unknown_model<S>(model_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::UnknownModel {
            model_name: model_name.into(),
        }
    }
}

impl Display for SchemaError<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::DuplicateEnum { enum_name } => {
                write!(f, "enum `{enum_name}` already exists")
            }
            Self::DuplicateModel { model_name } => {
                write!(f, "model `{model_name}` already exists")
            }
            Self::DuplicateModelField {
                model_name,
                field_name,
            } => {
                write!(
                    f,
                    "model `{model_name}` contains duplicate field \
                     `{field_name}`"
                )
            }
            Self::UnknownModel { model_name } => {
                write!(f, "model `{model_name}` does not exist")
            }
        }
    }
}

impl Error for SchemaError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_enum() {
        assert_eq!(
            SchemaError::duplicate_enum("foo").to_string(),
            "enum `foo` already exists"
        );
    }

    #[test]
    fn test_duplicate_model() {
        assert_eq!(
            SchemaError::duplicate_model("foo").to_string(),
            "model `foo` already exists"
        );
    }

    #[test]
    fn test_duplicate_model_field() {
        assert_eq!(
            SchemaError::duplicate_model_field("foo", "bar").to_string(),
            "model `foo` contains duplicate field `bar`"
        );
    }

    #[test]
    fn test_unknown_model() {
        assert_eq!(
            SchemaError::unknown_model("foo").to_string(),
            "model `foo` does not exist"
        );
    }

    #[test]
    fn test_source_duplicate_enum() {
        assert!(SchemaError::duplicate_enum("foo").source().is_none());
    }

    #[test]
    fn test_source_duplicate_model() {
        assert!(SchemaError::duplicate_model("foo").source().is_none());
    }

    #[test]
    fn test_source_duplicate_model_field() {
        assert!(SchemaError::duplicate_model_field("foo", "bar")
            .source()
            .is_none());
    }

    #[test]
    fn test_source_unknown_model() {
        assert!(SchemaError::unknown_model("foo").source().is_none());
    }
}
