use {
    super::r#type::Type,
    std::fmt::Display,
};

/// An interface property.
///
/// # Examples
///
/// ```rust
/// use dragonfly::{
///     ast::r#type::Basic,
///     generator::typescript::{
///         interface::Property,
///         r#type::{
///             Keyword,
///             Type,
///         },
///     },
/// };
///
/// let code = Property {
///     identifier: "foo".to_string(),
///     r#type: Type::Keyword(Keyword::String),
///     optional: false,
/// }
/// .to_string();
///
/// assert_eq!(code, "foo: string;");
///
/// let code = Property {
///     identifier: "bar".to_string(),
///     r#type: Type::Array(Box::new(Type::Keyword(Keyword::String))),
///     optional: true,
/// }
/// .to_string();
///
/// assert_eq!(code, "bar?: Array<string>;");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    /// The name of the property. Usually camelCase.
    pub identifier: String,
    /// The type of the property.
    pub r#type: Type,
    /// Whether the property is optional.
    pub optional: bool,
}

// TODO: Replace with pretty printer.
impl Display for Property {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            identifier,
            r#type: type_reference,
            optional,
        } = self;

        if *optional {
            write!(f, "{identifier}?: {type_reference};")
        } else {
            write!(f, "{identifier}: {type_reference};")
        }
    }
}

/// An expression with type arguments.
///
/// # Examples
///
/// ```rust
/// use dragonfly::{
///     ast::r#type::Basic,
///     generator::typescript::{
///         interface::ExpressionWithTypeArguments,
///         r#type::{
///             Keyword,
///             Type,
///         },
///     },
/// };
///
/// let code = ExpressionWithTypeArguments {
///     identifier: "Foo".to_string(),
///     type_arguments: vec![],
/// }
/// .to_string();
///
/// assert_eq!(code, "Foo");
///
/// let code = ExpressionWithTypeArguments {
///     identifier: "Bar".to_string(),
///     type_arguments: vec![
///         Type::Keyword(Keyword::String),
///         Type::Keyword(Keyword::Number),
///     ],
/// }
/// .to_string();
///
/// assert_eq!(code, "Bar<string, number>");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionWithTypeArguments {
    /// The name of the expression. Usually PascalCase.
    pub identifier: String,
    /// The type arguments of the expression.
    pub type_arguments: Vec<Type>,
}

impl Display for ExpressionWithTypeArguments {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            identifier,
            type_arguments,
        } = self;

        if type_arguments.is_empty() {
            write!(f, "{identifier}")
        } else {
            let type_arguments = type_arguments
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(", ");

            write!(f, "{identifier}<{type_arguments}>")
        }
    }
}

/// A type parameter.
///
/// # Examples
///
/// ```rust
/// use dragonfly::{
///     ast::r#type::Basic,
///     generator::typescript::{
///         interface::TypeParameter,
///         r#type::{
///             Keyword,
///             Type,
///         },
///     },
/// };
///
/// let code = TypeParameter {
///     identifier: "T".to_string(),
///     type_references: vec![],
/// }
/// .to_string();
///
/// assert_eq!(code, "T");
///
/// let code = TypeParameter {
///     identifier: "U".to_string(),
///     type_references: vec![
///         Type::Keyword(Keyword::String),
///         Type::Keyword(Keyword::Number),
///     ],
/// }
/// .to_string();
///
/// assert_eq!(code, "U extends string, number");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct TypeParameter {
    /// The name of the type parameter.
    pub identifier: String,
    /// The types that the type parameter extends.
    pub type_references: Vec<Type>,
}

