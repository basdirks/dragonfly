use {
    crate::{
        ast::r#type::{
            Scalar as AstScalarType,
            Type as AstType,
        },
        generator::printer::common::{
            comma_separated,
            separated,
        },
    },
    std::fmt::Display,
};

/// A JavaScript literal.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Literal {
    /// A BigInt literal: a number followed by `n`.
    BigInt(String),
    /// A boolean literal: `true` or `false`.
    Boolean(bool),
    /// A number literal.
    Number(String),
    /// A string literal: characters surrounded by double quotes.
    String(String),
}

impl Display for Literal {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::BigInt(value) => write!(f, "{value}n"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "\"{value}\""),
        }
    }
}

/// A TypeScript type keyword.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Keyword {
    /// The `any` type.
    Any,
    /// The `bigint` type.
    BigInt,
    /// The `boolean` type.
    Boolean,
    /// The `intrinsic` type.
    Intrinsic,
    /// The `never` type.
    Never,
    /// The `null` type.
    Null,
    /// The `number` type.
    Number,
    /// The `object` type.
    Object,
    /// The `string` type.
    String,
    /// The `symbol` type.
    Symbol,
    /// An `undefined` type.
    Undefined,
    /// An `unknown` type.
    Unknown,
    /// The `void` type.
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

/// A function argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionArgument {
    /// The name of the argument.
    pub name: String,
    /// The type of the argument.
    pub r#type: Type,
}

impl Display for FunctionArgument {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.r#type)
    }
}

/// An object literal property.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObjectLiteralProperty {
    /// The name of the property.
    pub name: String,
    /// The type of the property.
    pub r#type: Type,
}

impl Display for ObjectLiteralProperty {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.r#type)
    }
}

/// A TypeScript type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    /// An array type.
    Array(Box<Type>),
    /// A function type.
    Function {
        /// The function arguments.
        arguments: Vec<FunctionArgument>,
        /// The return type.
        return_type: Box<Type>,
    },
    /// An intersection of types.
    Intersection(Vec<Type>),
    /// A keyword type.
    Keyword(Keyword),
    /// A type literal.
    Literal(Literal),
    /// An object literal.
    ObjectLiteral(Vec<ObjectLiteralProperty>),
    /// A tuple of types.
    Tuple(Vec<Type>),
    /// A type reference.
    TypeReference {
        /// The name of the type.
        identifier: String,
        /// The type arguments.
        type_references: Vec<Type>,
    },
    /// A union of types.
    Union(Vec<Type>),
}

impl Display for Type {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Array(r#type) => write!(f, "Array<{type}>"),
            Self::Function {
                arguments,
                return_type,
            } => {
                write!(f, "({}) => {}", comma_separated(arguments), return_type)
            }
            Self::Keyword(keyword) => write!(f, "{keyword}"),
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
                        format!("<{}>", comma_separated(type_references))
                    }
                )
            }
            Self::Intersection(types) => {
                write!(f, "{}", separated(types, " & "))
            }
            Self::Literal(literal) => write!(f, "{literal}"),
            Self::ObjectLiteral(properties) => {
                write!(f, "{{ {} }}", comma_separated(properties))
            }
            Self::Tuple(types) => {
                write!(f, "[{}]", comma_separated(types))
            }
            Self::Union(types) => {
                write!(f, "{}", separated(types, " | "))
            }
        }
    }
}

impl From<AstScalarType> for Type {
    fn from(value: AstScalarType) -> Self {
        match value {
            AstScalarType::Boolean => Self::Keyword(Keyword::Boolean),
            AstScalarType::DateTime => {
                Self::TypeReference {
                    identifier: "Date".to_string(),
                    type_references: vec![],
                }
            }
            AstScalarType::Float => Self::Keyword(Keyword::Number),
            // This is not ideal, but an integer is an integer.
            AstScalarType::Int => Self::Keyword(Keyword::BigInt),
            AstScalarType::String => Self::Keyword(Keyword::String),
            AstScalarType::Reference(identifier) => {
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
            AstType::Scalar(r#type) => r#type.into(),
            AstType::Array(r#type) => Self::Array(Box::new(r#type.into())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_any() {
        assert_eq!(Keyword::Any.to_string(), "any");
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
            }))
            .to_string(),
            "Array<Partial<Image>>"
        );
    }

