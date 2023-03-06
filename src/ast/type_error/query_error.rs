use {
    crate::ast::{
        QueryArgument,
        QueryCondition,
        QueryReturnType,
    },
    std::{
        fmt,
        fmt::{
            Display,
            Formatter,
        },
    },
};

/// Errors that can occur when type checking a query.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum QueryError {
    /// The name of a query must be unique within the application. This query
    /// has the same name as another query.
    Duplicate,
    /// The name of a query argument must be unique within the query. This
    /// query contains an argument with the same name as another argument.
    DuplicateArgument(String),
    /// A schema should contain at least one field, but the schema of this
    /// query is empty. An empty schema is not allowed because no data would
    /// be returned.
    EmptySchema,
    /// The type of a query argument must be a primitive, a reference to an
    /// existing enum, or an array of such a type. The type of an argument of
    /// this query is unknown.
    InvalidArgumentType(QueryArgument),
    /// Operands must be compatible with their condition. For example, a string
    /// can only equal another string, and an integer can only equal another
    /// integer. This query contains a condition operand (either the field or
    /// the argument) that is not compatible with the condition.
    InvalidCondition(QueryCondition),
    /// The root node of content of the where clause should have the same name
    /// as the root node of the schema. The name of the root node of the where
    /// clause does not match that of the root node of the schema.
    InvalidWhereName {
        /// The name of the schema root node.
        schema_name: String,
        /// The name of the query root node.
        where_name: String,
    },
    /// A condition must refer to a query argument. This query contains a
    /// condition that refers to an undefined argument.
    UndefinedArgument(String),
    /// The structure of the schema of a query should match the structure of
    /// the model and its relations. This query schema includes a field that is
    /// not defined in the model.
    UndefinedField(String),
    /// The return type of a query must refer to an existing model. This query
    /// refers to an undefined model.
    UndefinedReturnType(QueryReturnType),
    /// Every argument of a query must be used in the where clause. This query
    /// contains an unused argument.
    UnusedArgument(String),
}

impl Display for QueryError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        match self {
            Self::Duplicate => write!(f, "duplicate query"),
            Self::DuplicateArgument(name) => {
                write!(f, "duplicate argument `${name}`")
            }
            Self::EmptySchema => write!(f, "empty schema"),
            Self::InvalidArgumentType(QueryArgument { name, r#type }) => {
                write!(f, "argument `${name}` has invalid type `{type}`")
            }
            Self::InvalidCondition(QueryCondition {
                argument_name,
                operator,
                path,
            }) => {
                write!(
                    f,
                    "operator `{operator}` is not compatible with the types \
                     of field `{path}` and argument `${argument_name}`"
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
            Self::UndefinedArgument(name) => {
                write!(f, "argument `${name}` is undefined")
            }
            Self::UndefinedField(name) => {
                write!(f, "schema field `{name}` is undefined")
            }
            Self::UndefinedReturnType(
                QueryReturnType::Array(name) | QueryReturnType::Model(name),
            ) => {
                write!(f, "return type refers to undefined model `{name}`")
            }
            Self::UnusedArgument(name) => {
                write!(f, "argument `${name}` is unused")
            }
        }
    }
}
