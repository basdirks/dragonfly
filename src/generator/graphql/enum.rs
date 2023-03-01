use {
    crate::{
        generator::printer::{
            indent,
            Print,
        },
        ir::Enum as IrEnum,
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

impl Enum {
    /// Create a new enum.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    /// * `values` - The values of the enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Enum;
    ///
    /// let r#enum = Enum::new("Foo", &["BAR", "BAZ"]);
    ///
    /// assert_eq!(r#enum.name, "Foo");
    /// assert_eq!(r#enum.values, vec!["BAR".to_owned(), "BAZ".to_owned()]);
    /// ```
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
    ) -> String {
        self.to_string()
    }
}

impl From<IrEnum> for Enum {
    fn from(ir_enum: IrEnum) -> Self {
        Self {
            name: ir_enum.name,
            values: ir_enum.values,
        }
    }
}

impl From<&IrEnum> for Enum {
    fn from(ir_enum: &IrEnum) -> Self {
        Self::from(ir_enum.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
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
        let ir_enum = IrEnum::new("Test", &["A", "B"]);
        let expected = Enum::new("Test", &["A", "B"]);

        assert_eq!(Enum::from(ir_enum.clone()), expected);
        assert_eq!(Enum::from(&ir_enum), expected);
    }
}
