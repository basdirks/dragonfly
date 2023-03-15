use {
    super::attribute::Block,
    ir,
    print::Print,
    std::{
        borrow::Cow,
        io,
    },
    token_set::TokenSet,
};

/// A Prisma enum.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Enum<'a> {
    /// The name of the enum.
    pub name: Cow<'a, str>,
    /// The values of the enum.
    pub values: TokenSet,
    /// The attributes of the enum.
    pub attributes: Vec<Block<'a>>,
}

impl Print for Enum<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            attributes,
            name,
            values,
        } = self;

        let indent_outer = Self::indent(level);
        let indent_inner = Self::indent(level + 1);

        writeln!(f, "{indent_outer}enum {name} {{")?;

        for value in values.iter() {
            writeln!(f, "{indent_inner}{value}")?;
        }

        if !attributes.is_empty() {
            writeln!(f)?;

            for attribute in attributes {
                attribute.print(level + 1, f)?;
            }
        }

        writeln!(f, "{indent_outer}}}")
    }
}

impl<'a> From<ir::Enum<'a>> for Enum<'a> {
    fn from(ir::Enum { name, values }: ir::Enum<'a>) -> Self {
        Self {
            name: name.clone(),
            values,
            attributes: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Argument,
            Value,
        },
    };

    #[test]
    fn test_print() {
        let r#enum = Enum {
            name: "Color".into(),
            values: TokenSet::from_iter(["Red", "Green", "Blue"]),
            attributes: vec![Block {
                name: "map".into(),
                arguments: vec![Argument {
                    name: None,
                    value: Value::String("colors".into()),
                }],
                group: None,
            }],
        };

        let mut f = Vec::new();

        r#enum.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "enum Color {
  Red
  Green
  Blue

  @@map(\"colors\")
}
"
        );
    }

    #[test]
    fn test_from_ir_enum() {
        let ir_enum = ir::Enum {
            name: "Color".into(),
            values: TokenSet::from_iter(["Red", "Green", "Blue"]),
        };

        let r#enum = Enum::from(ir_enum);
        let mut f = Vec::new();

        r#enum.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "enum Color {
  Red
  Green
  Blue
}
"
        );
    }

    #[test]
    fn test_print_enum() {
        let r#enum = Enum {
            name: "Color".into(),
            values: TokenSet::from_iter(["Red", "Green", "Blue"]),
            attributes: vec![Block {
                name: "map".into(),
                arguments: vec![Argument {
                    name: None,
                    value: Value::String("colors".into()),
                }],
                group: None,
            }],
        };

        let mut f = Vec::new();

        r#enum.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "enum Color {
  Red
  Green
  Blue

  @@map(\"colors\")
}
"
        );
    }
}