    #[test]
    fn test_display_bigint() {
        assert_eq!(Keyword::BigInt.to_string(), "bigint");
    }

    #[test]
    fn test_display_boolean() {
        assert_eq!(Keyword::Boolean.to_string(), "boolean");
    }

    #[test]
    fn test_display_function() {
        assert_eq!(
            Type::Function {
                arguments: vec![
                    FunctionArgument {
                        name: "name".to_string(),
                        r#type: Type::TypeReference {
                            identifier: "Partial".to_string(),
                            type_references: vec![Type::TypeReference {
                                identifier: "Image".to_string(),
                                type_references: vec![],
                            }]
                        }
                    },
                    FunctionArgument {
                        name: "countryName".to_string(),
                        r#type: Type::TypeReference {
                            identifier: "CountryName".to_string(),
                            type_references: vec![],
                        }
                    }
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
    fn test_display_keyword() {
        assert_eq!(Type::Keyword(Keyword::Any).to_string(), "any");
        assert_eq!(Type::Keyword(Keyword::BigInt).to_string(), "bigint");
        assert_eq!(Type::Keyword(Keyword::Boolean).to_string(), "boolean");
        assert_eq!(Type::Keyword(Keyword::Intrinsic).to_string(), "intrinsic");
        assert_eq!(Type::Keyword(Keyword::Never).to_string(), "never");
        assert_eq!(Type::Keyword(Keyword::Null).to_string(), "null");
        assert_eq!(Type::Keyword(Keyword::Number).to_string(), "number");
        assert_eq!(Type::Keyword(Keyword::Object).to_string(), "object");
        assert_eq!(Type::Keyword(Keyword::String).to_string(), "string");
        assert_eq!(Type::Keyword(Keyword::Symbol).to_string(), "symbol");
        assert_eq!(Type::Keyword(Keyword::Undefined).to_string(), "undefined");
        assert_eq!(Type::Keyword(Keyword::Unknown).to_string(), "unknown");
        assert_eq!(Type::Keyword(Keyword::Void).to_string(), "void");
    }

    #[test]
    fn test_display_literal() {
        assert_eq!(
            Type::Literal(Literal::BigInt("1".to_string())).to_string(),
            "1n"
        );

        assert_eq!(Type::Literal(Literal::Boolean(true)).to_string(), "true");
        assert_eq!(Type::Literal(Literal::Boolean(false)).to_string(), "false");

        assert_eq!(
            Type::Literal(Literal::Number("1.0".to_string())).to_string(),
            "1.0"
        );

        assert_eq!(
            Type::Literal(Literal::Number("1.5".to_string())).to_string(),
            "1.5"
        );

        assert_eq!(
            Type::Literal(Literal::String("hello".to_string())).to_string(),
            "\"hello\""
        );
    }

    #[test]
    fn test_display_intersection() {
        assert_eq!(
            Type::Intersection(vec![
                Type::TypeReference {
                    identifier: "Partial".to_string(),
                    type_references: vec![Type::TypeReference {
                        identifier: "T".to_string(),
                        type_references: vec![],
                    }]
                },
                Type::TypeReference {
                    identifier: "Partial".to_string(),
                    type_references: vec![Type::TypeReference {
                        identifier: "U".to_string(),
                        type_references: vec![],
                    }]
                },
            ])
            .to_string(),
            "Partial<T> & Partial<U>"
        );
    }

    #[test]
    fn test_display_intrinsic() {
        assert_eq!(Keyword::Intrinsic.to_string(), "intrinsic");
    }

    #[test]
    fn test_display_never() {
        assert_eq!(Keyword::Never.to_string(), "never");
    }

    #[test]
    fn test_display_null() {
        assert_eq!(Keyword::Null.to_string(), "null");
    }

    #[test]
    fn test_display_number() {
        assert_eq!(Keyword::Number.to_string(), "number");
    }

    #[test]
    fn test_display_object_literal() {
        assert_eq!(
            Type::ObjectLiteral(
                vec![
                    ObjectLiteralProperty {
                        name: "country".to_string(),
                        r#type: Type::ObjectLiteral(
                            vec![
                                ObjectLiteralProperty {
                                    name: "name".to_string(),
                                    r#type: Type::TypeReference {
                                        identifier: "CountryName".to_string(),
                                        type_references: vec![],
                                    }
                                },
                                ObjectLiteralProperty {
                                    name: "languages".to_string(),
                                    r#type: Type::Array(Box::new(
                                        Type::Keyword(Keyword::String)
                                    ))
                                }
                            ]
                            .into_iter()
                            .collect(),
                        )
                    },
                    ObjectLiteralProperty {
                        name: "tags".to_string(),
                        r#type: Type::Array(Box::new(Type::TypeReference {
                            identifier: "Tag".to_string(),
                            type_references: vec![],
                        }))
                    }
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
        assert_eq!(Keyword::String.to_string(), "string");
    }

    #[test]
    fn test_display_symbol() {
        assert_eq!(Keyword::Symbol.to_string(), "symbol");
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
    fn test_display_type_reference() {
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
    fn test_display_undefined() {
        assert_eq!(Keyword::Undefined.to_string(), "undefined");
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
        assert_eq!(Keyword::Unknown.to_string(), "unknown");
    }

    #[test]
    fn test_display_literal_bigint() {
        assert_eq!(
            Type::Literal(Literal::BigInt("1".to_string())).to_string(),
            "1n"
        );
    }

    #[test]
    fn test_display_literal_boolean() {
        assert_eq!(Type::Literal(Literal::Boolean(true)).to_string(), "true");
        assert_eq!(Type::Literal(Literal::Boolean(false)).to_string(), "false");
    }

    #[test]
    fn test_display_literal_number() {
        assert_eq!(
            Type::Literal(Literal::Number("1.0".to_string())).to_string(),
            "1.0"
        );

        assert_eq!(
            Type::Literal(Literal::Number("1.1".to_string())).to_string(),
            "1.1"
        );
    }

    #[test]
    fn test_display_literal_string() {
        assert_eq!(
            Type::Literal(Literal::String("foo".to_string())).to_string(),
            "\"foo\""
        );
    }

    #[test]
    fn test_display_void() {
        assert_eq!(Keyword::Void.to_string(), "void");
    }

    #[test]
    fn test_from_ast_scalar_boolean() {
        assert_eq!(
            Type::from(AstScalarType::Boolean),
            Type::Keyword(Keyword::Boolean)
        );
    }

    #[test]
    fn test_from_ast_scalar_date_time() {
        assert_eq!(
            Type::from(AstScalarType::DateTime),
            Type::TypeReference {
                identifier: "Date".to_string(),
                type_references: vec![]
            }
        );
    }

    #[test]
    fn test_from_ast_scalar_float() {
        assert_eq!(
            Type::from(AstScalarType::Float),
            Type::Keyword(Keyword::Number)
        );
    }

    #[test]
    fn test_from_ast_scalar_integer() {
        assert_eq!(
            Type::from(AstScalarType::Int),
            Type::Keyword(Keyword::BigInt)
        );
    }

    #[test]
    fn test_from_ast_scalar_string() {
        assert_eq!(
            Type::from(AstScalarType::String),
            Type::Keyword(Keyword::String)
        );
    }

    #[test]
    fn test_from_ast_scalar_identifier() {
        assert_eq!(
            Type::from(AstScalarType::Reference("Image".to_string())),
            Type::TypeReference {
                identifier: "Image".to_string(),
                type_references: vec![]
            }
        );
    }
}
