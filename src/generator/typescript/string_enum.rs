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

/// A TypeScript enum variant, usually called `member` in TypeScript ASTs. A
/// variant's value may differ from its name.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Variant {
    /// The name of the variant. Must be unique within the enum. Usually
    /// pascal case.
    pub name: String,
    /// The value of the variant. May differ from the name.
    pub value: String,
}

impl Variant {
    /// Create a new variant.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the variant.
    /// * `value` - The value of the variant.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::string_enum::Variant;
    ///
    /// let variant = Variant::new("France", "France");
    ///
    /// assert_eq!(variant.name, "France");
    /// assert_eq!(variant.value, "France");
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        value: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            value: value.to_owned(),
        }
    }
}

impl Print for Variant {
    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self { name, value } = self;
        let indent = indent::typescript(level);

        writeln!(f, "{indent}{name} = \"{value}\",")
    }
}

impl Display for Variant {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{} = \"{}\"", self.name, self.value)
    }
}

/// A TypeScript enum declaration.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct StringEnum {
    /// The name of the enum.
    ///
    /// # Examples
    ///
    /// `CountryName` is the identifier:
    ///
    /// ```typescript
    /// enum CountryName {
    ///     France = "France",
    ///     Germany = "Germany",
    /// }
    /// ```
    pub identifier: String,
    /// Enum variants, usually called `members` in TypeScript ASTs.
    ///
    /// # Examples
    ///
    /// `France` and `Germany` are variants:
    ///
    /// ```typescript
    /// enum CountryName {
    ///     France = "France",
    ///     Germany = "Germany",
    /// }
    /// ```
    pub variants: Vec<Variant>,
}

impl StringEnum {
    /// Create a new enum.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The name of the enum.
    /// * `variants` - The variants of the enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::string_enum::StringEnum;
    ///
    /// let r#enum = StringEnum::new("CountryName", &["France", "Germany"]);
    ///
    /// assert_eq!(r#enum.identifier, "CountryName".to_owned());
    /// assert_eq!(r#enum.variants.len(), 2);
    /// ```
    #[must_use]
    pub fn new(
        identifier: &str,
        variants: &[&str],
    ) -> Self {
        Self {
            identifier: identifier.to_owned(),
            variants: variants
                .iter()
                .map(|variant| Variant::new(variant, variant))
                .collect::<Vec<_>>(),
        }
    }
}

impl Print for StringEnum {
    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            identifier: name,
            variants,
        } = self;

        let indent = indent::typescript(level);

        writeln!(f, "{indent}enum {name} {{")?;

        for variant in variants {
            variant.print(level + 1, f)?;
        }

        writeln!(f, "{indent}}}")
    }
}

impl From<IrEnum> for StringEnum {
    fn from(ir_enum: IrEnum) -> Self {
        let IrEnum { name, values } = ir_enum;

        Self {
            identifier: name,
            variants: values
                .iter()
                .map(|value| Variant::new(value, value))
                .collect::<Vec<_>>(),
        }
    }
}

impl From<&IrEnum> for StringEnum {
    fn from(ir_enum: &IrEnum) -> Self {
        Self::from(ir_enum.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ir_enum() {
        assert_eq!(
            StringEnum::from(IrEnum::new(
                "CountryName",
                &["France", "Germany", "Italy"]
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
            )),
            StringEnum::new("CountryName", &["France", "Germany", "Italy"])
        );
    }

    #[test]
    fn test_print_enum() {
        let r#enum = StringEnum::new(
            "CountryName",
            &[
                "France",
                "Germany",
                "Italy",
                "Spain",
                "UnitedKingdom",
                "UnitedStates",
            ],
        );

        let mut f = Vec::new();

        r#enum.print(1, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "    enum CountryName {
        France = \"France\",
        Germany = \"Germany\",
        Italy = \"Italy\",
        Spain = \"Spain\",
        UnitedKingdom = \"UnitedKingdom\",
        UnitedStates = \"UnitedStates\",
    }
"
        );
    }

    #[test]
    fn test_display_variant() {
        assert_eq!(
            Variant::new("France", "France").to_string(),
            "France = \"France\""
        );
    }

    #[test]
    fn test_enum_from_ir_enum() {
        let ir_enum = IrEnum::new(
            "CountryName",
            &[
                "France",
                "Germany",
                "Italy",
                "Spain",
                "UnitedKingdom",
                "UnitedStates",
            ]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>(),
        );

        let expected = StringEnum::new(
            "CountryName",
            &[
                "France",
                "Germany",
                "Italy",
                "Spain",
                "UnitedKingdom",
                "UnitedStates",
            ],
        );

        assert_eq!(StringEnum::from(ir_enum.clone()), expected);
        assert_eq!(StringEnum::from(&ir_enum), expected);
    }
}
