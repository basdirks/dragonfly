use {
    crate::{
        ast::Enum as AstEnum,
        generator::printer::{
            indent,
            Print,
        },
    },
    std::fmt::Display,
};

/// A GraphQL enum.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The values of the enum.
    pub values: Vec<String>,
}

impl Display for Enum {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self { name, values } = self;
        let indent = indent::graphql(1);

        let values = values
            .iter()
            .map(|value| format!("{indent}{value}"))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "enum {name} {{\n{values}\n}}")
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
    fn from(ast_enum: AstEnum) -> Self {
        Self {
            name: ast_enum.name,
            values: ast_enum.variants,
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
    use super::*;

    #[test]
    fn test_display() {
        let r#enum = Enum {
            name: "Test".to_owned(),
            values: vec!["A".to_owned(), "B".to_owned()],
        };

        assert_eq!(
            r#enum.to_string(),
            "

enum Test {
  A
  B
}

"
            .trim()
        );
    }

    #[test]
    fn test_print() {
        let r#enum = Enum {
            name: "Test".to_owned(),
            values: vec!["A".to_owned(), "B".to_owned()],
        };

        assert_eq!(
            r#enum.print(0),
            "
        
enum Test {
  A
  B
}

"
            .trim()
        );
    }

    #[test]
    fn test_from() {
        let ast_enum = AstEnum {
            name: "Test".to_owned(),
            variants: vec!["A".to_owned(), "B".to_owned()],
        };

        let expected = Enum {
            name: "Test".to_owned(),
            values: vec!["A".to_owned(), "B".to_owned()],
        };

        assert_eq!(Enum::from(ast_enum.clone()), expected);
        assert_eq!(Enum::from(&ast_enum), expected);
    }
}
