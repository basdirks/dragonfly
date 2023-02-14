use {
    super::r#type::Type,
    std::fmt::Display,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Property {
    name: String,
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
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
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
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
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
                name: "Image".to_string(),
                parameters: vec!["T".to_string(),],
                extends: vec![Extend {
                    name: "Resource".to_string(),
                    generics: vec![Extend {
                        name: "T".to_string(),
                        generics: vec![]
                    }]
                }],
                properties: vec![
                    Property {
                        name: "title".to_string(),
                        r#type: Type::String,
                        optional: false
                    },
                    Property {
                        name: "countryName".to_string(),
                        r#type: Type::Identifier {
                            name: "CountryName".to_string(),
                            generics: vec![],
                        },
                        optional: true
                    },
                    Property {
                        name: "tags".to_string(),
                        r#type: Type::Array {
                            r#type: Box::new(Type::Identifier {
                                name: "Tag".to_string(),
                                generics: vec![],
                            })
                        },
                        optional: false
                    }
                ]
            }
            .to_string(),
            "interface Image<T> extends Resource<T> { title: string; \
             countryName?: CountryName; tags: Array<Tag>; }"
        );
    }
}
