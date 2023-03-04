pub use {
    argument::Argument,
    argument_type::ArgumentType,
    condition::Condition,
    operator::Operator,
    r#where::Where,
    return_type::ReturnType,
    schema::{
        Node as SchemaNode,
        Schema,
    },
};

/// Arguments.
pub mod argument;
/// Argument types.
pub mod argument_type;
/// Query conditions.
pub mod condition;
/// Query condition operators.
pub mod operator;
/// Return types.
pub mod return_type;
/// Query schemas.
pub mod schema;
/// Query where clauses.
pub mod r#where;

/// A query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query {
    /// The name of the query.
    pub name: String,
    /// The return type of the query.
    pub r#type: ReturnType,
    /// The arguments to the query.
    pub arguments: Vec<Argument>,
    /// The schema of the return type.
    pub schema: Schema,
    /// The where clause of the query.
    pub where_clause: Option<Where>,
}

impl Query {
    /// Create a new query.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the query.
    /// * `r#type` - The return type of the query.
    /// * `alias` - The name of the schema and the contents of the where clause.
    #[must_use]
    pub fn new(
        name: &str,
        r#type: ReturnType,
        alias: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            r#type,
            arguments: vec![],
            schema: Schema {
                alias: alias.to_owned(),
                nodes: vec![],
            },
            where_clause: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::ir::Cardinality,
    };

    #[test]
    fn test_new() {
        let query = Query::new(
            "get_user",
            ReturnType::new("User", Cardinality::One),
            "user",
        );

        assert_eq!(query.name, "get_user".to_owned());
        assert_eq!(query.r#type.model_name, "User".to_owned());
        assert_eq!(query.r#type.cardinality, Cardinality::One);
        assert_eq!(query.arguments, vec![]);
        assert_eq!(query.schema.alias, "user".to_owned());
        assert_eq!(query.schema.nodes, vec![]);
        assert_eq!(query.where_clause, None);
    }
}
