use {
    super::r#type::Type,
    std::fmt::Display,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Property {
    identifier: String,
    r#type: Type,
    optional: bool,
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

#[derive(Clone, Debug, PartialEq)]
pub struct ExpressionWithTypeArguments {
    identifier: String,
    type_arguments: Vec<Type>,
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

#[derive(Clone, Debug, PartialEq)]
pub struct TypeParameter {
    identifier: String,
    type_reference: Option<Type>,
}

impl Display for TypeParameter {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            identifier,
            type_reference,
        } = self;

        if let Some(type_reference) = type_reference {
            write!(f, "{identifier} extends {type_reference}")
        } else {
            write!(f, "{identifier}")
        }
    }
}

/// An interface declaration.
///
/// For now, we do not support methods, getters, setters, call signatures,
/// construct signatures, or index signatures.
#[derive(Clone, Debug, PartialEq)]
pub struct Interface {
    /// The types that the interface extends.
    ///
    /// # Examples
    ///
    /// `Bar` and `Baz` are types that the interface extends:
    ///
    /// ```typescript
    /// interface Foo extends Bar, Baz {}
    /// ```
    extends: Vec<ExpressionWithTypeArguments>,
    /// The name of the interface.
    ///
    /// # Examples
    ///
    /// `Foo` is the name:
    ///
    /// ```typescript
    /// interface Foo {}
    /// ```
    identifier: String,
    /// The type parameters of the interface.
    ///
    /// # Examples
    ///
    /// `T` and `U` are type parameters:
    ///
    /// ```typescript
    /// interface Foo<T, U> {}
    /// ```
    type_parameters: Vec<TypeParameter>,
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
    properties: Vec<Property>,
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
                    type_reference: None,
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
