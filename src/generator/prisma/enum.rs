use {
    super::attribute::Block,
    crate::{
        ast::Enum as AstEnum,
        generator::printer::Print,
    },
    std::fmt::Display,
};

/// A Prisma enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The values of the enum.
    pub values: Vec<String>,
    /// The attributes of the enum.
    pub attributes: Vec<Block>,
}

impl Display for Enum {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            attributes,
            name,
            values,
        } = self;

        let values = values
            .iter()
            .map(|value| format!("  {value}"))
            .collect::<Vec<_>>()
            .join("\n");

        let attributes = if attributes.is_empty() {
            String::new()
        } else {
            format!(
                "\n\n{}",
                attributes
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join("\n")
            )
        };

        write!(f, "enum {name} {{\n{values}{attributes}\n}}")
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
            variants: values,
        }: AstEnum
    ) -> Self {
        Self {
            name,
            values,
            attributes: vec![],
        }
    }
}

impl From<&AstEnum> for Enum {
    fn from(ast_enum: &AstEnum) -> Self {
        Self::from(ast_enum.clone())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::prisma::{
            Argument,
            Value,
        },
    };

    #[test]
    fn test_display() {
        let r#enum = Enum {
            attributes: vec![Block {
                arguments: vec![Argument {
                    name: None,
                    value: Value::String("colors".to_owned()),
                }],
                group: None,
                name: "map".to_owned(),
            }],
            name: "Color".to_owned(),
            values: vec![
                "Red".to_owned(),
                "Green".to_owned(),
                "Blue".to_owned(),
            ],
        };

        assert_eq!(
            r#enum.to_string(),
            "

enum Color {
  Red
  Green
  Blue

  @@map(\"colors\")
}

"
            .trim()
        );
    }

    #[test]
    fn test_from() {
        let (ast_enum, _) = AstEnum::parse(
            "

enum Color {
    Red
    Green
    Blue
}

"
            .trim(),
        )
        .unwrap();

        let r#enum = Enum::from(ast_enum);

        assert_eq!(
            r#enum.to_string(),
            "

enum Color {
  Red
  Green
  Blue
}

"
            .trim()
        );
    }

    #[test]
    fn test_print_enum() {
        let enum_ = Enum {
            attributes: vec![Block {
                arguments: vec![Argument {
                    name: None,
                    value: Value::String("colors".to_owned()),
                }],
                group: None,
                name: "map".to_owned(),
            }],
            name: "Color".to_owned(),
            values: vec![
                "Red".to_owned(),
                "Green".to_owned(),
                "Blue".to_owned(),
            ],
        };

        assert_eq!(
            enum_.print(0),
            "

enum Color {
  Red
  Green
  Blue

  @@map(\"colors\")
}

"
            .trim()
        );
    }

    #[test]
    fn test_enum_from_ast_enum() {
        let (ast_enum, _) = AstEnum::parse(
            "

enum Color {
  Red
  Green
  Blue
}

"
            .trim(),
        )
        .unwrap();

        let r#enum = Enum::from(ast_enum);

        assert_eq!(
            r#enum.to_string(),
            "

enum Color {
  Red
  Green
  Blue
}

"
            .trim()
        );
    }
}
