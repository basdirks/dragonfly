use {
    crate::ir,
    std::fmt::Display,
};
pub use {
    argument::Argument,
    attribute::{
        Block as BlockAttribute,
        Field as FieldAttribute,
    },
    data_source::{
        DataSource,
        Provider as DataSourceProvider,
        RelationMode,
    },
    generator::{
        BinaryTarget,
        EngineType,
        Generator,
        PreviewFeature,
        Provider as GeneratorProvider,
    },
    model::{
        Field,
        FieldType,
        Model,
    },
    r#enum::Enum,
    value::{
        Function,
        Value,
    },
};

/// Arguments.
pub mod argument;
/// Attributes.
pub mod attribute;
/// Data source definitions.
pub mod data_source;
/// Enumerate types.
pub mod r#enum;
/// Generator definitions.
pub mod generator;
/// Model definitions.
pub mod model;
/// Values.
pub mod value;

/// A Prisma schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Schema {
    /// The data source.
    pub data_source: Option<DataSource>,
    /// The enums.
    pub enums: Vec<Enum>,
    /// The generators.
    pub generators: Vec<Generator>,
    /// The models.
    pub models: Vec<Model>,
}

impl Display for Schema {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            data_source,
            enums,
            generators,
            models,
        } = self;

        let mut output = Vec::new();

        for generator in generators {
            output.push(generator.to_string());
        }

        if let Some(data_source) = data_source {
            output.push(data_source.to_string());
        }

        for r#enum in enums {
            output.push(r#enum.to_string());
        }

        for model in models {
            output.push(model.to_string());
        }

        write!(f, "{}", output.join("\n\n"))
    }
}

impl From<&ir::Ir> for Schema {
    fn from(_value: &ir::Ir) -> Self {
        unimplemented!()
    }
}
