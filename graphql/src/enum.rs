use {
    ir,
    print::Print,
    std::{
        borrow::Cow,
        io,
    },
    token_set::TokenSet,
};

/// A GraphQL enum.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Enum<'a> {
    /// The name of the enum.
    pub name: Cow<'a, str>,
    /// The values of the enum.
    pub values: TokenSet,
}

impl Print for Enum<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self { name, values } = self;
        let indent_outer = Self::indent(level);
        let indent_inner = Self::indent(level + 1);

        writeln!(f, "{indent_outer}enum {name} {{")?;

        for value in values.iter() {
            writeln!(f, "{indent_inner}{value}")?;
        }

        writeln!(f, "{indent_outer}}}")
    }
}

impl<'a> From<ir::Enum<'a>> for Enum<'a> {
    fn from(ir_enum: ir::Enum<'a>) -> Self {
        let mut r#enum = Self {
            name: ir_enum.name.clone(),
            values: TokenSet::new(),
        };

        for value in ir_enum.values.iter() {
            let _: bool = r#enum.values.insert(value);
        }

        r#enum
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r#enum = Enum {
            name: "Test".into(),
            values: TokenSet::from_iter(["A", "B"]),
        };

        let mut f = Vec::new();

        r#enum.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "enum Test {
  A
  B
}
"
        );
    }

    #[test]
    fn test_print() {
        let r#enum = Enum {
            name: "Test".into(),
            values: TokenSet::from_iter(["A", "B"]),
        };

        let mut f = Vec::new();

        r#enum.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "\
enum Test {
  A
  B
}
"
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(
            Enum::from(ir::Enum {
                name: "Test".into(),
                values: TokenSet::from_iter(["A", "B"])
            }),
            Enum {
                name: "Test".into(),
                values: TokenSet::from_iter(["A", "B"])
            }
        );
    }
}
