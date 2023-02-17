use std::fmt::Display;

/// An enumerated type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The values of the enum.
    pub enumerators: Vec<String>,
}

impl Display for Enum {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self { name, enumerators } = self;

        let enumerators = enumerators
            .iter()
            .map(|enumerator| format!("  {enumerator}"))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "enum {name} {{\n{enumerators}\n}}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let enum_ = Enum {
            name: "Color".to_string(),
            enumerators: vec![
                "Red".to_string(),
                "Green".to_string(),
                "Blue".to_string(),
            ],
        };

        assert_eq!(
            enum_.to_string(),
            "\
enum Color {
  Red
  Green
  Blue
}"
        );
    }
}
