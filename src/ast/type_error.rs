use super::{
    Field,
    QueryArgument,
    QueryCondition,
    Type,
};

/// Type checking errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeError {
    /// A query schema should contain at least one field, but the schema of
    /// this query is empty. An empty schema is not allowed because it would
    /// not return any data.
    EmptyQuerySchema {
        /// The name of the empty query.
        query_name: String,
    },
    /// The structure of the schema of a query should match the structure of
    /// of the model and its relations. This query schema includes a field
    /// that is not defined in the model.
    IncompatibleQuerySchema {
        /// The inferred type of the query schema.
        actual: Type,
        /// The return type of the query.
        expected: Type,
        /// The name of the query.
        query_name: String,
    },
    /// Certain conditions only make sense for certain types. For example, the
    /// `contains` condition does not make sense for a number, because the
    /// meaning of a number another number is ambiguous. This query contains
    /// a type of condition that is not compatible with the type of the field
    /// it is applied to.
    IncompatibleQueryCondition {
        /// The name of the query.
        query_name: String,
        /// The condition that was not satisfied.
        condition: QueryCondition,
        /// The type of the condition as given by the argument.
        expected: Type,
    },
    /// Condition operands must be compatible. For example, a string can only
    /// equal another string, and an integer can only equal another integer.
    /// This query contains a condition operand that is not compatible with
    /// the type of the field.
    IncompatibleQueryOperand {
        /// The name of the query.
        query_name: String,
        /// The condition that was not satisfied.
        condition: QueryCondition,
        /// The type of the condition as given by the argument.
        expected: Type,
    },

    /// The structure of a where clause of a query does not match the structure
    /// of the model and its relations.
    IncompatibleQueryWhere {
        /// The name of the query.
        query_name: String,
    },
    /// The name of the root node of the where clause of a query does not match
    /// the name of the root node of the schema.
    IncompatibleQueryRootNodes {
        /// The name of the schema root node.
        schema_root: String,
        /// The name of the query root node.
        where_root: String,
        /// The name of the query.
        query_name: String,
    },
    /// The type of an argument may not be an array or a model.
    InvalidQueryArgumentType {
        /// The argument that has an invalid type.
        argument: QueryArgument,
        /// The name of the query.
        query_name: String,
    },
    /// The type of a field of a model is undefined.
    UnknownModelFieldType {
        /// The field whose type is undefined.
        field: Field,
        /// The name of the model.
        model_name: String,
    },
    /// The type of a query argument is undefined.
    UnknownQueryArgumentType {
        /// The argument whose type is undefined.
        argument: QueryArgument,
        /// The name of the query.
        query_name: String,
    },
    /// The return type of a query is undefined.
    UnknownQueryReturnType {
        /// The name of the query.
        query_name: String,
        /// The return type of the query.
        r#type: Type,
    },
    /// A condition mentions an undefined argument.
    UnknownQueryConditionName {
        /// The condition that mentions an undefined argument.
        condition: QueryCondition,
        /// The name of the query.
        query_name: String,
    },
    /// The root component of a route is undefined.
    UnknownRouteRoot {
        /// The name of the route.
        route_name: String,
        /// The name of the component.
        root: String,
    },
    /// An argument of a query is not used in the where clause.
    UnusedQueryArgument {
        /// The argument that is not used.
        argument: QueryArgument,
        /// The name of the query.
        query_name: String,
    },
}
