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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

impl From<&AstEnum> for StringEnum {
    fn from(value: &AstEnum) -> Self {
        Self::from(value.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ast_enum() {
        assert_eq!(
            StringEnum::from(AstEnum {
                name: "CountryName".to_owned(),
                variants: vec![
                    "France".to_owned(),
                    "Germany".to_owned(),
                    "Italy".to_owned(),
                    "Spain".to_owned(),
                    "UnitedKingdom".to_owned(),
                    "UnitedStates".to_owned(),
                ]
            }),
            StringEnum {
                identifier: "CountryName".to_owned(),
                variants: vec![
                    Variant {
                        name: "France".to_owned(),
                        value: "France".to_owned()
                    },
                    Variant {
                        name: "Germany".to_owned(),
                        value: "Germany".to_owned()
                    },
                    Variant {
                        name: "Italy".to_owned(),
                        value: "Italy".to_owned()
                    },
                    Variant {
                        name: "Spain".to_owned(),
                        value: "Spain".to_owned()
                    },
                    Variant {
                        name: "UnitedKingdom".to_owned(),
                        value: "UnitedKingdom".to_owned()
                    },
                    Variant {
                        name: "UnitedStates".to_owned(),
                        value: "UnitedStates".to_owned()
                    },
                ]
            }
        );
    }

    #[test]
    fn test_print_enum() {
        assert_eq!(
            StringEnum {
                identifier: "CountryName".to_owned(),
                variants: vec![
                    Variant {
                        name: "France".to_owned(),
                        value: "France".to_owned()
                    },
                    Variant {
                        name: "Germany".to_owned(),
                        value: "Germany".to_owned()
                    },
                    Variant {
                        name: "Italy".to_owned(),
                        value: "Italy".to_owned()
                    },
                    Variant {
                        name: "Spain".to_owned(),
                        value: "Spain".to_owned()
                    },
                    Variant {
                        name: "UnitedKingdom".to_owned(),
                        value: "UnitedKingdom".to_owned()
                    },
                    Variant {
                        name: "UnitedStates".to_owned(),
                        value: "UnitedStates".to_owned()
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

    #[test]
    fn test_display_variant() {
        assert_eq!(
            Variant {
                name: "France".to_owned(),
                value: "France".to_owned()
            }
            .to_string(),
            "France = \"France\""
        );
    }

    #[test]
    fn test_enum_from_ast_enum() {
        assert_eq!(
            StringEnum::from(AstEnum {
                name: "CountryName".to_owned(),
                variants: vec![
                    "France".to_owned(),
                    "Germany".to_owned(),
                    "Italy".to_owned(),
                    "Spain".to_owned(),
                    "UnitedKingdom".to_owned(),
                    "UnitedStates".to_owned(),
                ]
            }),
            StringEnum {
                identifier: "CountryName".to_owned(),
                variants: vec![
                    Variant {
                        name: "France".to_owned(),
                        value: "France".to_owned()
                    },
                    Variant {
                        name: "Germany".to_owned(),
                        value: "Germany".to_owned()
                    },
                    Variant {
                        name: "Italy".to_owned(),
                        value: "Italy".to_owned()
                    },
                    Variant {
                        name: "Spain".to_owned(),
                        value: "Spain".to_owned()
                    },
                    Variant {
                        name: "UnitedKingdom".to_owned(),
                        value: "UnitedKingdom".to_owned()
                    },
                    Variant {
                        name: "UnitedStates".to_owned(),
                        value: "UnitedStates".to_owned()
                    },
                ]
            }
        );
    }
}
