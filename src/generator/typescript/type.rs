use {
    super::literal::Literal,
    crate::{
        generator::printer::{
            comma_separated,
            separated,
        },
        ir,
    },
    std::fmt::Display,
};

/// A TypeScript type keyword.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
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

impl Keyword {
    /// Create a new `any` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::any(), Type::Keyword(Keyword::Any));
    /// ```
    #[must_use]
    pub const fn any() -> Type {
        Type::Keyword(Self::Any)
    }

    /// Create a new `bigint` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::bigint(), Type::Keyword(Keyword::BigInt));
    /// ```
    #[must_use]
    pub const fn bigint() -> Type {
        Type::Keyword(Self::BigInt)
    }

    /// Create a new `boolean` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::boolean(), Type::Keyword(Keyword::Boolean));
    /// ```
    #[must_use]
    pub const fn boolean() -> Type {
        Type::Keyword(Self::Boolean)
    }

    /// Create a new `intrinsic` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::intrinsic(), Type::Keyword(Keyword::Intrinsic));
    /// ```
    #[must_use]
    pub const fn intrinsic() -> Type {
        Type::Keyword(Self::Intrinsic)
    }

    /// Create a new `never` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::never(), Type::Keyword(Keyword::Never));
    /// ```
    #[must_use]
    pub const fn never() -> Type {
        Type::Keyword(Self::Never)
    }

    /// Create a new `null` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::null(), Type::Keyword(Keyword::Null));
    /// ```
    #[must_use]
    pub const fn null() -> Type {
        Type::Keyword(Self::Null)
    }

    /// Create a new `number` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::number(), Type::Keyword(Keyword::Number));
    /// ```
    #[must_use]
    pub const fn number() -> Type {
        Type::Keyword(Self::Number)
    }

    /// Create a new `object` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::object(), Type::Keyword(Keyword::Object));
    /// ```
    #[must_use]
    pub const fn object() -> Type {
        Type::Keyword(Self::Object)
    }

    /// Create a new `string` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::string(), Type::Keyword(Keyword::String));
    /// ```
    #[must_use]
    pub const fn string() -> Type {
        Type::Keyword(Self::String)
    }

    /// Create a new `symbol` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::symbol(), Type::Keyword(Keyword::Symbol));
    /// ```
    #[must_use]
    pub const fn symbol() -> Type {
        Type::Keyword(Self::Symbol)
    }

    /// Create a new `undefined` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::undefined(), Type::Keyword(Keyword::Undefined));
    /// ```
    #[must_use]
    pub const fn undefined() -> Type {
        Type::Keyword(Self::Undefined)
    }

    /// Create a new `unknown` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::unknown(), Type::Keyword(Keyword::Unknown));
    /// ```
    #[must_use]
    pub const fn unknown() -> Type {
        Type::Keyword(Self::Unknown)
    }

    /// Create a new `void` type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Keyword::void(), Type::Keyword(Keyword::Void));
    /// ```
    #[must_use]
    pub const fn void() -> Type {
        Type::Keyword(Self::Void)
    }
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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct FunctionArgument {
    /// The name of the argument.
    pub name: String,
    /// The type of the argument.
    pub r#type: Type,
}

impl FunctionArgument {
    /// Create a new array argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `inner` - The inner type of the array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::r#type::{
    ///     FunctionArgument,
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// let argument = FunctionArgument::array("foo", Keyword::number());
    ///
    /// assert_eq!(argument.name, "foo".to_owned());
    /// assert_eq!(argument.r#type, Type::array(Keyword::number()));
    /// ```
    #[must_use]
    pub fn array(
        name: &str,
        inner: Type,
    ) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::array(inner),
        }
    }
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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
        type_arguments: Vec<Type>,
    },
    /// A union of types.
    Union(Vec<Type>),
}

