use {
    super::{
        query::condition::FieldPath,
        Field,
        QueryArgument,
        QueryCondition,
        Type,
    },
    std::fmt::Display,
};

/// Type checking errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeError {
    /// A query schema should contain at least one field, but the schema of
    /// this query is empty. An empty schema is not allowed because it would
    /// not return any data.
    ///
    /// Checked in `dragonfly::ast::Query::check_empty_schema`.
    EmptyQuerySchema {
        /// The name of the empty query.
        query_name: String,
    },
    /// Operands must be compatible with their condition. For example, a string
    /// can only equal another string, and an integer can only equal another
    /// integer. This query contains a condition operand (either the field or
    /// the argument) that is not compatible with the condition.
    ///
    /// Checked in `dragonfly::ast::Ast::check_query_condition_types`.
    IncompatibleQueryOperator {
        /// The name of the query.
        query_name: String,
        /// The condition.
        condition: QueryCondition,
        /// The type of the condition as given by the argument.
        argument_type: Type,
        /// The type of the field that the condition is applied to.
        field_type: Type,
    },
    /// The root node of content of the where clause should have the same name
    /// as the root node of the query schema. The name of the root node of the
    /// where clause of this query does not match that of the root node of the
    /// schema.
    ///
    /// Checked in `dragonfly::ast::Query::check_root_nodes`.
    IncompatibleQueryRootNodes {
        /// The name of the schema root node.
        schema_root: String,
        /// The name of the query root node.
        where_root: String,
        /// The name of the query.
        query_name: String,
    },
    /// The structure of the schema of a query should match the structure of
    /// the model and its relations. This query schema includes a field that is
    /// not defined in the model.
    IncompatibleQuerySchema {
        /// The inferred type of the query schema.
        actual: Type,
        /// The return type of the query.
        expected: Type,
        /// The name of the query.
        query_name: String,
    },
    /// The type of a query argument must be a primitive, a reference to an
    /// existing enum, or an array of such a type. The type of an argument of
    /// this query is unknown.
    ///
    /// Checked in `dragonfly::ast::Query::check_argument_types`.
    InvalidQueryArgumentType {
        /// The argument that has an invalid type.
        argument: QueryArgument,
        /// The name of the query.
        query_name: String,
    },
    /// The type of a field in a model must be a primitive, a reference to an
    /// existing enum or model, or an array of such a type. The type of a field
    /// of this model is unknown.
    ///
    /// Checked in `dragonfly::ast::Model::check_field_types`.
    UnknownModelFieldType {
        /// The field whose type is undefined.
        field: Field,
        /// The name of the model.
        model_name: String,
    },
    /// A condition must refer to a query argument. This query contains a
    /// condition that refers to an undefined argument.
    ///
    /// Checked in `dragonfly::ast::Query::check_condition_references`.
    UnknownQueryConditionReference {
        /// The condition that mentions an undefined argument.
        condition: QueryCondition,
        /// The name of the query.
        query_name: String,
    },
    /// The return type of a query must reference an existing model. The model
    /// that this return type references does not exist.
    ///
    /// Checked in `dragonfly::ast::Query::check_return_type`.
    UnknownQueryReturnType {
        /// The name of the query.
        query_name: String,
        /// The name of the model.
        model_name: String,
    },
    /// The root of a route must be a reference to a known component. The root
    /// of this route is unknown.
    ///
    /// Checked in `dragonfly::ast::Route::check_root`.
    UnknownRouteRoot {
        /// The name of the route.
        route_name: String,
        /// The name of the component.
        root: String,
    },
    /// The path of each condition in a query must be a reference to a field
    /// that is defined in the model. This query contains a condition that
    /// refers to an undefined field.
    ///
    /// Checked in `dragonfly::ast::Ast::check_query_condition_types`.
    UnresolvedPath {
        /// The path that can not be resolved.
        path: FieldPath,
        /// The name of the model.
        model_name: String,
        /// The name of the query.
        query_name: String,
    },
    /// Every query argument must be used in the where clause. This query
    /// contains an argument that is not used.
    ///
    /// Checked in `dragonfly::ast::Query::check_unused_arguments`.
    UnusedQueryArgument {
        /// The argument that is not used.
        argument: QueryArgument,
        /// The name of the query.
        query_name: String,
    },
}

