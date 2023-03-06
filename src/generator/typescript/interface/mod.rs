pub use property::Property;
use {
    super::{
        r#type::Type,
        type_parameter::TypeParameter,
        ExpressionWithTypeArguments,
    },
    crate::{
        generator::printer::{
            common::comma_separated,
            indent,
            Print,
        },
        ir,
    },
};

/// Interface properties.
pub mod property;

/// An interface declaration.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Interface {
    /// The types that the interface extends.
    ///
    /// Note: An interface can only extend an object type or intersection of
    /// object types with statically known members.
    pub extends: Vec<ExpressionWithTypeArguments>,
    /// The name of the interface.
    pub identifier: String,
    /// The type parameters of the interface.
    pub type_parameters: Vec<TypeParameter>,
    /// The properties of the interface.
    pub properties: Vec<Property>,
}

impl Interface {
    /// Create an empty interface.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The name of the interface.
    #[must_use]
    pub fn new(identifier: &str) -> Self {
        Self {
            extends: vec![],
            identifier: identifier.to_owned(),
            type_parameters: vec![],
            properties: vec![],
        }
    }
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

impl From<&ir::Model> for Interface {
    fn from(model: &ir::Model) -> Self {
        let mut interface = Self::new(&model.name());

        for (name, field) in model.fields() {
            interface
                .properties
                .push(Property::required(&name, Type::from(field)));
        }

        for (name, r#enum) in model.enum_relations() {
            interface
                .properties
                .push(Property::required(&name, Type::from(r#enum)));
        }

        for (name, model) in model.model_relations() {
            interface
                .properties
                .push(Property::required(&name, Type::from(model)));
        }

        for (name, model) in model.owned_model_relations() {
            let r#type = Type::from(model);

            interface.properties.push(Property::new(
                &name,
                r#type.clone(),
                !matches!(r#type, Type::Array { .. }),
            ));
        }

        interface
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
    fn test_from_ir_mode() {
        let mut model = ir::Model::new("Image");

        let _ = model.insert_enum_relation("countryName", "CountryName");
        let _ = model.insert_enums_relation("tags", "Tag");
        let _ = model.insert_field(ir::Field::boolean("isPublic"));
        let _ = model.insert_field(ir::Field::date_time("createdAt"));
        let _ = model.insert_field(ir::Field::float("latitude"));
        let _ = model.insert_field(ir::Field::int("height"));
        let _ = model.insert_field(ir::Field::string("title"));
        let _ =
            model.insert_field(ir::Field::booleans("anArrayOfBooleansIsDumb"));
        let _ = model.insert_field(ir::Field::date_times("events"));
        let _ = model.insert_field(ir::Field::floats("latitudes"));
        let _ = model.insert_field(ir::Field::ints("heights"));
        let _ = model.insert_field(ir::Field::strings("names"));
        let _ = model.insert_model_relation("owner", "User");
        let _ = model.insert_models_relation("images", "Image");
        let _ = model.insert_owned_model_relation("resource", "Resource");
        let _ = model.insert_owned_models_relation("resources", "Resource");

        let interface = Interface::from(&model);

        assert_eq!(
            interface.print(0),
            "

interface Image {
    anArrayOfBooleansIsDumb: Array<boolean>;
    createdAt: Date;
    events: Array<Date>;
    height: number;
    heights: Array<number>;
    isPublic: boolean;
    latitude: number;
    latitudes: Array<number>;
    names: Array<string>;
    title: string;
    countryName: CountryName;
    tags: Array<Tag>;
    images: Array<Image>;
    owner: User;
    resource?: Resource;
    resources: Array<Resource>;
}

"
            .trim()
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
                        type_arguments: vec![],
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
                            type_arguments: vec![],
                        },
                        optional: true,
                    },
                    Property {
                        identifier: "tags".to_owned(),
                        r#type: Type::Array(Box::new(Type::TypeReference {
                            identifier: "Tag".to_owned(),
                            type_arguments: vec![],
                        })),
                        optional: false,
                    },
                ],
            }
            .print(0),
            expected
        );
    }
}
