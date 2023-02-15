use {
    super::r#type::Type,
    std::fmt::Display,
};

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Interface {
    heritage_clause: Vec<ExpressionWithTypeArguments>,
    identifier: String,
    type_parameters: Vec<TypeParameter>,
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
            heritage_clause: extends,
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
            .join(" ");

        write!(
            f,
            "interface {name}{parameters}{extends} {{ {properties} }}"
        )
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::typescript::r#type::Type,
    };

    #[test]
    fn test_display_interface() {
        assert_eq!(
            Interface {
                heritage_clause: vec![ExpressionWithTypeArguments {
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
                        r#type: Type::String,
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
            "interface Image<T> extends Resource<T> { title: string; \
             countryName?: CountryName; tags: Array<Tag>; }"
        );
    }
}
