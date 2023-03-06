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
    std::io,
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
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            identifier: name,
            extends,
            type_parameters: parameters,
            properties,
        } = self;

        let indent = indent::typescript(level);

        write!(f, "{indent}interface {name}")?;

        if !parameters.is_empty() {
            write!(f, "<{}>", comma_separated(parameters))?;
        };

        if !extends.is_empty() {
            write!(f, " extends {}", comma_separated(extends))?;
        };

        writeln!(f, " {{")?;

        for property in properties {
            property.print(level + 1, f)?;
        }

        writeln!(f, "}}")
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
        crate::{
            ast::TypeError,
            generator::typescript::r#type::{
                Keyword,
                Type,
            },
        },
    };

    #[test]
    fn test_from_ir_mode() -> Result<(), TypeError> {
        let mut model = ir::Model::new("Image");

        model.insert_enum_relation("countryName", "CountryName")?;
        model.insert_enums_relation("tags", "Tag")?;
        model.insert_field(ir::Field::boolean("isPublic"))?;
        model.insert_field(ir::Field::date_time("createdAt"))?;
        model.insert_field(ir::Field::float("latitude"))?;
        model.insert_field(ir::Field::int("height"))?;
        model.insert_field(ir::Field::string("title"))?;
        model.insert_field(ir::Field::booleans("anArrayOfBooleansIsDumb"))?;
        model.insert_field(ir::Field::date_times("events"))?;
        model.insert_field(ir::Field::floats("latitudes"))?;
        model.insert_field(ir::Field::ints("heights"))?;
        model.insert_field(ir::Field::strings("names"))?;
        model.insert_model_relation("owner", "User")?;
        model.insert_models_relation("images", "Image")?;
        model.insert_owned_model_relation("resource", "Resource")?;
        model.insert_owned_models_relation("resources", "Resource")?;

        let interface = Interface::from(&model);
        let mut f = Vec::new();

        interface.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "interface Image {
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
        );

        Ok(())
    }

    #[test]
    fn test_print_interface() {
        let interface = Interface {
            extends: vec![ExpressionWithTypeArguments::new(
                "Resource",
                &[Type::type_reference("T", &[])],
            )],
            identifier: "Image".to_owned(),
            type_parameters: vec![TypeParameter::new("T", &[])],
            properties: vec![
                Property::required("title", Type::Keyword(Keyword::String)),
                Property::optional(
                    "countryName",
                    Type::type_reference("CountryName", &[]),
                ),
                Property::required(
                    "tags",
                    Type::array(Type::type_reference("Tag", &[])),
                ),
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
