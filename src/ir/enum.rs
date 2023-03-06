use {
    crate::ast::{
        self,
        TypeError,
    },
    std::collections::HashSet,
};

/// An enum type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The values of the enum.
    pub values: Vec<String>,
}

impl Enum {
    /// Create a new enum.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    /// * `values` - The values of the enum.
    #[must_use]
    pub fn new(
        name: &str,
        values: &[String],
    ) -> Self {
        Self {
            name: name.to_owned(),
            values: values.to_vec(),
        }
    }
}

impl TryFrom<ast::Enum> for Enum {
    type Error = TypeError;

    fn try_from(ast_enum: ast::Enum) -> Result<Self, Self::Error> {
        let mut unique_values = HashSet::new();
        let mut values = Vec::new();

        for value in ast_enum.variants {
            if !unique_values.insert(value.clone()) {
                return Err(TypeError::duplicate_enum_variant(
                    &ast_enum.name,
                    &value,
                ));
            }

            values.push(value);
        }

        Ok(Self::new(&ast_enum.name, &values))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Enum::new(
                "Role",
                &["Admin", "User"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            ),
            Enum {
                name: "Role".to_owned(),
                values: vec!["Admin".to_owned(), "User".to_owned()],
            }
        );
    }

    #[test]
    fn test_try_from_ast_enum() {
        assert_eq!(
            Enum::try_from(ast::Enum::new("Role", &["Admin", "User"])),
            Ok(Enum::new(
                "Role",
                &["Admin", "User"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            ))
        );
    }
}
