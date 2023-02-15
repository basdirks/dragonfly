use {
    crate::ast::r#type::{
        Basic as AstBasicType,
        Type as AstType,
    },
    std::fmt::Display,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    Any,
    BigInt,
    Boolean,
    Intrinsic,
    Never,
    Null,
    Number,
    Object,
    String,
    Symbol,
    Undefined,
    Unknown,
    Void,
}

impl Display for Keyword {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::BigInt => write!(f, "bigint"),
            Self::Boolean => write!(f, "boolean"),
            Self::Intrinsic => write!(f, "intrinsic"),
            Self::Never => write!(f, "never"),
            Self::Null => write!(f, "null"),
            Self::Number => write!(f, "number"),
            Self::Object => write!(f, "object"),
            Self::String => write!(f, "string"),
            Self::Symbol => write!(f, "symbol"),
            Self::Undefined => write!(f, "undefined"),
            Self::Unknown => write!(f, "unknown"),
            Self::Void => write!(f, "void"),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    Array(Box<Type>),
    Keyword(Keyword),
    Function {
        arguments: Vec<(String, Type)>,
        return_type: Box<Type>,
    },
    TypeReference {
        identifier: String,
        type_references: Vec<Type>,
    },
    Intersection(Vec<Type>),
    ObjectLiteral(Vec<(String, Type)>),
    Tuple(Vec<Type>),
    Union(Vec<Type>),
}

