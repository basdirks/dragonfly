use crate::ast;

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
        values: &[&str],
    ) -> Self {
        Self {
            name: name.to_owned(),
            values: values.iter().map(ToString::to_string).collect(),
        }
    }
}

impl From<ast::Enum> for Enum {
    fn from(ast_enum: ast::Enum) -> Self {
        Self {
            name: ast_enum.name,
            values: ast_enum.variants,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Enum::new("Role", &["Admin", "User"]),
            Enum {
                name: "Role".to_owned(),
                values: vec!["Admin".to_owned(), "User".to_owned()],
            }
        );
    }

    #[test]
    fn test_from_ast_enum() {
        assert_eq!(
            Enum::from(ast::Enum::new("Role", &["Admin", "User"])),
            Enum::new("Role", &["Admin", "User"]),
        );
    }
}
