use {
    super::attribute::Block,
    crate::{
        generator::printer::Print,
        ir::Enum as IrEnum,
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

impl Enum {
    /// Create a new enum.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    /// * `values` - The values of the enum.
    /// * `attributes` - The attributes of the enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::Enum;
    ///
    /// let r#enum = Enum::new("Foo", &["Bar"], &[]);
    ///
    /// assert_eq!(r#enum.name, "Foo");
    /// assert_eq!(r#enum.values, vec!["Bar".to_owned()]);
    /// assert!(r#enum.attributes.is_empty());
    /// ```
    pub fn new(
        name: &str,
        values: &[&str],
        attributes: &[Block],
    ) -> Self {
        Self {
            name: name.to_owned(),
            values: values.iter().map(ToString::to_string).collect(),
            attributes: attributes.to_owned(),
        }
    }
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

impl From<&IrEnum> for Enum {
    fn from(IrEnum { name, values }: &IrEnum) -> Self {
        Self {
            name: name.clone(),
            values: values.iter().map(ToString::to_string).collect(),
            attributes: vec![],
        }
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
        let r#enum = Enum::new(
            "Color",
            &["Red", "Green", "Blue"],
            &[Block::new(
                "map",
                &[Argument::unnamed(&Value::string("colors"))],
                None,
            )],
        );

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
        let ir_enum = IrEnum::new(
            "Color",
            &["Red", "Green", "Blue"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        );
        let r#enum = Enum::from(&ir_enum);

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
        let r#enum = Enum::new(
            "Color",
            &["Red", "Green", "Blue"],
            &[Block::new(
                "map",
                &[Argument::unnamed(&Value::string("colors"))],
                None,
            )],
        );

        assert_eq!(
            r#enum.print(0),
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
    fn test_enum_from_ir_enum() {
        let ir_enum = IrEnum::new(
            "Color",
            &["Red", "Green", "Blue"]
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>(),
        );

        let r#enum = Enum::from(&ir_enum);

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
