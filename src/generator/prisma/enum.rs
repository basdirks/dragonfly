use {
    crate::{
        ast::Enum as AstEnum,
        generator::printer::Print,
    },
    std::fmt::Display,
};

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

impl Print for Enum {
    fn print(
        &self,
        _: usize,
    ) -> String {
        self.to_string()
    }
}

impl From<AstEnum> for Enum {
    fn from(
        AstEnum {
            name,
            variants: enumerators,
        }: AstEnum
    ) -> Self {
        Self { name, enumerators }
    }
}

impl From<&AstEnum> for Enum {
    fn from(ast_enum: &AstEnum) -> Self {
        Self::from(ast_enum.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let enum_ = Enum {
            name: "Color".to_owned(),
            enumerators: vec![
                "Red".to_owned(),
                "Green".to_owned(),
                "Blue".to_owned(),
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

    #[test]
    fn test_from() {
        let (ast_enum, _) = AstEnum::parse(
            "\
enum Color {
    Red
    Green
    Blue
}",
        )
        .unwrap();

        let r#enum = Enum::from(ast_enum);

        assert_eq!(
            r#enum.to_string(),
            "\
enum Color {
  Red
  Green
  Blue
}"
        );
    }
}
