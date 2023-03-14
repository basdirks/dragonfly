use {
    enum_error::EnumError,
    model_error::ModelError,
    query_error::QueryError,
    std::{
        borrow::Cow,
        error::Error,
        fmt::{
            self,
            Display,
            Formatter,
        },
    },
};

/// Enum errors.
pub mod enum_error;
/// Model errors.
pub mod model_error;
/// Query errors.
pub mod query_error;

/// Type checking errors.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum TypeError<'a> {
    /// An enum error.
    EnumError {
        /// The name of the enum.
        enum_name: Cow<'a, str>,
        /// The error.
        error: EnumError,
    },
    /// A model error.
    ModelError {
        /// The name of the model.
        model_name: Cow<'a, str>,
        /// The error.
        error: ModelError<'a>,
    },
    /// A query error.
    QueryError {
        /// The name of the query.
        query_name: Cow<'a, str>,
        /// The error.
        error: QueryError<'a>,
    },
}

impl<'a> TypeError<'a> {
    /// Create a duplicate enum error.
    ///
    /// # Arguments
    ///
    /// * `enum_name` - The name of the enum.
    #[must_use]
    pub fn duplicate_enum<S>(enum_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::EnumError {
            enum_name: enum_name.into(),
            error: EnumError::Duplicate,
        }
    }

    /// Create a duplicate model error.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    #[must_use]
    pub fn duplicate_model<S>(model_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::ModelError {
            model_name: model_name.into(),
            error: ModelError::Duplicate,
        }
    }

    /// Create a duplicate model field error.
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
        Self::ModelError {
            model_name: model_name.into(),
            error: ModelError::DuplicateField {
                field_name: field_name.into(),
            },
        }
    }

    /// Create an empty model error.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    #[must_use]
    pub fn empty_model<S>(model_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::ModelError {
            model_name: model_name.into(),
            error: ModelError::Empty,
        }
    }

    /// Create an unknown model field error.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    /// * `field` - The field.
    #[must_use]
    pub fn unknown_model_field_type<S, T, U>(
        model_name: S,
        field_name: T,
        field_type: U,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        Self::ModelError {
            model_name: model_name.into(),
            error: ModelError::UnknownFieldType {
                field_name: field_name.into(),
                field_type: field_type.into(),
            },
        }
    }

    /// Create a duplicate query error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    #[must_use]
    pub fn duplicate_query<S>(query_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::QueryError {
            query_name: query_name.into(),
            error: QueryError::Duplicate,
        }
    }

    /// Create an empty query schema error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    #[must_use]
    pub fn empty_query_schema<S>(query_name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self::QueryError {
            query_name: query_name.into(),
            error: QueryError::EmptySchema,
        }
    }

    /// Create an invalid query argument type error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `argument` - The argument.
    #[must_use]
    pub fn invalid_query_argument_type<S, T, U>(
        query_name: S,
        argument_name: T,
        argument_type: U,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        Self::QueryError {
            query_name: query_name.into(),
            error: QueryError::InvalidArgumentType {
                argument_name: argument_name.into(),
                argument_type: argument_type.into(),
            },
        }
    }

    /// Create an invalid query condition error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `lhs_name` - The name of the left hand side of the condition.
    /// * `rhs_name` - The name of the right hand side of the condition.
    /// * `operator` - The operator of the condition.
    #[must_use]
    pub fn invalid_query_condition<S, T, U, V>(
        query_name: S,
        lhs_name: T,
        rhs_name: U,
        operator: V,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
        V: Into<Cow<'a, str>>,
    {
        Self::QueryError {
            query_name: query_name.into(),
            error: QueryError::InvalidCondition {
                lhs_name: lhs_name.into(),
                rhs_name: rhs_name.into(),
                operator: operator.into(),
            },
        }
    }

    /// Create an invalid query where name error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `schema_name` - The name of the root node of schema.
    /// * `where_name` - The name of the root node of where clause.
    #[must_use]
    pub fn invalid_query_where_name<S, T, U>(
        query_name: S,
        schema_name: T,
        where_name: U,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
        U: Into<Cow<'a, str>>,
    {
        Self::QueryError {
            query_name: query_name.into(),
            error: QueryError::InvalidWhereName {
                schema_name: schema_name.into(),
                where_name: where_name.into(),
            },
        }
    }

    /// Create an undefined query argument error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `argument_name` - The name of the argument.
    #[must_use]
    pub fn undefined_query_argument<S, T>(
        query_name: S,
        argument_name: T,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::QueryError {
            query_name: query_name.into(),
            error: QueryError::UndefinedArgument {
                argument_name: argument_name.into(),
            },
        }
    }

    /// Create an undefined query field name error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `field_name` - The name of the field.
    #[must_use]
    pub fn undefined_query_field<S, T>(
        query_name: S,
        field_name: T,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::QueryError {
            query_name: query_name.into(),
            error: QueryError::UndefinedField {
                field_name: field_name.into(),
            },
        }
    }

    /// Create an undefined return type error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `return_type` - The return type.
    #[must_use]
    pub fn undefined_query_return_type<S, T>(
        query_name: S,
        model_name: T,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::QueryError {
            query_name: query_name.into(),
            error: QueryError::UndefinedReturnType {
                model_name: model_name.into(),
            },
        }
    }

    /// Create an unused query argument error.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `argument_name` - The name of the argument.
    #[must_use]
    pub fn unused_query_argument<S, T>(
        query_name: S,
        argument_name: T,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::QueryError {
            query_name: query_name.into(),
            error: QueryError::UnusedArgument {
                argument_name: argument_name.into(),
            },
        }
    }
}

impl Display for TypeError<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::EnumError { enum_name, error } => {
                write!(f, "Error in enum `{enum_name}`: {error}.")
            }
            Self::ModelError { model_name, error } => {
                write!(f, "Error in model `{model_name}`: {error}.")
            }
            Self::QueryError { query_name, error } => {
                write!(f, "Error in query `{query_name}`: {error}.")
            }
        }
    }
}