impl Display for TypeError {
    #[allow(clippy::too_many_lines)]
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::EmptyQuerySchema { query_name } => {
                write!(f, "Query `{query_name}` has an empty schema.")
            }
            Self::IncompatibleQueryOperator {
                query_name,
                condition,
                argument_type,
                field_type,
            } => {
                write!(
                    f,
                    "Query `{query_name}` contains a condition with \
                     incompatible operands. The condition is `{condition}`. \
                     The type of the condition as given by the argument is \
                     `{argument_type}`. The type of the field that the \
                     condition is applied to is `{field_type}`.",
                )
            }
            Self::IncompatibleQueryRootNodes {
                schema_root,
                where_root,
                query_name,
            } => {
                write!(
                    f,
                    "Query `{query_name}` contains a where clause with a root \
                     node that is incompatible with the root node of the \
                     schema. The root node of the schema is `{schema_root}`. \
                     The root node of the where clause is `{where_root}`.",
                )
            }
            Self::IncompatibleQuerySchema {
                actual,
                expected,
                query_name,
            } => {
                write!(
                    f,
                    "Query `{query_name}` contains a schema that is \
                     incompatible with the return type of the query. The \
                     inferred type of the query schema is `{actual}`. The \
                     return type of the query is `{expected}`.",
                )
            }
            Self::InvalidQueryArgumentType {
                argument,
                query_name,
            } => {
                write!(
                    f,
                    "Query `{query_name}` contains an argument with an \
                     invalid type. The argument is `{argument}`.",
                )
            }
            Self::UnknownModelFieldType { field, model_name } => {
                write!(
                    f,
                    "Model `{model_name}` contains a field with an unknown \
                     type. The field is `{field}`.",
                )
            }
            Self::UnknownQueryConditionReference {
                condition,
                query_name,
            } => {
                write!(
                    f,
                    "Query `{query_name}` contains a condition that refers to \
                     an undefined argument. The condition is `{condition}`.",
                )
            }
            Self::UnknownQueryReturnType {
                query_name,
                model_name,
            } => {
                write!(
                    f,
                    "Query `{query_name}` contains a return type that refers \
                     to an undefined model. The model is `{model_name}`.",
                )
            }
            Self::UnknownRouteRoot { route_name, root } => {
                write!(
                    f,
                    "Route `{route_name}` contains a root that refers to an \
                     undefined component. The root is `{root}`.",
                )
            }
            Self::UnresolvedPath {
                path,
                model_name,
                query_name,
            } => {
                write!(
                    f,
                    "Query `{query_name}` contains a condition that refers to \
                     an undefined field. The path is `{path}`. The model is \
                     `{model_name}`.",
                )
            }
            Self::UnusedQueryArgument {
                argument,
                query_name,
            } => {
                write!(
                    f,
                    "Query `{query_name}` contains an argument that is not \
                     used. The argument is `{argument}`.",
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_empty_schema_error() {
        assert_eq!(
            TypeError::EmptyQuerySchema {
                query_name: "foo".to_owned(),
            }
            .to_string(),
            "Query `foo` has an empty schema."
        );
    }

    #[test]
    fn test_display_incompatible_query_operator_error() {
        use crate::ast::{
            QueryCondition,
            QueryOperator,
            Scalar,
        };

        assert_eq!(
            TypeError::IncompatibleQueryOperator {
                query_name: "foo".to_owned(),
                condition: QueryCondition {
                    operator: QueryOperator::Equals,
                    field_path: FieldPath::new(&["foo", "bar", "baz",]),
                    argument: "baz".to_owned(),
                },
                argument_type: Type::Scalar(Scalar::String),
                field_type: Type::Scalar(Scalar::Int),
            }
            .to_string(),
            "Query `foo` contains a condition with incompatible operands. The \
             condition is `foo { bar { baz } } equals $baz`. The type of the \
             condition as given by the argument is `String`. The type of the \
             field that the condition is applied to is `Int`."
        );
    }

    #[test]
    fn test_display_incompatible_query_root_nodes_error() {
        assert_eq!(
            TypeError::IncompatibleQueryRootNodes {
                schema_root: "foo".to_owned(),
                where_root: "bar".to_owned(),
                query_name: "baz".to_owned(),
            }
            .to_string(),
            "Query `baz` contains a where clause with a root node that is \
             incompatible with the root node of the schema. The root node of \
             the schema is `foo`. The root node of the where clause is `bar`."
        );
    }

    #[test]
    fn test_display_incompatible_query_schema_error() {
        use crate::ast::{
            Scalar,
            Type,
        };

        assert_eq!(
            TypeError::IncompatibleQuerySchema {
                actual: Type::Array(Scalar::Reference("foo".to_owned())),
                expected: Type::Array(Scalar::Reference("bar".to_owned())),
                query_name: "baz".to_owned(),
            }
            .to_string(),
            "Query `baz` contains a schema that is incompatible with the \
             return type of the query. The inferred type of the query schema \
             is `[foo]`. The return type of the query is `[bar]`."
        );
    }

    #[test]
    fn test_display_invalid_query_argument_type_error() {
        use crate::ast::Scalar;

        assert_eq!(
            TypeError::InvalidQueryArgumentType {
                argument: QueryArgument {
                    name: "foo".to_owned(),
                    r#type: Type::Scalar(Scalar::String),
                },
                query_name: "bar".to_owned(),
            }
            .to_string(),
            "Query `bar` contains an argument with an invalid type. The \
             argument is `$foo: String`."
        );
    }

    #[test]
    fn test_display_unknown_model_field_type_error() {
        use crate::ast::Scalar;

        assert_eq!(
            TypeError::UnknownModelFieldType {
                field: Field {
                    name: "foo".to_owned(),
                    r#type: Type::Scalar(Scalar::String),
                },
                model_name: "bar".to_owned(),
            }
            .to_string(),
            "Model `bar` contains a field with an unknown type. The field is \
             `foo: String`."
        );
    }

    #[test]
    fn test_display_unknown_query_condition_reference_error() {
        use crate::ast::{
            QueryCondition,
            QueryOperator,
        };

        assert_eq!(
            TypeError::UnknownQueryConditionReference {
                condition: QueryCondition {
                    operator: QueryOperator::Equals,
                    field_path: FieldPath::new(&["foo", "bar", "baz",]),
                    argument: "baz".to_owned(),
                },
                query_name: "bar".to_owned(),
            }
            .to_string(),
            "Query `bar` contains a condition that refers to an undefined \
             argument. The condition is `foo { bar { baz } } equals $baz`."
        );
    }

    #[test]
    fn test_display_unknown_query_return_type_error() {
        assert_eq!(
            TypeError::UnknownQueryReturnType {
                query_name: "foo".to_owned(),
                model_name: "bar".to_owned(),
            }
            .to_string(),
            "Query `foo` contains a return type that refers to an undefined \
             model. The model is `bar`."
        );
    }

    #[test]
    fn test_display_unknown_route_root_error() {
        assert_eq!(
            TypeError::UnknownRouteRoot {
                route_name: "foo".to_owned(),
                root: "bar".to_owned(),
            }
            .to_string(),
            "Route `foo` contains a root that refers to an undefined \
             component. The root is `bar`."
        );
    }

    #[test]
    fn test_display_unresolved_path_error() {
        assert_eq!(
            TypeError::UnresolvedPath {
                path: FieldPath::new(&["foo", "bar", "baz",]),
                model_name: "foo".to_owned(),
                query_name: "bar".to_owned(),
            }
            .to_string(),
            "Query `bar` contains a condition that refers to an undefined \
             field. The path is `foo { bar { baz } }`. The model is `foo`."
        );
    }
}