impl Display for TypeParameter {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            identifier,
            type_references,
        } = self;

        if type_references.is_empty() {
            write!(f, "{identifier}")
        } else {
            write!(
                f,
                "{identifier} extends {}",
                type_references
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

/// An interface declaration.
///
/// Only supports properties, so does *not* support methods, getters, setters,
/// call signatures, construct signatures, or index signatures.
///
/// # Examples
///
/// ```rust
/// use dragonfly::{
///     ast::r#type::Basic,
///     generator::typescript::{
///         interface::{
///             ExpressionWithTypeArguments,
///             Interface,
///             Property,
///             TypeParameter,
///         },
///         r#type::{
///             Keyword,
///             Type,
///         },
///     },
/// };
///
/// let code = Interface {
///     extends: vec![ExpressionWithTypeArguments {
///         identifier: "Bar".to_string(),
///         type_arguments: vec![
///             Type::Keyword(Keyword::String),
///             Type::Keyword(Keyword::Number),
///         ],
///     }],
///     identifier: "Foo".to_string(),
///     type_parameters: vec![
///         TypeParameter {
///             identifier: "T".to_string(),
///             type_references: vec![],
///         },
///         TypeParameter {
///             identifier: "U".to_string(),
///             type_references: vec![Type::Keyword(Keyword::String)],
///         },
///     ],
///     properties: vec![
///         Property {
///             identifier: "bar".to_string(),
///             r#type: Type::Array(Box::new(Type::TypeReference {
///                 identifier: "Bar".to_string(),
///                 type_references: vec![],
///             })),
///             optional: true,
///         },
///         Property {
///             identifier: "baz".to_string(),
///             r#type: Type::Keyword(Keyword::Number),
///             optional: false,
///         },
///     ],
/// }
/// .to_string();
///
/// assert_eq!(
///     code,
///     "\
/// interface Foo<T, U extends string> extends Bar<string, number> {
///   bar?: Array<Bar>;
///   baz: number;
/// }"
/// );
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Interface {
    /// The types that the interface extends.
    ///
    /// Note: An interface can only extend an object type or intersection of
    /// object types with statically known members.
    ///
    /// # Examples
    ///
    /// `Bar` and `Baz` are types that the interface extends:
    ///
    /// ```typescript
    /// interface Foo extends Bar, Baz {}
    /// ```
    pub extends: Vec<ExpressionWithTypeArguments>,
    /// The name of the interface.
    ///
    /// # Examples
    ///
    /// `Foo` is the identifier:
    ///
    /// ```typescript
    /// interface Foo {}
    /// ```
    pub identifier: String,
    /// The type parameters of the interface.
    ///
    /// # Examples
    ///
    /// `T` and `U` are type parameters:
    ///
    /// ```typescript
    /// interface Foo<T, U> {}
    /// ```
    pub type_parameters: Vec<TypeParameter>,
    /// The properties of the interface.
    ///
    /// # Examples
    ///
    /// `bar` and `baz` are properties:
    ///
    /// ```typescript
    /// interface Foo {
    ///     bar: String;
    ///     baz: Int;
    /// }
    /// ```
    pub properties: Vec<Property>,
}

// TODO: Replace with pretty printer.
impl Display for Interface {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            identifier: name,
            extends,
            type_parameters: parameters,
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
            .join("\n  ");

        write!(
            f,
            "interface {name}{parameters}{extends} {{\n  {properties}\n}}"
        )
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::typescript::r#type::{
            Keyword,
            Type,
        },
    };

    #[test]
    fn test_display_interface() {
        assert_eq!(
            Interface {
                extends: vec![ExpressionWithTypeArguments {
                    identifier: "Resource".to_string(),
                    type_arguments: vec![Type::TypeReference {
                        identifier: "T".to_string(),
                        type_references: vec![],
                    }],
                }],
                identifier: "Image".to_string(),
                type_parameters: vec![TypeParameter {
                    identifier: "T".to_string(),
                    type_references: vec![],
                }],
                properties: vec![
                    Property {
                        identifier: "title".to_string(),
                        r#type: Type::Keyword(Keyword::String),
                        optional: false,
                    },
                    Property {
                        identifier: "countryName".to_string(),
                        r#type: Type::TypeReference {
                            identifier: "CountryName".to_string(),
                            type_references: vec![],
                        },
                        optional: true,
                    },
                    Property {
                        identifier: "tags".to_string(),
                        r#type: Type::Array(Box::new(Type::TypeReference {
                            identifier: "Tag".to_string(),
                            type_references: vec![],
                        })),
                        optional: false,
                    },
                ],
            }
            .to_string(),
            "\
interface Image<T> extends Resource<T> {
  title: string;
  countryName?: CountryName;
  tags: Array<Tag>;
}"
        );
    }
}