impl Error for TypeError<'static> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::EnumError { error, .. } => Some(error),
            Self::ModelError { error, .. } => Some(error),
            Self::QueryError { error, .. } => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_duplicate_enum() {
        assert_eq!(
            TypeError::duplicate_enum("foo").to_string(),
            "Error in enum `foo`: enum already exists."
        );
    }

    #[test]
    fn test_duplicate_model() {
        assert_eq!(
            TypeError::duplicate_model("foo").to_string(),
            "Error in model `foo`: model already exists."
        );
    }

    #[test]
    fn test_duplicate_model_field() {
        assert_eq!(
            TypeError::duplicate_model_field("foo", "bar").to_string(),
            "Error in model `foo`: field `bar` already exists."
        );
    }

    #[test]
    fn test_empty_model() {
        assert_eq!(
            TypeError::empty_model("foo").to_string(),
            "Error in model `foo`: model has no fields."
        );
    }

    #[test]
    fn test_unknown_model_field_type() {
        assert_eq!(
            TypeError::unknown_model_field_type("foo", "bar", "[Boolean]")
                .to_string(),
            "Error in model `foo`: field `bar` has unknown type `[Boolean]`."
        );
    }

    #[test]
    fn test_duplicate_query() {
        assert_eq!(
            TypeError::duplicate_query("foo").to_string(),
            "Error in query `foo`: query already exists."
        );
    }

    #[test]
    fn test_empty_query_schema() {
        assert_eq!(
            TypeError::empty_query_schema("foo").to_string(),
            "Error in query `foo`: query schema is empty."
        );
    }

    #[test]
    fn test_invalid_query_argument_type() {
        assert_eq!(
            TypeError::invalid_query_argument_type("foo", "bar", "Float")
                .to_string(),
            "Error in query `foo`: argument `$bar` has invalid type `Float`."
        );
    }

    #[test]
    fn test_invalid_query_condition() {
        assert_eq!(
            TypeError::invalid_query_condition("foo", "bar", "baz", "equals")
                .to_string(),
            "Error in query `foo`: condition `bar equals baz` is invalid."
        );
    }

    #[test]
    fn test_invalid_query_where() {
        assert_eq!(
            TypeError::invalid_query_where_name("user", "post", "posts")
                .to_string(),
            "Error in query `user`: name of where root `posts` does not match \
             name of schema root `post`."
        );
    }

    #[test]
    fn test_undefined_query_argument() {
        assert_eq!(
            TypeError::undefined_query_argument("foo", "bar").to_string(),
            "Error in query `foo`: argument `$bar` is undefined."
        );
    }

    #[test]
    fn test_undefined_query_field() {
        assert_eq!(
            TypeError::undefined_query_field("foo", "bar").to_string(),
            "Error in query `foo`: field `bar` is undefined."
        );
    }

    #[test]
    fn test_undefined_query_return_type() {
        assert_eq!(
            TypeError::undefined_query_return_type("foo", "Bar").to_string(),
            "Error in query `foo`: return type `Bar` is undefined."
        );
    }

    #[test]
    fn test_unused_query_argument() {
        assert_eq!(
            TypeError::unused_query_argument("foo", "bar").to_string(),
            "Error in query `foo`: argument `$bar` is unused."
        );
    }

    #[test]
    fn test_source_enum_error() {
        assert_eq!(
            TypeError::duplicate_enum("foo")
                .source()
                .unwrap()
                .to_string(),
            "enum already exists"
        );
    }

    #[test]
    fn test_source_duplicate_model() {
        assert_eq!(
            TypeError::duplicate_model("foo")
                .source()
                .unwrap()
                .to_string(),
            "model already exists"
        );
    }

    #[test]
    fn test_source_duplicate_model_field() {
        assert_eq!(
            TypeError::duplicate_model_field("foo", "bar")
                .source()
                .unwrap()
                .to_string(),
            "field `bar` already exists"
        );
    }

    #[test]
    fn test_source_empty_model() {
        assert_eq!(
            TypeError::empty_model("foo").source().unwrap().to_string(),
            "model has no fields"
        );
    }

    #[test]
    fn test_source_unknown_model_field_type() {
        assert_eq!(
            TypeError::unknown_model_field_type("foo", "bar", "[Boolean]")
                .source()
                .unwrap()
                .to_string(),
            "field `bar` has unknown type `[Boolean]`"
        );
    }

    #[test]
    fn test_source_duplicate_query() {
        assert_eq!(
            TypeError::duplicate_query("foo")
                .source()
                .unwrap()
                .to_string(),
            "query already exists"
        );
    }
}
