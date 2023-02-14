use {
    crate::ast::r#enum::Enum as AstEnum,
    std::fmt::Display,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variant {
    name: String,
    value: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    name: String,
    variants: Vec<Variant>,
}

// TODO: Replace with pretty printer.
impl Display for Enum {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self { name, variants } = self;

        let variants = variants
            .iter()
            .map(|Variant { name, value }| format!("    {name} = \"{value}\","))
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "enum {name} {{\n{variants}\n}}")
    }
}

impl From<AstEnum> for Enum {
    fn from(value: AstEnum) -> Self {
        let AstEnum { name, variants } = value;

        Self {
            name,
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
            Enum::from(AstEnum {
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
            Enum {
                name: "CountryName".to_string(),
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
    fn test_display_enum() {
        assert_eq!(
            Enum {
                name: "CountryName".to_string(),
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
            .to_string(),
            "\
enum CountryName {
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
