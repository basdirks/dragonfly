use super::Condition;

/// A query where clause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Where {
    /// The alias of the where clause.
    pub alias: String,
    /// The conditions of the where clause.
    pub conditions: Vec<Condition>,
}

impl Where {
    /// Create a new where clause.
    ///
    /// # Arguments
    ///
    /// * `alias` - The alias of the where clause.
    /// * `conditions` - The conditions of the where clause.
    #[must_use]
    pub fn new(
        alias: &str,
        conditions: &[Condition],
    ) -> Self {
        Self {
            alias: alias.to_owned(),
            conditions: conditions.to_owned(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Where::new("user", &[]),
            Where {
                alias: "user".to_owned(),
                conditions: vec![],
            }
        );
    }
}
