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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(
            Schema {
                data_source: Some(DataSource {
                    name: "db".to_owned(),
                    provider: DataSourceProvider::PostgreSql {
                        user: "user".to_owned(),
                        password: "password".to_owned(),
                        host: "localhost".to_owned(),
                        port: 5432,
                        database: "database".to_owned(),
                        schema: "public".to_owned(),
                        extensions: vec![],
                    },
                    shadow_database_url: Some(
                        "postgresql://user:password@localhost:5432/database"
                            .to_owned()
                    ),
                    direct_url: Some(
                        "postgresql://user:password@localhost:5432/database"
                            .to_owned()
                    ),
                    relation_mode: RelationMode::ForeignKeys,
                }),
                enums: vec![
                    Enum::new("Role", &["USER", "ADMIN"], &[]),
                    Enum::new("Status", &["ACTIVE", "INACTIVE"], &[])
                ],
                generators: vec![Generator {
                    name: "client".to_owned(),
                    provider: GeneratorProvider::PrismaClientJs,
                    output: Some("path/to/client".to_owned()),
                    binary_targets: vec![BinaryTarget::AlpineOpenSsl3_0],
                    preview_features: vec![
                        PreviewFeature::ExtendedWhereUnique,
                        PreviewFeature::FullTextIndex,
                        PreviewFeature::FullTextSearch,
                    ],
                    engine_type: Some(EngineType::Binary),
                }],
                models: vec![Model::new(
                    "User",
                    &[Field::id(), Field::created_at()],
                    &[]
                )]
            }
            .to_string(),
            "

generator client {
  provider        = \"prisma-client-js\"
  output          = \"path/to/client\"
  binaryTargets   = [\"linux-musl-openssl-3.0.x\"]
  previewFeatures = [\"extendedWhereUnique\", \"fullTextIndex\", \
             \"fullTextSearch\"]
  engineType      = \"binary\"
}

datasource db {
  provider          = \"postgresql\"
  url               = \
             \"postgresql://user:password@localhost:5432/database?\
             schema=public\"
  shadowDatabaseUrl = \"postgresql://user:password@localhost:5432/database\"
  directUrl         = \"postgresql://user:password@localhost:5432/database\"
  relationMode      = \"foreignKeys\"
  extensions        = []
}

enum Role {
  USER
  ADMIN
}

enum Status {
  ACTIVE
  INACTIVE
}

model User {
  createdAt DateTime @default(now())
  id        Int      @id @default(autoincrement())
}

"
            .trim()
        );
    }
}
