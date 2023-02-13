use std::fmt::Display;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Any,
    Array {
        r#type: Box<Type>,
    },
    BigInt,
    Boolean,
    Function {
        arguments: Vec<(String, Type)>,
        return_type: Box<Type>,
    },
    Named {
        name: String,
        generics: Vec<String>,
    },
    Never,
    Null,
    Number,
    ObjectLiteral {
        // Order matters.
        properties: Vec<(String, Type)>,
    },
    String,
    Symbol,
    Tuple {
        types: Vec<Type>,
    },
    Undefined,
    Union {
        types: Vec<Type>,
    },
    Unknown,
    Void,
}

// TODO: Replace with pretty printer.
impl Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::Array { r#type } => write!(f, "Array<{type}>"),
            Self::BigInt => write!(f, "bigint"),
            Self::Boolean => write!(f, "boolean"),
            Self::Function {
                arguments,
                return_type,
            } => {
                write!(
                    f,
                    "({}) => {}",
                    arguments
                        .iter()
                        .map(|(name, r#type)| format!("{name}: {type}"))
                        .collect::<Vec<_>>()
                        .join(", "),
                    return_type
                )
            }
            Self::Named { name, generics } => {
                write!(
                    f,
                    "{}{}",
                    name,
                    if generics.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "<{}>",
                            generics
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    }
                )
            }
            Self::Never => write!(f, "never"),
            Self::Null => write!(f, "null"),
            Self::Number => write!(f, "number"),
            Self::ObjectLiteral { properties } => {
                write!(
                    f,
                    "{{ {} }}",
                    properties
                        .iter()
                        .map(|(name, r#type)| format!("{name}: {type}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::String => write!(f, "string"),
            Self::Symbol => write!(f, "symbol"),
            Self::Tuple { types } => {
                write!(
                    f,
                    "[{}]",
                    types
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
            Self::Undefined => write!(f, "undefined"),
            Self::Union { types } => {
                write!(
                    f,
                    "{}",
                    types
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" | ")
                )
            }
            Self::Unknown => write!(f, "unknown"),
            Self::Void => write!(f, "void"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Property {
    name: String,
    r#type: Type,
    optional: bool,
}

// TODO: Replace with pretty printer.
impl Display for Property {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name,
            r#type,
            optional,
        } = self;

        if *optional {
            write!(f, "{name}?: {type};")
        } else {
            write!(f, "{name}: {type};")
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Extend {
    name: String,
    generics: Vec<Extend>,
}

impl Display for Extend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { name, generics } = self;

        let generics = generics
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        if generics.is_empty() {
            write!(f, "{name}")
        } else {
            write!(f, "{name}<{generics}>")
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interface {
    name: String,
    extends: Vec<Extend>,
    // TODO: Support extending of parameters.
    parameters: Vec<String>,
    properties: Vec<Property>,
}

// TODO: Replace with pretty printer.
impl Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self {
            name,
            extends,
            parameters,
            properties,
        } = self;

        let extends = if extends.is_empty() {
            String::new()
        } else {
            format!(
                " extends {}",
                extends
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        let parameters = if parameters.is_empty() {
            String::new()
        } else {
            format!(
                "<{}>",
                parameters
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        };

        let properties = properties
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(" ");

        write!(
            f,
            "interface {name}{parameters}{extends} {{ {properties} }}"
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Enum {
    name: String,
    values: Vec<String>,
}

impl Display for Enum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let Self { name, values } = self;

        let values = values
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");

        write!(f, "enum {name} {{ {values} }}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_any() {
        assert_eq!(Type::Any.to_string(), "any");
    }

    #[test]
    fn test_display_array() {
        assert_eq!(
            Type::Array {
                r#type: Box::new(Type::Named {
                    name: "Partial".to_string(),
                    generics: vec!["Image".to_string()]
                }),
            }
            .to_string(),
            "Array<Partial<Image>>"
        );
    }

    #[test]
    fn test_display_bigint() {
        assert_eq!(Type::BigInt.to_string(), "bigint");
    }

    #[test]
    fn test_display_boolean() {
        assert_eq!(Type::Boolean.to_string(), "boolean");
    }

    #[test]
    fn test_display_function() {
        assert_eq!(
            Type::Function {
                arguments: vec![
                    (
                        "name".to_string(),
                        Type::Named {
                            name: "Partial".to_string(),
                            generics: vec!["Image".to_string(),]
                        }
                    ),
                    (
                        "countryName".to_string(),
                        Type::Named {
                            name: "CountryName".to_string(),
                            generics: vec![],
                        }
                    )
                ]
                .into_iter()
                .collect(),
                return_type: Box::new(Type::Named {
                    name: "String".to_string(),
                    generics: vec![]
                }),
            }
            .to_string(),
            "(name: Partial<Image>, countryName: CountryName) => String"
        );
    }

    #[test]
    fn test_display_named() {
        assert_eq!(
            Type::Named {
                name: "Partial".to_string(),
                generics: vec!["Image".to_string()]
            }
            .to_string(),
            "Partial<Image>"
        );
    }

    #[test]
    fn test_display_never() {
        assert_eq!(Type::Never.to_string(), "never");
    }

    #[test]
    fn test_display_null() {
        assert_eq!(Type::Null.to_string(), "null");
    }

    #[test]
    fn test_display_number() {
        assert_eq!(Type::Number.to_string(), "number");
    }

    #[test]
    fn test_display_object_literal() {
        assert_eq!(
            Type::ObjectLiteral {
                properties: vec![
                    (
                        "country".to_string(),
                        Type::ObjectLiteral {
                            properties: vec![
                                (
                                    "name".to_string(),
                                    Type::Named {
                                        name: "CountryName".to_string(),
                                        generics: vec![],
                                    }
                                ),
                                (
                                    "languages".to_string(),
                                    Type::Array {
                                        r#type: Box::new(Type::String)
                                    }
                                )
                            ]
                            .into_iter()
                            .collect(),
                        }
                    ),
                    (
                        "tags".to_string(),
                        Type::Array {
                            r#type: Box::new(Type::Named {
                                name: "Tag".to_string(),
                                generics: vec![],
                            })
                        }
                    )
                ]
                .into_iter()
                .collect(),
            }
            .to_string(),
            "{ country: { name: CountryName, languages: Array<string> }, tags: Array<Tag> }"
        );
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Type::String.to_string(), "string");
    }

    #[test]
    fn test_display_symbol() {
        assert_eq!(Type::Symbol.to_string(), "symbol");
    }

    #[test]
    fn test_display_tuple() {
        assert_eq!(
            Type::Tuple {
                types: vec![
                    Type::Named {
                        name: "CountryName".to_string(),
                        generics: vec![],
                    },
                    Type::String,
                    Type::Tuple {
                        types: vec![Type::Number, Type::String,]
                    }
                ]
            }
            .to_string(),
            "[CountryName, string, [number, string]]"
        );
    }

    #[test]
    fn test_display_undefined() {
        assert_eq!(Type::Undefined.to_string(), "undefined");
    }

    #[test]
    fn test_display_union() {
        assert_eq!(
            Type::Union {
                types: vec![
                    Type::Named {
                        name: "CountryName".to_string(),
                        generics: vec![],
                    },
                    Type::String,
                    Type::Tuple {
                        types: vec![Type::Number, Type::String,]
                    }
                ]
            }
            .to_string(),
            "CountryName | string | [number, string]"
        );
    }

    #[test]
    fn test_display_unknown() {
        assert_eq!(Type::Unknown.to_string(), "unknown");
    }

    #[test]
    fn test_display_void() {
        assert_eq!(Type::Void.to_string(), "void");
    }

    #[test]
    fn test_display_interface() {
        assert_eq!(
            Interface {
                name: "Image".to_string(),
                parameters: vec![
                    "T".to_string(),
                ],
                extends: vec![
                    Extend {
                    name: "Resource".to_string(),
                    generics: vec![
                        Extend {
                            name: "T".to_string(),
                            generics: vec![]
                        }
                    ]
                }],
                properties: vec![
                    Property {
                        name: "title".to_string(),
                        r#type: Type::String,
                        optional: false
                    },
                    Property {
                        name: "countryName".to_string(),
                        r#type: Type::Named {
                            name: "CountryName".to_string(),
                            generics: vec![],
                        },
                        optional: true
                    },
                    Property {
                        name: "tags".to_string(),
                        r#type: Type::Array {
                            r#type: Box::new(Type::Named {
                                name: "Tag".to_string(),
                                generics: vec![],
                            })
                        },
                        optional: false
                    }
                ]
            }
            .to_string(),
            "interface Image<T> extends Resource<T> { title: string; countryName?: CountryName; tags: Array<Tag>; }"
        );
    }

    #[test]
    fn test_display_enum() {
        assert_eq!(
            Enum {
                name: "CountryName".to_string(),
                values: vec![
                    "France".to_string(),
                    "Germany".to_string(),
                    "Italy".to_string(),
                    "Spain".to_string(),
                    "UnitedKingdom".to_string(),
                    "UnitedStates".to_string(),
                ]
            }
            .to_string(),
            "enum CountryName { France, Germany, Italy, Spain, UnitedKingdom, UnitedStates }"
        );
    }
}
