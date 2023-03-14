use std::{
    borrow::Cow,
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

/// Errors that can occur when type checking a query.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum QueryError<'a> {
    /// The name of a query must be unique within the application. This query
    /// has the same name as another query.
    Duplicate,
    /// A schema should contain at least one field, but the schema of this
    /// query is empty. An empty schema is not allowed because no data would
    /// be returned.
    EmptySchema,
    /// The type of a query argument must be a primitive, a reference to an
    /// existing enum, or an array of such a type. The type of an argument of
    /// this query is unknown.
    InvalidArgumentType {
        /// The name of the argument.
        argument_name: Cow<'a, str>,
        /// The type of the argument.
        argument_type: Cow<'a, str>,
    },
    /// Operands must be compatible with their condition. For example, a string
    /// can only equal another string, and an integer can only equal another
    /// integer. This query contains a condition operand (either the field or
    /// the argument) that is not compatible with the condition.
    InvalidCondition {
        /// The name of the field.
        lhs_name: Cow<'a, str>,
        /// The name of the argument.
        rhs_name: Cow<'a, str>,
        /// The operator.
        operator: Cow<'a, str>,
    },
    /// The root node of content of the where clause should have the same name
    /// as the root node of the schema. The name of the root node of the where
    /// clause does not match that of the root node of the schema.
    InvalidWhereName {
        /// The name of the schema root node.
        schema_name: Cow<'a, str>,
        /// The name of the query root node.
        where_name: Cow<'a, str>,
    },
    /// A condition must refer to a query argument. This query contains a
    /// condition that refers to an undefined argument.
    UndefinedArgument {
        /// The name of the argument.
        argument_name: Cow<'a, str>,
    },
    /// The structure of the schema of a query should match the structure of
    /// the model and its relations. This query schema includes a field that is
    /// not defined in the model.
    UndefinedField {
        /// The name of the field.
        field_name: Cow<'a, str>,
    },
    /// The return type of a query must refer to an existing model. This query
    /// refers to an undefined model.
    UndefinedReturnType {
        /// The name of the model.
        model_name: Cow<'a, str>,
    },
    /// Every argument of a query must be used in the where clause. This query
    /// contains an unused argument.
    UnusedArgument {
        /// The name of the argument.
        argument_name: Cow<'a, str>,
    },
}

impl Display for QueryError<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Duplicate => write!(f, "query already exists"),
            Self::EmptySchema => write!(f, "query schema is empty"),
            Self::InvalidArgumentType {
                argument_name,
                argument_type,
            } => {
                write!(
                    f,
                    "argument `${argument_name}` has invalid type \
                     `{argument_type}`"
                )
            }
            Self::InvalidCondition {
                lhs_name,
                rhs_name,
                operator,
            } => {
                write!(
                    f,
                    "condition `{lhs_name} {operator} {rhs_name}` is invalid"
                )
            }
            Self::InvalidWhereName {
                schema_name,
                where_name,
            } => {
                write!(
                    f,
                    "name of where root `{where_name}` does not match name of \
                     schema root `{schema_name}`"
                )
            }
            Self::UndefinedArgument { argument_name } => {
                write!(f, "argument `${argument_name}` is undefined")
            }
            Self::UndefinedField { field_name } => {
                write!(f, "field `{field_name}` is undefined")
            }
            Self::UndefinedReturnType { model_name } => {
                write!(f, "return type `{model_name}` is undefined")
            }
            Self::UnusedArgument { argument_name } => {
                write!(f, "argument `${argument_name}` is unused")
            }
        }
    }
}

impl Error for QueryError<'_> {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_duplicate() {
        assert!(QueryError::Duplicate.source().is_none());
    }

    #[test]
    fn test_source_empty_schema() {
        assert!(QueryError::EmptySchema.source().is_none());
    }

    #[test]
    fn test_source_invalid_argument_type() {
        assert!(QueryError::InvalidArgumentType {
            argument_name: "foo".into(),
            argument_type: "bar".into(),
        }
        .source()
        .is_none());
    }

    #[test]
    fn test_source_invalid_condition() {
        assert!(QueryError::InvalidCondition {
            lhs_name: "foo".into(),
            rhs_name: "bar".into(),
            operator: "baz".into(),
        }
        .source()
        .is_none());
    }

    #[test]
    fn test_source_invalid_where_name() {
        assert!(QueryError::InvalidWhereName {
            schema_name: "foo".into(),
            where_name: "bar".into(),
        }
        .source()
        .is_none());
    }

    #[test]
    fn test_source_undefined_argument() {
        assert!(QueryError::UndefinedArgument {
            argument_name: "foo".into(),
        }
        .source()
        .is_none());
    }

    #[test]
    fn test_source_undefined_field() {
        assert!(QueryError::UndefinedField {
            field_name: "foo".into(),
        }
        .source()
        .is_none());
    }

    #[test]
    fn test_source_undefined_return_type() {
        assert!(QueryError::UndefinedReturnType {
            model_name: "foo".into(),
        }
        .source()
        .is_none());
    }

    #[test]
    fn test_source_unused_argument() {
        assert!(QueryError::UnusedArgument {
            argument_name: "foo".into(),
        }
        .source()
        .is_none());
    }
}
