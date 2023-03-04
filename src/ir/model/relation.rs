use crate::ir::Cardinality;

/// A model relation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Relation {
    /// The name of the model.
    pub name: String,
    /// The cardinality of the relation.
    pub cardinality: Cardinality,
}

impl Relation {
    /// Create a new model relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    /// * `cardinality` - The cardinality of the relation.
    #[must_use]
    pub fn new(
        name: &str,
        cardinality: Cardinality,
    ) -> Self {
        Self {
            name: name.to_owned(),
            cardinality,
        }
    }

    /// Create a model to-many relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    #[must_use]
    pub fn many(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            cardinality: Cardinality::Many,
        }
    }

    /// Create a model to-one relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    #[must_use]
    pub fn one(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            cardinality: Cardinality::One,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Relation::new("user", Cardinality::One),
            Relation {
                name: "user".to_owned(),
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_many() {
        assert_eq!(
            Relation::many("users"),
            Relation {
                name: "users".to_owned(),
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_one() {
        assert_eq!(
            Relation::one("user"),
            Relation {
                name: "user".to_owned(),
                cardinality: Cardinality::One,
            }
        );
    }
}
