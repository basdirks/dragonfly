use {
    crate::ast::r#type::{
        Basic as AstBasicType,
        Type as AstType,
    },
    std::fmt::Display,
};

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
    Identifier {
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
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
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
            Self::Identifier { name, generics } => {
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

impl From<AstBasicType> for Type {
    fn from(value: AstBasicType) -> Self {
        match value {
            AstBasicType::Boolean => Self::Boolean,
            AstBasicType::Float | AstBasicType::Int => Self::Number,
            AstBasicType::String => Self::String,
            AstBasicType::Identifier(name) => {
                Self::Identifier {
                    name,
                    generics: Vec::new(),
                }
            }
        }
    }
}

impl From<AstType> for Type {
    fn from(r#type: AstType) -> Self {
        match r#type {
            AstType::One(r#type) => r#type.into(),
            AstType::Array(r#type) => {
                Self::Array {
                    r#type: Box::new(r#type.into()),
                }
            }
        }
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
                r#type: Box::new(Type::Identifier {
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
                        Type::Identifier {
                            name: "Partial".to_string(),
                            generics: vec!["Image".to_string(),]
                        }
                    ),
                    (
                        "countryName".to_string(),
                        Type::Identifier {
                            name: "CountryName".to_string(),
                            generics: vec![],
                        }
                    )
                ]
                .into_iter()
                .collect(),
                return_type: Box::new(Type::Identifier {
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
            Type::Identifier {
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
                                    Type::Identifier {
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
                            r#type: Box::new(Type::Identifier {
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
            "{ country: { name: CountryName, languages: Array<string> }, \
             tags: Array<Tag> }"
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
                    Type::Identifier {
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
                    Type::Identifier {
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
    fn test_from_ast_primitive_boolean() {
        assert_eq!(Type::from(AstBasicType::Boolean), Type::Boolean);
    }

    #[test]
    fn test_from_ast_primitive_float() {
        assert_eq!(Type::from(AstBasicType::Float), Type::Number);
    }

    #[test]
    fn test_from_ast_primitive_integer() {
        assert_eq!(Type::from(AstBasicType::Int), Type::Number);
    }

    #[test]
    fn test_from_ast_primitive_string() {
        assert_eq!(Type::from(AstBasicType::String), Type::String);
    }

    #[test]
    fn test_from_ast_primitive_identifier() {
        assert_eq!(
            Type::from(AstBasicType::Identifier("Image".to_string())),
            Type::Identifier {
                name: "Image".to_string(),
                generics: vec![]
            }
        );
    }
}
