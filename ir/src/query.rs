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
use {
    ord_str_map::OrdStrMap,
    std::borrow::Cow,
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
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Query<'a> {
    /// The name of the query.
    pub name: Cow<'a, str>,
    /// The return type of the query.
    pub r#type: ReturnType<'a>,
    /// The arguments to the query.
    pub arguments: OrdStrMap<Argument<'a>>,
    /// The schema of the return type.
    pub schema: Schema<'a>,
    /// The where clause of the query.
    pub r#where: Option<Where<'a>>,
}

impl<'a> Query<'a> {
    /// Create a new query.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the query.
    /// * `r#type` - The return type of the query.
    /// * `alias` - The name of the schema and the contents of the where clause.
    #[must_use]
    pub fn new<S, T>(
        name: S,
        r#type: ReturnType<'a>,
        alias: T,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            r#type,
            arguments: OrdStrMap::new(),
            schema: Schema {
                alias: alias.into(),
                nodes: Vec::new(),
            },
            r#where: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Cardinality,
    };

    #[test]
    fn test_new() {
        let query = Query::new(
            "get_user",
            ReturnType {
                model_name: "User".into(),
                cardinality: Cardinality::One,
            },
            "user",
        );

        assert_eq!(
            query,
            Query {
                name: "get_user".into(),
                r#type: ReturnType {
                    model_name: "User".into(),
                    cardinality: Cardinality::One,
                },
                arguments: OrdStrMap::new(),
                schema: Schema {
                    alias: "user".into(),
                    nodes: Vec::new(),
                },
                r#where: None,
            }
        );
    }
}