// TODO: Replace with pretty printer.
impl Display for Type {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Keyword(keyword) => write!(f, "{keyword}"),
            Self::Array(r#type) => write!(f, "Array<{type}>"),
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
            Self::TypeReference {
                identifier,
                type_references,
            } => {
                write!(
                    f,
                    "{}{}",
                    identifier,
                    if type_references.is_empty() {
                        String::new()
                    } else {
                        format!(
                            "<{}>",
                            type_references
                                .iter()
                                .map(ToString::to_string)
                                .collect::<Vec<_>>()
                                .join(", ")
                        )
                    }
                )
            }
            Self::Intersection(types) => {
                write!(
                    f,
                    "{}",
                    types
                        .iter()
                        .map(ToString::to_string)
                        .collect::<Vec<_>>()
                        .join(" & ")
                )
            }
            Self::ObjectLiteral(properties) => {
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
            Self::Tuple(types) => {
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
            Self::Union(types) => {
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
        }
    }
}

impl From<AstBasicType> for Type {
    fn from(value: AstBasicType) -> Self {
        match value {
            AstBasicType::Boolean => Self::Keyword(Keyword::Boolean),
            // This is quite tragic.
            AstBasicType::Float | AstBasicType::Int => {
                Self::Keyword(Keyword::Number)
            }
            AstBasicType::String => Self::Keyword(Keyword::String),
            AstBasicType::Identifier(identifier) => {
                Self::TypeReference {
                    identifier,
                    type_references: vec![],
                }
            }
        }
    }
}

impl From<AstType> for Type {
    fn from(r#type: AstType) -> Self {
        match r#type {
            AstType::One(r#type) => r#type.into(),
            AstType::Array(r#type) => Self::Array(Box::new(r#type.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_any() {
        assert_eq!(Type::Keyword(Keyword::Any).to_string(), "any");
    }

    #[test]
    fn test_display_array() {
        assert_eq!(
            Type::Array(Box::new(Type::TypeReference {
                identifier: "Partial".to_string(),
                type_references: vec![Type::TypeReference {
                    identifier: "Image".to_string(),
                    type_references: vec![],
                }]
            }),)
            .to_string(),
            "Array<Partial<Image>>"
        );
    }

    #[test]
    fn test_display_bigint() {
        assert_eq!(Type::Keyword(Keyword::BigInt).to_string(), "bigint");
    }

    #[test]
    fn test_display_boolean() {
        assert_eq!(Type::Keyword(Keyword::Boolean).to_string(), "boolean");
    }

    #[test]
    fn test_display_function() {
        assert_eq!(
            Type::Function {
                arguments: vec![
                    (
                        "name".to_string(),
                        Type::TypeReference {
                            identifier: "Partial".to_string(),
                            type_references: vec![Type::TypeReference {
                                identifier: "Image".to_string(),
                                type_references: vec![],
                            }]
                        }
                    ),
                    (
                        "countryName".to_string(),
                        Type::TypeReference {
                            identifier: "CountryName".to_string(),
                            type_references: vec![],
                        }
                    )
                ]
                .into_iter()
                .collect(),
                return_type: Box::new(Type::TypeReference {
                    identifier: "String".to_string(),
                    type_references: vec![]
                }),
            }
            .to_string(),
            "(name: Partial<Image>, countryName: CountryName) => String"
        );
    }

    #[test]
    fn test_display_named() {
        assert_eq!(
            Type::TypeReference {
                identifier: "Partial".to_string(),
                type_references: vec![Type::TypeReference {
                    identifier: "Image".to_string(),
                    type_references: vec![],
                }]
            }
            .to_string(),
            "Partial<Image>"
        );
    }

    #[test]
    fn test_display_never() {
        assert_eq!(Type::Keyword(Keyword::Never).to_string(), "never");
    }

    #[test]
    fn test_display_null() {
        assert_eq!(Type::Keyword(Keyword::Null).to_string(), "null");
    }

    #[test]
    fn test_display_number() {
        assert_eq!(Type::Keyword(Keyword::Number).to_string(), "number");
    }

    #[test]
    fn test_display_object_literal() {
        assert_eq!(
            Type::ObjectLiteral(
                vec![
                    (
                        "country".to_string(),
                        Type::ObjectLiteral(
                            vec![
                                (
                                    "name".to_string(),
                                    Type::TypeReference {
                                        identifier: "CountryName".to_string(),
                                        type_references: vec![],
                                    }
                                ),
                                (
                                    "languages".to_string(),
                                    Type::Array(Box::new(Type::Keyword(
                                        Keyword::String
                                    )))
                                )
                            ]
                            .into_iter()
                            .collect(),
                        )
                    ),
                    (
                        "tags".to_string(),
                        Type::Array(Box::new(Type::TypeReference {
                            identifier: "Tag".to_string(),
                            type_references: vec![],
                        }))
                    )
                ]
                .into_iter()
                .collect(),
            )
            .to_string(),
            "{ country: { name: CountryName, languages: Array<string> }, \
             tags: Array<Tag> }"
        );
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Type::Keyword(Keyword::String).to_string(), "string");
    }

    #[test]
    fn test_display_symbol() {
        assert_eq!(Type::Keyword(Keyword::Symbol).to_string(), "symbol");
    }

    #[test]
    fn test_display_tuple() {
        assert_eq!(
            Type::Tuple(vec![
                Type::TypeReference {
                    identifier: "CountryName".to_string(),
                    type_references: vec![],
                },
                Type::Keyword(Keyword::String),
                Type::Tuple(vec![
                    Type::Keyword(Keyword::Number),
                    Type::Keyword(Keyword::String)
                ])
            ])
            .to_string(),
            "[CountryName, string, [number, string]]"
        );
    }

    #[test]
    fn test_display_undefined() {
        assert_eq!(Type::Keyword(Keyword::Undefined).to_string(), "undefined");
    }

    #[test]
    fn test_display_union() {
        assert_eq!(
            Type::Union(vec![
                Type::TypeReference {
                    identifier: "CountryName".to_string(),
                    type_references: vec![],
                },
                Type::Keyword(Keyword::String),
                Type::Tuple(vec![
                    Type::Keyword(Keyword::Number),
                    Type::Keyword(Keyword::String)
                ])
            ])
            .to_string(),
            "CountryName | string | [number, string]"
        );
    }

    #[test]
    fn test_display_unknown() {
        assert_eq!(Type::Keyword(Keyword::Unknown).to_string(), "unknown");
    }

    #[test]
    fn test_display_void() {
        assert_eq!(Type::Keyword(Keyword::Void).to_string(), "void");
    }

    #[test]
    fn test_from_ast_primitive_boolean() {
        assert_eq!(
            Type::from(AstBasicType::Boolean),
            Type::Keyword(Keyword::Boolean)
        );
    }

    #[test]
    fn test_from_ast_primitive_float() {
        assert_eq!(
            Type::from(AstBasicType::Float),
            Type::Keyword(Keyword::Number)
        );
    }

    #[test]
    fn test_from_ast_primitive_integer() {
        assert_eq!(
            Type::from(AstBasicType::Int),
            Type::Keyword(Keyword::Number)
        );
    }

    #[test]
    fn test_from_ast_primitive_string() {
        assert_eq!(
            Type::from(AstBasicType::String),
            Type::Keyword(Keyword::String)
        );
    }

    #[test]
    fn test_from_ast_primitive_identifier() {
        assert_eq!(
            Type::from(AstBasicType::Identifier("Image".to_string())),
            Type::TypeReference {
                identifier: "Image".to_string(),
                type_references: vec![]
            }
        );
    }
}
