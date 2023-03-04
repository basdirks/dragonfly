use crate::ir::Cardinality;

/// An enum relation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EnumRelation {
    /// The name of the enum.
    pub name: String,
    /// The cardinality of the relation.
    pub cardinality: Cardinality,
}

impl EnumRelation {
    /// Create a new enum relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
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

    /// Create an enum to-many relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    #[must_use]
    pub fn many(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an enum to-one relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
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
            EnumRelation::new("Role", Cardinality::Many),
            EnumRelation {
                name: "Role".to_owned(),
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_one() {
        assert_eq!(
            EnumRelation::one("role"),
            EnumRelation {
                name: "role".to_owned(),
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_many() {
        assert_eq!(
            EnumRelation::many("roles"),
            EnumRelation {
                name: "roles".to_owned(),
                cardinality: Cardinality::Many,
            }
        );
    }
}
