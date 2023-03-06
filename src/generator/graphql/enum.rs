use {
    crate::{
        generator::printer::{
            indent,
            Print,
        },
        ir::Enum as IrEnum,
    },
    std::{
        fmt::Display,
        io,
    },
};

/// A GraphQL enum.
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
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{self}")
    }
}

impl From<&IrEnum> for Enum {
    fn from(ir_enum: &IrEnum) -> Self {
        Self {
            name: ir_enum.name.clone(),
            values: ir_enum.values.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let r#enum = Enum::new("Test", &["A", "B"]);

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
        let r#enum = Enum::new("Test", &["A", "B"]);
        let mut f = Vec::new();

        r#enum.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "\
enum Test {
  A
  B
}"
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(
            Enum::from(&IrEnum::new(
                "Test",
                &["A", "B"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            )),
            Enum::new("Test", &["A", "B"])
        );
    }
}
