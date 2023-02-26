use {
    crate::ast::Ast,
    std::fmt::Display,
};
pub use {
    argument::Argument,
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

impl From<Ast> for Schema {
    fn from(value: Ast) -> Self {
        Self {
            data_source: None,
            generators: Vec::new(),
            enums: value.enums.values().map(Enum::from).collect(),
            models: value.models.values().map(Model::from).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::ast::Ast,
    };

    #[test]
    fn test_display() {
        let source = "

model Foo {
    bar: String
    baz: Int
    qux: [Bar]
}

model Bar {
    quux: String
    quuz: Baz
}

enum Baz {
    Alpha
    Beta
    Gamma
}

"
        .trim();

        let (ast, _) = Ast::parse(source).unwrap();
        let mut schema = Schema::from(ast);

        schema.data_source = Some(DataSource {
            name: "db".to_owned(),
            provider: DataSourceProvider::PostgreSql {
                user: "admin".to_owned(),
                password: "admin".to_owned(),
                host: "localhost".to_owned(),
                port: 5432,
                database: "db".to_owned(),
                schema: "public".to_owned(),
                extensions: vec!["uuid-ossp".to_owned()],
            },
            shadow_database_url: None,
            direct_url: None,
            relation_mode: RelationMode::ForeignKeys,
        });

        schema.generators.push(Generator {
            name: "client".to_owned(),
            provider: GeneratorProvider::PrismaClientJs,
            output: None,
            binary_targets: vec![BinaryTarget::DarwinArm64],
            preview_features: vec![PreviewFeature::Deno],
            engine_type: None,
        });

        let expected = "

generator client {
  provider = \"prisma-client-js\"
  binaryTargets = [\"darwin-arm64\"]
  previewFeatures = [\"deno\"]
}

datasource db {
  provider = \"postgresql\"
  url = \"postgresql://admin:admin@localhost:5432/db?schema=public\"
  relationMode = \"foreignKeys\"
  extensions = [uuid-ossp]
}

enum Baz {
  Alpha
  Beta
  Gamma
}

model Bar {
  createdAt DateTime @default(now())
  id        Int      @id @default(autoincrement())
  quux      String
  quuz      Baz
}

model Foo {
  bar       String
  baz       Int
  createdAt DateTime @default(now())
  id        Int      @id @default(autoincrement())
  qux       Bar[]
}

"
        .trim();

        assert_eq!(schema.to_string(), expected);
    }
}
