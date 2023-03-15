pub use {
    function_argument::FunctionArgument,
    keyword::Keyword,
    literal::Literal,
    object_literal_property::ObjectLiteralProperty,
};
use {
    ir,
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A function argument.
pub mod function_argument;
/// A TypeScript type keyword.
pub mod keyword;
/// Type literals.
pub mod literal;
/// An object literal property.
pub mod object_literal_property;

/// A TypeScript type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Type<'a> {
    /// An array type.
    Array(Box<Type<'a>>),
    /// A function type.
    Function {
        /// The function arguments.
        arguments: Vec<FunctionArgument<'a>>,
        /// The return type.
        return_type: Box<Type<'a>>,
    },
    /// An intersection of types.
    Intersection(Vec<Type<'a>>),
    /// A keyword type.
    Keyword(Keyword),
    /// A type literal.
    Literal(Literal),
    /// An object literal.
    ObjectLiteral(Vec<ObjectLiteralProperty<'a>>),
    /// A tuple of types.
    Tuple(Vec<Type<'a>>),
    /// A type reference.
    TypeReference {
        /// The name of the type.
        identifier: Cow<'a, str>,
        /// The type arguments.
        type_arguments: Vec<Type<'a>>,
    },
    /// A union of types.
    Union(Vec<Type<'a>>),
}

impl From<ir::Type> for Type<'_> {
    fn from(r#type: ir::Type) -> Self {
        Self::TypeReference {
            identifier: match r#type {
                ir::Type::Boolean => "boolean".into(),
                ir::Type::DateTime => "Date".into(),
                ir::Type::Float | ir::Type::Int => "number".into(),
                ir::Type::String => "string".into(),
            },
            type_arguments: Vec::new(),
        }
    }
}

impl<'a> From<ir::model::EnumRelation<'a>> for Type<'a> {
    fn from(
        ir::model::EnumRelation { name, cardinality }: ir::model::EnumRelation<
            'a,
        >
    ) -> Self {
        match cardinality {
            ir::Cardinality::One => {
                Self::TypeReference {
                    identifier: name,
                    type_arguments: Vec::new(),
                }
            }
            ir::Cardinality::Many => {
                Self::Array(Box::new(Self::TypeReference {
                    identifier: name,
                    type_arguments: Vec::new(),
                }))
            }
        }
    }
}

impl<'a> From<ir::model::Field<'a>> for Type<'a> {
    fn from(
        ir::model::Field {
            r#type,
            cardinality,
            ..
        }: ir::model::Field
    ) -> Self {
        match cardinality {
            ir::Cardinality::One => r#type.into(),
            ir::Cardinality::Many => Self::Array(Box::new(r#type.into())),
        }
    }
}

impl PrintInline for Type<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        match self {
            Self::Array(r#type) => {
                write!(f, "Array<")?;
                r#type.print(f)?;
                write!(f, ">")
            }
            Self::Function {
                arguments,
                return_type,
            } => {
                write!(f, "(")?;
                PrintInline::intercalate(arguments.clone(), f, ", ")?;
                write!(f, ") => ")?;
                return_type.print(f)
            }
            Self::Keyword(keyword) => keyword.print(f),
            Self::TypeReference {
                identifier,
                type_arguments: type_references,
            } => {
                write!(f, "{identifier}")?;

                if !type_references.is_empty() {
                    write!(f, "<")?;
                    PrintInline::intercalate(type_references.clone(), f, ", ")?;
                    write!(f, ">")?;
                }

                Ok(())
            }
            Self::Intersection(types) => {
                PrintInline::intercalate(types.clone(), f, " & ")
            }
            Self::Literal(literal) => literal.print(f),
            Self::ObjectLiteral(properties) => {
                write!(f, "{{ ")?;
                PrintInline::intercalate(properties.clone(), f, ", ")?;
                write!(f, " }}")
            }
            Self::Tuple(types) => {
                write!(f, "[")?;
                PrintInline::intercalate(types.clone(), f, ", ")?;
                write!(f, "]")
            }
            Self::Union(types) => {
                PrintInline::intercalate(types.clone(), f, " | ")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ir_enum_relation_one() {
        assert_eq!(
            Type::from(ir::model::EnumRelation {
                name: "foo".into(),
                cardinality: ir::Cardinality::One,
            }),
            Type::TypeReference {
                identifier: "foo".into(),
                type_arguments: Vec::new()
            }
        );
    }

    #[test]
    fn test_from_ir_enum_relation_many() {
        assert_eq!(
            Type::from(ir::model::EnumRelation {
                name: "foo".into(),
                cardinality: ir::Cardinality::Many,
            }),
            Type::Array(Box::new(Type::TypeReference {
                identifier: "foo".into(),
                type_arguments: Vec::new()
            }))
        );
    }

    #[test]
    fn test_print_function() {
        let function = Type::Function {
            arguments: vec![FunctionArgument {
                name: "foo".into(),
                r#type: Type::Array(Box::new(Type::Keyword(Keyword::Number))),
            }],
            return_type: Box::new(Type::Keyword(Keyword::String)),
        };

        let mut f = Vec::new();

        function.print(&mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "(foo: Array<number>) => string"
        );
    }

    #[test]
    fn test_print_literal_bigint() {
        let literal = Type::Literal(Literal::BigInt("1".into()));
        let mut f = Vec::new();

        literal.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "1n");
    }

    #[test]
    fn test_print_literal_boolean() {
        let keyword = Type::Literal(Literal::Boolean(true));
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "true");
    }

    #[test]
    fn test_print_literal_number() {
        let literal = Type::Literal(Literal::Number("1.0".into()));
        let mut f = Vec::new();

        literal.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "1.0");
    }

    #[test]
    fn test_print_literal_string() {
        let literal = Type::Literal(Literal::String("hello".into()));
        let mut f = Vec::new();

        literal.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "\"hello\"");
    }

    #[test]
    fn test_print_intersection() {
        let intersection = Type::Intersection(vec![
            Type::TypeReference {
                identifier: "Partial".into(),
                type_arguments: vec![Type::TypeReference {
                    identifier: "T".into(),
                    type_arguments: Vec::new(),
                }],
            },
            Type::TypeReference {
                identifier: "Partial".into(),
                type_arguments: vec![Type::TypeReference {
                    identifier: "U".into(),
                    type_arguments: Vec::new(),
                }],
            },
        ]);

        let mut f = Vec::new();

        intersection.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Partial<T> & Partial<U>");
    }

    #[test]
    fn test_print_object_literal() {
        let object_literal = Type::ObjectLiteral(
            vec![
                ObjectLiteralProperty {
                    name: "country".into(),
                    r#type: Type::ObjectLiteral(
                        vec![
                            ObjectLiteralProperty {
                                name: "name".into(),
                                r#type: Type::TypeReference {
                                    identifier: "CountryName".into(),
                                    type_arguments: Vec::new(),
                                },
                            },
                            ObjectLiteralProperty {
                                name: "languages".into(),
                                r#type: Type::Array(Box::new(Type::Keyword(
                                    Keyword::String,
                                ))),
                            },
                        ]
                        .into_iter()
                        .collect(),
                    ),
                },
                ObjectLiteralProperty {
                    name: "tags".into(),
                    r#type: Type::Array(Box::new(Type::TypeReference {
                        identifier: "Tag".into(),
                        type_arguments: Vec::new(),
                    })),
                },
            ]
            .into_iter()
            .collect(),
        );

        let mut f = Vec::new();

        object_literal.print(&mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "{ country: { name: CountryName, languages: Array<string> }, \
             tags: Array<Tag> }"
        );
    }

    #[test]
    fn test_print_tuple() {
        let tuple = Type::Tuple(vec![
            Type::TypeReference {
                identifier: "CountryName".into(),
                type_arguments: Vec::new(),
            },
            Type::Keyword(Keyword::String),
            Type::Tuple(vec![
                Type::Keyword(Keyword::Number),
                Type::Keyword(Keyword::String),
            ]),
        ]);

        let mut f = Vec::new();

        tuple.print(&mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "[CountryName, string, [number, string]]"
        );
    }

    #[test]
    fn test_print_type_reference() {
        let type_reference = Type::TypeReference {
            identifier: "Partial".into(),
            type_arguments: vec![Type::TypeReference {
                identifier: "Image".into(),
                type_arguments: Vec::new(),
            }],
        };

        let mut f = Vec::new();

        type_reference.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Partial<Image>");
    }

    #[test]
    fn test_print_union() {
        let union = Type::Union(vec![
            Type::TypeReference {
                identifier: "CountryName".into(),
                type_arguments: Vec::new(),
            },
            Type::Keyword(Keyword::String),
            Type::Tuple(vec![
                Type::Keyword(Keyword::Number),
                Type::Keyword(Keyword::String),
            ]),
        ]);

        let mut f = Vec::new();

        union.print(&mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "CountryName | string | [number, string]"
        );
    }
}