impl Type {
    /// Create an array type.
    ///
    /// # Arguments
    ///
    /// * `inner` - The inner type of the array.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::r#type::{
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// let array = Type::array(Keyword::number());
    ///
    /// assert_eq!(array, Type::Array(Box::new(Type::Keyword(Keyword::Number))));
    /// ```
    #[must_use]
    pub fn array(inner: Self) -> Self {
        Self::Array(Box::new(inner))
    }

    /// Create a function type.
    ///
    /// # Arguments
    ///
    /// * `arguments` - The function arguments.
    /// * `return_type` - The return type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::r#type::{
    ///     FunctionArgument,
    ///     Keyword,
    ///     Type,
    /// };
    ///
    /// let function = Type::function(
    ///     &[FunctionArgument::array("foo", Keyword::number())],
    ///     Keyword::string(),
    /// );
    ///
    /// assert_eq!(
    ///     function,
    ///     Type::Function {
    ///         arguments: vec![FunctionArgument {
    ///             name: "foo".to_owned(),
    ///             r#type: Type::array(Keyword::number()),
    ///         }],
    ///         return_type: Box::new(Keyword::string()),
    ///     },
    /// );
    /// ```
    #[must_use]
    pub fn function(
        arguments: &[FunctionArgument],
        return_type: Self,
    ) -> Self {
        Self::Function {
            arguments: arguments.to_owned(),
            return_type: Box::new(return_type),
        }
    }

    /// Create a type reference.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The name of the type.
    /// * `type_arguments` - The type arguments.
    #[must_use]
    pub fn type_reference(
        identifier: &str,
        type_arguments: &[Self],
    ) -> Self {
        Self::TypeReference {
            identifier: identifier.to_owned(),
            type_arguments: type_arguments.to_owned(),
        }
    }
}

impl From<ir::Type> for Type {
    fn from(r#type: ir::Type) -> Self {
        Self::TypeReference {
            identifier: match r#type {
                ir::Type::Boolean => "boolean".to_owned(),
                ir::Type::DateTime => "Date".to_owned(),
                ir::Type::Float | ir::Type::Int => "number".to_owned(),
                ir::Type::String => "string".to_owned(),
            },
            type_arguments: vec![],
        }
    }
}

impl From<ir::EnumRelation> for Type {
    fn from(ir::EnumRelation { name, cardinality }: ir::EnumRelation) -> Self {
        match cardinality {
            ir::Cardinality::One => Self::type_reference(&name, &[]),
            ir::Cardinality::Many => {
                Self::array(Self::type_reference(&name, &[]))
            }
        }
    }
}

impl From<ir::ModelRelation> for Type {
    fn from(
        ir::ModelRelation {
            name, cardinality, ..
        }: ir::ModelRelation
    ) -> Self {
        match cardinality {
            ir::Cardinality::One => Self::type_reference(&name, &[]),
            ir::Cardinality::Many => {
                Self::array(Self::type_reference(&name, &[]))
            }
        }
    }
}

