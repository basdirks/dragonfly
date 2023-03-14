use {
    crate::Cardinality,
    std::borrow::Cow,
};

/// The return type of a query.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct ReturnType<'a> {
    /// The name of the model.
    pub model_name: Cow<'a, str>,
    /// The cardinality of the return type.
    pub cardinality: Cardinality,
}

impl<'a> From<ast::QueryReturnType<'a>> for ReturnType<'a> {
    fn from(ast_return_type: ast::QueryReturnType<'a>) -> Self {
        match ast_return_type {
            ast::QueryReturnType::Model(name) => {
                Self {
                    model_name: name,
                    cardinality: Cardinality::One,
                }
            }
            ast::QueryReturnType::Array(name) => {
                Self {
                    model_name: name,
                    cardinality: Cardinality::Many,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one_from_ast() {
        assert_eq!(
            ReturnType::from(ast::QueryReturnType::Model("User".into())),
            ReturnType {
                model_name: "User".into(),
                cardinality: Cardinality::One,
            },
        );
    }

    #[test]
    fn test_many_from_ast() {
        assert_eq!(
            ReturnType::from(ast::QueryReturnType::Array("User".into())),
            ReturnType {
                model_name: "User".into(),
                cardinality: Cardinality::Many,
            },
        );
    }
}
