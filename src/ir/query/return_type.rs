use crate::{
    ast,
    ir::Cardinality,
};

/// The return type of a query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ReturnType {
    /// The name of the model.
    pub model_name: String,
    /// The cardinality of the return type.
    pub cardinality: Cardinality,
}

impl ReturnType {
    /// Create a new query return type.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    /// * `cardinality` - The cardinality of the return type.
    #[must_use]
    pub fn new(
        model_name: &str,
        cardinality: Cardinality,
    ) -> Self {
        Self {
            model_name: model_name.to_owned(),
            cardinality,
        }
    }

    /// Create a new return type with a single model.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    #[must_use]
    pub fn one(model_name: &str) -> Self {
        Self::new(model_name, Cardinality::One)
    }

    /// Create a new return type with multiple models.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    #[must_use]
    pub fn many(model_name: &str) -> Self {
        Self::new(model_name, Cardinality::Many)
    }
}

impl From<&ast::QueryReturnType> for ReturnType {
    fn from(ast_return_type: &ast::QueryReturnType) -> Self {
        match ast_return_type {
            ast::QueryReturnType::Model(name) => Self::one(name),
            ast::QueryReturnType::Array(name) => Self::many(name),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            ReturnType::new("User", Cardinality::Many),
            ReturnType {
                model_name: "User".to_owned(),
                cardinality: Cardinality::Many,
            },
        );
    }

    #[test]
    fn test_one_from_ast() {
        assert_eq!(
            ReturnType::from(&ast::QueryReturnType::Model("User".to_owned())),
            ReturnType {
                model_name: "User".to_owned(),
                cardinality: Cardinality::One,
            },
        );
    }

    #[test]
    fn test_many_from_ast() {
        assert_eq!(
            ReturnType::from(&ast::QueryReturnType::Array("User".to_owned())),
            ReturnType {
                model_name: "User".to_owned(),
                cardinality: Cardinality::Many,
            },
        );
    }

    #[test]
    fn test_one() {
        assert_eq!(
            ReturnType::one("User"),
            ReturnType {
                model_name: "User".to_owned(),
                cardinality: Cardinality::One,
            },
        );
    }

    #[test]
    fn test_many() {
        assert_eq!(
            ReturnType::many("User"),
            ReturnType {
                model_name: "User".to_owned(),
                cardinality: Cardinality::Many,
            },
        );
    }
}