impl From<ir::Field> for Type {
    fn from(
        ir::Field {
            r#type,
            cardinality,
            ..
        }: ir::Field
    ) -> Self {
        match cardinality {
            ir::Cardinality::One => r#type.into(),
            ir::Cardinality::Many => Self::array(r#type.into()),
        }
    }
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
                type_arguments: type_references,
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
                identifier: "Partial".to_owned(),
                type_arguments: vec![Type::TypeReference {
                    identifier: "Image".to_owned(),
                    type_arguments: vec![],
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
                        name: "name".to_owned(),
                        r#type: Type::TypeReference {
                            identifier: "Partial".to_owned(),
                            type_arguments: vec![Type::TypeReference {
                                identifier: "Image".to_owned(),
                                type_arguments: vec![],
                            }]
                        }
                    },
                    FunctionArgument {
                        name: "countryName".to_owned(),
                        r#type: Type::TypeReference {
                            identifier: "CountryName".to_owned(),
                            type_arguments: vec![],
                        }
                    }
                ]
                .into_iter()
                .collect(),
                return_type: Box::new(Type::TypeReference {
                    identifier: "String".to_owned(),
                    type_arguments: vec![]
                }),
            }
            .to_string(),
            "(name: Partial<Image>, countryName: CountryName) => String"
        );
    }

    #[test]
    fn test_display_keyword() {
        assert_eq!(Keyword::any().to_string(), "any");
        assert_eq!(Keyword::bigint().to_string(), "bigint");
        assert_eq!(Keyword::boolean().to_string(), "boolean");
        assert_eq!(Keyword::intrinsic().to_string(), "intrinsic");
        assert_eq!(Keyword::never().to_string(), "never");
        assert_eq!(Keyword::null().to_string(), "null");
        assert_eq!(Keyword::number().to_string(), "number");
        assert_eq!(Keyword::object().to_string(), "object");
        assert_eq!(Keyword::string().to_string(), "string");
        assert_eq!(Keyword::symbol().to_string(), "symbol");
        assert_eq!(Keyword::undefined().to_string(), "undefined");
        assert_eq!(Keyword::unknown().to_string(), "unknown");
        assert_eq!(Keyword::void().to_string(), "void");
    }

    #[test]
    fn test_display_literal() {
        assert_eq!(
            Type::Literal(Literal::BigInt("1".to_owned())).to_string(),
            "1n"
        );

        assert_eq!(Type::Literal(Literal::Boolean(true)).to_string(), "true");
        assert_eq!(Type::Literal(Literal::Boolean(false)).to_string(), "false");

        assert_eq!(
            Type::Literal(Literal::Number("1.0".to_owned())).to_string(),
            "1.0"
        );

        assert_eq!(
            Type::Literal(Literal::Number("1.5".to_owned())).to_string(),
            "1.5"
        );

        assert_eq!(
            Type::Literal(Literal::String("hello".to_owned())).to_string(),
            "\"hello\""
        );
    }

    #[test]
    fn test_display_intersection() {
        assert_eq!(
            Type::Intersection(vec![
                Type::TypeReference {
                    identifier: "Partial".to_owned(),
                    type_arguments: vec![Type::TypeReference {
                        identifier: "T".to_owned(),
                        type_arguments: vec![],
                    }]
                },
                Type::TypeReference {
                    identifier: "Partial".to_owned(),
                    type_arguments: vec![Type::TypeReference {
                        identifier: "U".to_owned(),
                        type_arguments: vec![],
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
                        name: "country".to_owned(),
                        r#type: Type::ObjectLiteral(
                            vec![
                                ObjectLiteralProperty {
                                    name: "name".to_owned(),
                                    r#type: Type::TypeReference {
                                        identifier: "CountryName".to_owned(),
                                        type_arguments: vec![],
                                    }
                                },
                                ObjectLiteralProperty {
                                    name: "languages".to_owned(),
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
                        name: "tags".to_owned(),
                        r#type: Type::Array(Box::new(Type::TypeReference {
                            identifier: "Tag".to_owned(),
                            type_arguments: vec![],
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
                    identifier: "CountryName".to_owned(),
                    type_arguments: vec![],
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
                identifier: "Partial".to_owned(),
                type_arguments: vec![Type::TypeReference {
                    identifier: "Image".to_owned(),
                    type_arguments: vec![],
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
                    identifier: "CountryName".to_owned(),
                    type_arguments: vec![],
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
            Type::Literal(Literal::BigInt("1".to_owned())).to_string(),
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
            Type::Literal(Literal::Number("1.0".to_owned())).to_string(),
            "1.0"
        );

        assert_eq!(
            Type::Literal(Literal::Number("1.1".to_owned())).to_string(),
            "1.1"
        );
    }

    #[test]
    fn test_display_literal_string() {
        assert_eq!(
            Type::Literal(Literal::String("foo".to_owned())).to_string(),
            "\"foo\""
        );
    }

    #[test]
    fn test_display_void() {
        assert_eq!(Keyword::Void.to_string(), "void");
    }
}
