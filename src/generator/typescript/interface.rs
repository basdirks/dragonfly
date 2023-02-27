use {
    super::r#type::Type,
    crate::{
        ast::{
            Field,
            Model,
        },
        generator::printer::{
            common::comma_separated,
            indent,
            Print,
        },
    },
    std::fmt::Display,
};

/// An interface property.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Property {
    /// The name of the property. Usually camel case.
    pub identifier: String,
    /// The type of the property.
    pub r#type: Type,
    /// Whether the property is optional.
    pub optional: bool,
}

impl Print for Property {
    fn print(
        &self,
        level: usize,
    ) -> String {
        let Self {
            identifier,
            r#type: type_reference,
            optional,
        } = self;

        let optional = if *optional { "?" } else { "" };
        let indent = indent::typescript(level);

        format!("{indent}{identifier}{optional}: {type_reference};")
    }
}

/// An expression with type arguments.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExpressionWithTypeArguments {
    /// The name of the expression. Usually pascal case.
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
            write!(f, "{identifier}<{}>", comma_separated(type_arguments))
        }
    }
}

/// A type parameter.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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
                comma_separated(type_references)
            )
        }
    }
}

/// An interface declaration.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

impl Print for Interface {
    fn print(
        &self,
        level: usize,
    ) -> String {
        let Self {
            identifier: name,
            extends,
            type_parameters: parameters,
            properties,
        } = self;

        let indent = indent::typescript(level);

        let extends = if extends.is_empty() {
            String::new()
        } else {
            format!(" extends {}", comma_separated(extends))
        };

        let parameters = if parameters.is_empty() {
            String::new()
        } else {
            format!("<{}>", comma_separated(parameters))
        };

        let properties = properties
            .iter()
            .map(|property| property.print(level + 1))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "{indent}interface {name}{parameters}{extends} \
             {{\n{properties}\n{indent}}}",
        )
    }
}

impl From<Model> for Interface {
    fn from(model: Model) -> Self {
        let Model { name, fields, .. } = model;

        let fields = fields
            .into_iter()
            .map(|(identifier, Field { r#type, .. })| {
                Property {
                    identifier,
                    r#type: r#type.into(),
                    optional: false,
                }
            })
            .collect();

        Self {
            extends: vec![],
            identifier: name,
            type_parameters: vec![],
            properties: fields,
        }
    }
}

impl From<&Model> for Interface {
    fn from(model: &Model) -> Self {
        Self::from(model.clone())
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
    fn test_display_type_parameter() {
        assert_eq!(
            TypeParameter {
                identifier: "T".to_owned(),
                type_references: vec![],
            }
            .to_string(),
            "T"
        );

        assert_eq!(
            TypeParameter {
                identifier: "U".to_owned(),
                type_references: vec![Type::Keyword(Keyword::String)],
            }
            .to_string(),
            "U extends string"
        );

        assert_eq!(
            TypeParameter {
                identifier: "V".to_owned(),
                type_references: vec![
                    Type::Keyword(Keyword::String),
                    Type::Keyword(Keyword::Number),
                ],
            }
            .to_string(),
            "V extends string, number"
        );
    }

    #[test]
    fn test_display_expression_with_type_arguments() {
        assert_eq!(
            ExpressionWithTypeArguments {
                identifier: "Foo".to_owned(),
                type_arguments: vec![],
            }
            .to_string(),
            "Foo"
        );

        assert_eq!(
            ExpressionWithTypeArguments {
                identifier: "Foo".to_owned(),
                type_arguments: vec![Type::Keyword(Keyword::String)],
            }
            .to_string(),
            "Foo<string>"
        );

        assert_eq!(
            ExpressionWithTypeArguments {
                identifier: "Foo".to_owned(),
                type_arguments: vec![
                    Type::Keyword(Keyword::String),
                    Type::Keyword(Keyword::Number),
                ],
            }
            .to_string(),
            "Foo<string, number>"
        );
    }

    #[test]
    fn test_print_property() {
        assert_eq!(
            Property {
                identifier: "foo".to_owned(),
                r#type: Type::Keyword(Keyword::String),
                optional: false,
            }
            .print(0),
            "foo: string;"
        );

        assert_eq!(
            Property {
                identifier: "foo".to_owned(),
                r#type: Type::Keyword(Keyword::String),
                optional: true,
            }
            .print(0),
            "foo?: string;"
        );
    }

    #[test]
    fn test_print_interface() {
        let expected = "

interface Image<T> extends Resource<T> {
    title: string;
    countryName?: CountryName;
    tags: Array<Tag>;
}

"
        .trim();

        assert_eq!(
            Interface {
                extends: vec![ExpressionWithTypeArguments {
                    identifier: "Resource".to_owned(),
                    type_arguments: vec![Type::TypeReference {
                        identifier: "T".to_owned(),
                        type_references: vec![],
                    }],
                }],
                identifier: "Image".to_owned(),
                type_parameters: vec![TypeParameter {
                    identifier: "T".to_owned(),
                    type_references: vec![],
                }],
                properties: vec![
                    Property {
                        identifier: "title".to_owned(),
                        r#type: Type::Keyword(Keyword::String),
                        optional: false,
                    },
                    Property {
                        identifier: "countryName".to_owned(),
                        r#type: Type::TypeReference {
                            identifier: "CountryName".to_owned(),
                            type_references: vec![],
                        },
                        optional: true,
                    },
                    Property {
                        identifier: "tags".to_owned(),
                        r#type: Type::Array(Box::new(Type::TypeReference {
                            identifier: "Tag".to_owned(),
                            type_references: vec![],
                        })),
                        optional: false,
                    },
                ],
            }
            .print(0),
            expected
        );
    }

    #[test]
    fn test_from_model() {
        let input = "

model Image {
  tags: [Tag]
  title: String    
  country: Country
}

"
        .trim();

        let expected = "

interface Image {
    country: Country;
    tags: Array<Tag>;
    title: string;
}

"
        .trim();

        let (model, _) = Model::parse(input).unwrap();
        let interface = Interface::from(model);
        let code = interface.print(0);

        assert_eq!(code, expected);
    }

    #[test]
    fn test_interface_from_model() {
        let input = "

model Image {
  tags: [Tag]
  title: String
  country: Country
}

"
        .trim();

        let (model, _) = Model::parse(input).unwrap();
        let interface = Interface::from(model);
        let code = interface.print(0);

        let expected = "

interface Image {
    country: Country;
    tags: Array<Tag>;
    title: string;
}

"
        .trim();

        assert_eq!(code, expected);
    }
}
