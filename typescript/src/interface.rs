pub use property::Property;
use {
    super::{
        type_parameter::TypeParameter,
        ExpressionWithTypeArguments,
    },
    crate::Type,
    ir,
    print::{
        Print,
        PrintInline,
    },
    std::{
        borrow::Cow,
        io,
    },
};

/// Interface properties.
pub mod property;

/// An interface declaration.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Interface<'a> {
    /// The types that the interface extends.
    ///
    /// Note: An interface can only extend an object type or intersection of
    /// object types with statically known members.
    pub extends: Vec<ExpressionWithTypeArguments<'a>>,
    /// The name of the interface.
    pub identifier: Cow<'a, str>,
    /// The type parameters of the interface.
    pub type_parameters: Vec<TypeParameter<'a>>,
    /// The properties of the interface.
    pub properties: Vec<Property<'a>>,
}

impl Print for Interface<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            identifier: name,
            extends,
            type_parameters: parameters,
            properties,
        } = self;

        write!(f, "{}interface {name}", Self::indent(level))?;

        if !parameters.is_empty() {
            write!(f, "<")?;
            PrintInline::intercalate(parameters.clone(), f, ", ")?;
            write!(f, ">")?;
        };

        if !extends.is_empty() {
            write!(f, " extends ")?;
            PrintInline::intercalate(extends.clone(), f, ", ")?;
        };

        writeln!(f, " {{")?;

        for property in properties {
            property.print(level + 1, f)?;
        }

        writeln!(f, "}}\n")
    }
}

impl<'a> From<ir::Model<'a>> for Interface<'a> {
    fn from(ir_model: ir::Model<'a>) -> Self {
        let mut interface = Self {
            extends: vec![],
            identifier: ir_model.name,
            type_parameters: vec![],
            properties: vec![],
        };

        for (name, field) in ir_model.fields {
            interface.properties.push(Property {
                identifier: name.into(),
                r#type: field.into(),
                optional: false,
            });
        }

        for (relation_name, relation) in ir_model.enums {
            interface.properties.push(Property {
                identifier: relation_name.into(),
                r#type: relation.into(),
                optional: false,
            });
        }

        for (relation_name, relation) in ir_model.relations {
            interface.properties.push(match relation.r#type {
                ir::model::model_relation::Type::OneToOne => {
                    Property {
                        identifier: relation_name.into(),
                        r#type: Type::TypeReference {
                            identifier: relation.model_name.clone(),
                            type_arguments: vec![],
                        },
                        optional: true,
                    }
                }
                ir::model::model_relation::Type::ManyToOne => {
                    Property {
                        identifier: relation_name.into(),
                        r#type: Type::TypeReference {
                            identifier: relation.model_name.clone(),
                            type_arguments: vec![],
                        },
                        optional: false,
                    }
                }
                ir::model::model_relation::Type::OneToMany
                | ir::model::model_relation::Type::ManyToMany => {
                    Property {
                        identifier: relation_name.into(),
                        r#type: Type::Array(Box::new(Type::TypeReference {
                            identifier: relation.model_name.clone(),
                            type_arguments: vec![],
                        })),
                        optional: false,
                    }
                }
            });
        }

        interface
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::r#type::{
            Keyword,
            Type,
        },
        ir,
    };

    #[test]
    fn test_from_ir_model() -> Result<(), ir::TypeError<'static>> {
        let mut model = ir::Model::new("Image");

        model.insert_enum_relation("countryName", "CountryName")?;
        model.insert_enums_relation("tags", "Tag")?;

        model.insert_field(ir::model::Field {
            name: "isPublic".into(),
            r#type: ir::Type::Boolean,
            cardinality: ir::Cardinality::One,
        })?;

        model.insert_field(ir::model::Field {
            name: "createdAt".into(),
            r#type: ir::Type::DateTime,
            cardinality: ir::Cardinality::One,
        })?;

        model.insert_field(ir::model::Field {
            name: "latitude".into(),
            r#type: ir::Type::Float,
            cardinality: ir::Cardinality::One,
        })?;

        model.insert_field(ir::model::Field {
            name: "height".into(),
            r#type: ir::Type::Int,
            cardinality: ir::Cardinality::One,
        })?;

        model.insert_field(ir::model::Field {
            name: "title".into(),
            r#type: ir::Type::String,
            cardinality: ir::Cardinality::One,
        })?;

        model.insert_field(ir::model::Field {
            name: "anArrayOfBooleansIsDumb".into(),
            r#type: ir::Type::Boolean,
            cardinality: ir::Cardinality::Many,
        })?;

        model.insert_field(ir::model::Field {
            name: "events".into(),
            r#type: ir::Type::DateTime,
            cardinality: ir::Cardinality::Many,
        })?;

        model.insert_field(ir::model::Field {
            name: "latitudes".into(),
            r#type: ir::Type::Float,
            cardinality: ir::Cardinality::Many,
        })?;

        model.insert_field(ir::model::Field {
            name: "heights".into(),
            r#type: ir::Type::Int,
            cardinality: ir::Cardinality::Many,
        })?;

        model.insert_field(ir::model::Field {
            name: "names".into(),
            r#type: ir::Type::String,
            cardinality: ir::Cardinality::Many,
        })?;

        model.insert_many_to_one("owner", "User")?;
        model.insert_one_to_many("images", "Image")?;
        model.insert_one_to_one("resource", "Resource")?;
        model.insert_many_to_many("resources", "Resource")?;

        let interface = Interface::from(model);
        let mut f = Vec::new();

        interface.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "interface Image {
    isPublic: boolean;
    createdAt: Date;
    latitude: number;
    height: number;
    title: string;
    anArrayOfBooleansIsDumb: Array<boolean>;
    events: Array<Date>;
    latitudes: Array<number>;
    heights: Array<number>;
    names: Array<string>;
    countryName: CountryName;
    tags: Array<Tag>;
    owner: User;
    images: Array<Image>;
    resource?: Resource;
    resources: Array<Resource>;
}

"
        );

        Ok(())
    }

    #[test]
    fn test_print() {
        let interface = Interface {
            extends: vec![ExpressionWithTypeArguments {
                identifier: "Resource".into(),
                type_arguments: vec![Type::TypeReference {
                    identifier: "T".into(),
                    type_arguments: vec![],
                }],
            }],
            identifier: "Image".into(),
            type_parameters: vec![TypeParameter {
                identifier: "T".into(),
                type_references: Vec::new(),
            }],
            properties: vec![
                Property {
                    identifier: "title".into(),
                    r#type: Type::Keyword(Keyword::String),
                    optional: false,
                },
                Property {
                    identifier: "countryName".into(),
                    r#type: Type::TypeReference {
                        identifier: "CountryName".into(),
                        type_arguments: vec![],
                    },
                    optional: true,
                },
                Property {
                    identifier: "tags".into(),
                    r#type: Type::Array(Box::new(Type::TypeReference {
                        identifier: "Tag".into(),
                        type_arguments: vec![],
                    })),
                    optional: false,
                },
            ],
        };

        let mut f = Vec::new();

        interface.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "interface Image<T> extends Resource<T> {
    title: string;
    countryName?: CountryName;
    tags: Array<Tag>;
}

"
        );
    }
}
