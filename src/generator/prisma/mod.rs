use {
    crate::ast::{
        Ast,
        Field as AstField,
        Model as AstModel,
        Scalar as AstScalar,
        Type as AstType,
    },
    std::{
        collections::BTreeMap,
        fmt::Display,
    },
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

/// A model augmentation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Augmentation {
    /// The type of a model field implies a relation.
    BelongsTo {
        /// The name of the model that the field belongs to.
        belongs_to: String,
        /// The name of the model that the field is a relation to.
        model_name: String,
        /// The name of the relation.
        relation_name: String,
    },
}

impl Augmentation {
    /// Get augmentations from an AST model.
    ///
    /// Augmentations are used to add fields to models that are not explicitly
    /// defined in the schema.
    ///
    /// # Arguments
    ///
    /// * `ast_model` - The AST model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Ast,
    ///     generator::prisma::Augmentation,
    /// };
    ///
    /// let source = "
    ///
    /// model Foo {
    ///   bar: String
    ///   baz: Int
    ///   qux: Bar
    /// }
    ///
    /// model Bar {
    ///   quux: String
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let (ast, _) = Ast::parse(source).unwrap();
    /// let augmentations = Augmentation::from_model(&ast.models["Foo"].clone());
    ///
    /// assert_eq!(augmentations.len(), 1);
    /// assert_eq!(
    ///     augmentations[0],
    ///     Augmentation::BelongsTo {
    ///         belongs_to: "Foo".to_string(),
    ///         model_name: "Bar".to_string(),
    ///         relation_name: "qux".to_string(),
    ///     }
    /// );
    /// ```
    #[must_use]
    pub fn from_model(ast_model: &AstModel) -> Vec<Self> {
        let mut augmentations = Vec::new();

        for field in ast_model.fields.values() {
            if let AstField {
                name: relation_name,
                r#type: AstType::Scalar(AstScalar::Reference(model_name)),
            } = field
            {
                augmentations.push(Self::BelongsTo {
                    belongs_to: ast_model.name.clone(),
                    model_name: model_name.clone(),
                    relation_name: relation_name.clone(),
                });
            }
        }

        augmentations
    }

    /// Apply an augmentation to a model.
    ///
    /// # Arguments
    ///
    /// * `models` - The models.
    ///
    /// # Errors
    ///
    /// Returns an error if the augmentation cannot be applied.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Ast,
    ///     generator::prisma::{
    ///         Argument,
    ///         Augmentation,
    ///         Field,
    ///         FieldAttribute,
    ///         FieldType,
    ///         Function,
    ///         Model,
    ///         Value,
    ///     },
    /// };
    ///
    /// let source = "
    ///
    /// model Profile {
    ///   bar: String
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let mut models = Ast::parse(source)
    ///     .unwrap()
    ///     .0
    ///     .models
    ///     .into_iter()
    ///     .map(|(name, model)| (name, Model::from(&model)))
    ///     .collect();
    ///
    /// assert_eq!(
    ///     Augmentation::BelongsTo {
    ///         belongs_to: "User".to_string(),
    ///         model_name: "Profile".to_string(),
    ///         relation_name: "profile".to_string(),
    ///     }
    ///     .apply(&mut models),
    ///     Ok(())
    /// );
    ///
    /// let actual = models["Profile"].to_string();
    ///
    /// let expected = "
    ///
    /// model Profile {
    ///   bar       String
    ///   createdAt DateTime @default(now())
    ///   id        Int      @id @default(autoincrement())
    ///   user      User     @relation(\"profileOnUser\", fields: [userId], \
    ///                 references: [id])
    ///   userId    Int      @unique
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(actual, expected);
    /// ```
    pub fn apply(
        self,
        models: &mut BTreeMap<String, Model>,
    ) -> Result<(), String> {
        return match self {
            Self::BelongsTo {
                belongs_to,
                model_name,
                relation_name,
            } => {
                if let Some(model) = models.get_mut(&model_name) {
                    let id_field_name =
                        format!("{}Id", belongs_to.to_ascii_lowercase());

                    let relation_name =
                        format!("{relation_name}On{belongs_to}");

                    model.fields.push(Field {
                        name: belongs_to.to_ascii_lowercase(),
                        r#type: FieldType::Name(belongs_to),
                        required: true,
                        array: false,
                        attributes: vec![FieldAttribute {
                            group: None,
                            name: "relation".to_owned(),
                            arguments: vec![
                                Argument {
                                    name: None,
                                    value: Value::String(relation_name),
                                },
                                Argument {
                                    name: Some("fields".to_owned()),
                                    value: Value::Array(vec![Value::Keyword(
                                        id_field_name.clone(),
                                    )]),
                                },
                                Argument {
                                    name: Some("references".to_owned()),
                                    value: Value::Array(vec![Value::Keyword(
                                        "id".to_owned(),
                                    )]),
                                },
                            ],
                        }],
                    });

                    model.fields.push(Field {
                        name: id_field_name,
                        r#type: FieldType::Name("Int".to_owned()),
                        required: true,
                        array: false,
                        attributes: vec![FieldAttribute::unique()],
                    });

                    Ok(())
                } else {
                    Err(format!("Model {model_name} not found"))
                }
            }
        };
    }
}

impl From<Ast> for Schema {
    fn from(ast: Ast) -> Self {
        let mut models = BTreeMap::new();
        let mut augmentations = Vec::new();

        for (name, ast_model) in ast.models {
            let _ = models.insert(name, Model::from(&ast_model));
            augmentations.extend(Augmentation::from_model(&ast_model));
        }

        for augmentation in augmentations {
            let _ = augmentation.apply(&mut models);
        }

        Self {
            data_source: None,
            generators: Vec::new(),
            enums: ast.enums.values().map(Enum::from).collect(),
            models: models.values().cloned().collect(),
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
  provider        = \"prisma-client-js\"
  binaryTargets   = [\"darwin-arm64\"]
  previewFeatures = [\"deno\"]
}

datasource db {
  provider     = \"postgresql\"
  url          = \"postgresql://admin:admin@localhost:5432/db?schema=public\"
  relationMode = \"foreignKeys\"
  extensions   = [uuid-ossp]
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
