use {
    crate::{
        ast::Enum as AstEnum,
        generator::printer::{
            indent,
            newline_separated,
            Print,
        },
    },
    std::fmt::Display,
};

/// A TypeScript enum variant, usually called `member` in TypeScript ASTs. A
/// variant's value may differ from its name.
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variant {
    /// The name of the variant. Must be unique within the enum. Usually
    /// pascal case.
    name: String,
    /// The value of the variant. May differ from the name.
    value: String,
}

impl Print for Variant {
    fn print(
        &self,
        level: usize,
    ) -> String {
        let Self { name, value } = self;
        let indent = indent::typescript(level);

        format!("{indent}{name} = \"{value}\",")
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
    identifier: String,
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
    variants: Vec<Variant>,
}

impl Print for StringEnum {
    fn print(
        &self,
        level: usize,
    ) -> String {
        let Self {
            identifier: name,
            variants,
        } = self;

        let indent = indent::typescript(level);

        let variants = newline_separated(
            &variants
                .iter()
                .map(|variant| variant.print(level + 1))
                .collect::<Vec<_>>(),
        );

        format!("{indent}enum {name} {{\n{variants}\n{indent}}}")
    }
}

impl From<AstEnum> for StringEnum {
    fn from(value: AstEnum) -> Self {
        let AstEnum { name, variants } = value;

        Self {
            identifier: name,
            variants: variants
                .iter()
                .map(|variant| {
                    Variant {
                        name: variant.clone(),
                        value: variant.clone(),
                    }
                })
                .collect::<Vec<_>>(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ast_enum() {
        assert_eq!(
            StringEnum::from(AstEnum {
                name: "CountryName".to_string(),
                variants: vec![
                    "France".to_string(),
                    "Germany".to_string(),
                    "Italy".to_string(),
                    "Spain".to_string(),
                    "UnitedKingdom".to_string(),
                    "UnitedStates".to_string(),
                ]
            }),
            StringEnum {
                identifier: "CountryName".to_string(),
                variants: vec![
                    Variant {
                        name: "France".to_string(),
                        value: "France".to_string()
                    },
                    Variant {
                        name: "Germany".to_string(),
                        value: "Germany".to_string()
                    },
                    Variant {
                        name: "Italy".to_string(),
                        value: "Italy".to_string()
                    },
                    Variant {
                        name: "Spain".to_string(),
                        value: "Spain".to_string()
                    },
                    Variant {
                        name: "UnitedKingdom".to_string(),
                        value: "UnitedKingdom".to_string()
                    },
                    Variant {
                        name: "UnitedStates".to_string(),
                        value: "UnitedStates".to_string()
                    },
                ]
            }
        );
    }

    #[test]
    fn test_print_enum() {
        assert_eq!(
            StringEnum {
                identifier: "CountryName".to_string(),
                variants: vec![
                    Variant {
                        name: "France".to_string(),
                        value: "France".to_string()
                    },
                    Variant {
                        name: "Germany".to_string(),
                        value: "Germany".to_string()
                    },
                    Variant {
                        name: "Italy".to_string(),
                        value: "Italy".to_string()
                    },
                    Variant {
                        name: "Spain".to_string(),
                        value: "Spain".to_string()
                    },
                    Variant {
                        name: "UnitedKingdom".to_string(),
                        value: "UnitedKingdom".to_string()
                    },
                    Variant {
                        name: "UnitedStates".to_string(),
                        value: "UnitedStates".to_string()
                    },
                ]
            }
            .print(1),
            "    enum CountryName {
        France = \"France\",
        Germany = \"Germany\",
        Italy = \"Italy\",
        Spain = \"Spain\",
        UnitedKingdom = \"UnitedKingdom\",
        UnitedStates = \"UnitedStates\",
    }"
        );
    }
}
