//! Prisma schema generation.
//!
//! A schema can be created from an intermediate representation (`ir::Ir`) of a
//! Dragonfly program.
#![feature(rustdoc_missing_doc_code_examples)]
#![deny(
    clippy::all,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::missing_docs_in_private_items,
    clippy::mixed_read_write_in_expression,
    clippy::nursery,
    clippy::pedantic,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rustdoc::missing_doc_code_examples,
    rustdoc::missing_crate_level_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

use {
    self::schema_error::SchemaError,
    model::field::Modifier,
    ord_str_map::OrdStrMap,
    print::Print,
    std::io,
};
pub use {
    argument::Argument,
    data_source::DataSource,
    generator::Generator,
    model::Model,
    r#enum::Enum,
    value::{
        Function,
        Value,
    },
};

/// Tab size.
pub const TAB_SIZE: usize = 2;

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
/// Errors that can occur when working with Prisma schemas.
pub mod schema_error;
/// Values.
pub mod value;

/// A Prisma schema.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Schema<'a> {
    /// The data source.
    pub data_source: Option<DataSource<'a>>,
    /// The enums.
    pub enums: OrdStrMap<Enum<'a>>,
    /// The generators.
    pub generators: OrdStrMap<Generator<'a>>,
    /// The models.
    pub models: OrdStrMap<Model<'a>>,
}

impl<'a> Schema<'a> {
    /// Create an empty schema.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            data_source: None,
            enums: OrdStrMap::new(),
            generators: OrdStrMap::new(),
            models: OrdStrMap::new(),
        }
    }

    /// Add foreign keys to the schema based on the given model.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to add foreign keys for.
    ///
    /// # Errors
    ///
    /// Returns an error if a related model is not found.
    // Laughs in LoC.
    #[allow(clippy::too_many_lines)]
    fn add_foreign_keys(
        &mut self,
        source: &ir::Model<'a>,
    ) -> Result<(), SchemaError> {
        for (relation_name, relation) in source.relations.clone() {
            if let Some(target) = self.models.get_mut(&relation.model_name) {
                let reverse_relation_name = source.name().to_ascii_lowercase();

                match relation.r#type {
                    ir::model::model_relation::Type::OneToMany => {
                        let field = model::Field {
                            name: reverse_relation_name.clone().into(),
                            r#type: model::field::Type::Name(source.name()),
                            modifier: Modifier::Optional,
                            attributes: vec![attribute::Field {
                                name: "relation".into(),
                                arguments: vec![
                                    Argument {
                                        name: Some("name".into()),
                                        value: Value::String(
                                            format!(
                                                "{}On{}",
                                                relation_name, source.name
                                            )
                                            .into(),
                                        ),
                                    },
                                    Argument {
                                        name: Some("fields".into()),
                                        value: Value::Array(vec![
                                            Value::Keyword(
                                                format!(
                                                    "{}Id",
                                                    source
                                                        .name
                                                        .to_ascii_lowercase()
                                                )
                                                .into(),
                                            ),
                                        ]),
                                    },
                                    Argument {
                                        name: Some("references".into()),
                                        value: Value::Array(vec![
                                            Value::Keyword("id".into()),
                                        ]),
                                    },
                                ],
                                group: None,
                            }],
                        };

                        if target
                            .fields
                            .insert(reverse_relation_name.clone(), field)
                            .is_some()
                        {
                            return Err(SchemaError::duplicate_model_field(
                                target.name.clone(),
                                reverse_relation_name,
                            ));
                        }

                        let field = model::Field {
                            name: format!("{}Id", source.name).into(),
                            r#type: model::field::Type::Name("Int".into()),
                            modifier: Modifier::Optional,
                            attributes: vec![attribute::Field::unique()],
                        };

                        if target
                            .fields
                            .insert(format!("{reverse_relation_name}Id"), field)
                            .is_some()
                        {
                            return Err(SchemaError::duplicate_model_field(
                                target.name.clone(),
                                reverse_relation_name,
                            ));
                        }
                    }
                    ir::model::model_relation::Type::ManyToMany => {
                        let field = model::Field {
                            name: reverse_relation_name.clone().into(),
                            r#type: model::field::Type::Name(source.name()),
                            modifier: Modifier::List,
                            attributes: vec![attribute::Field {
                                name: "relation".into(),
                                arguments: vec![Argument {
                                    name: Some("name".into()),
                                    value: Value::String(
                                        format!(
                                            "{}On{}",
                                            relation_name, source.name
                                        )
                                        .into(),
                                    ),
                                }],
                                group: None,
                            }],
                        };

                        if target
                            .fields
                            .insert(reverse_relation_name.clone(), field)
                            .is_some()
                        {
                            return Err(SchemaError::duplicate_model(
                                relation_name,
                            ));
                        }
                    }
                    ir::model::model_relation::Type::OneToOne => {
                        let field = model::Field {
                            name: reverse_relation_name.clone().into(),
                            r#type: model::field::Type::Name(source.name()),
                            modifier: Modifier::None,
                            attributes: vec![attribute::Field {
                                name: "relation".into(),
                                arguments: vec![
                                    Argument {
                                        name: Some("name".into()),
                                        value: Value::String(
                                            format!(
                                                "{}On{}",
                                                relation_name, source.name
                                            )
                                            .into(),
                                        ),
                                    },
                                    Argument {
                                        name: Some("fields".into()),
                                        value: Value::Array(vec![
                                            Value::Keyword(
                                                format!(
                                                    "{}Id",
                                                    source
                                                        .name
                                                        .to_ascii_lowercase()
                                                )
                                                .into(),
                                            ),
                                        ]),
                                    },
                                    Argument {
                                        name: Some("references".into()),
                                        value: Value::Array(vec![
                                            Value::Keyword("id".into()),
                                        ]),
                                    },
                                ],
                                group: None,
                            }],
                        };

                        if target
                            .fields
                            .insert(reverse_relation_name.clone(), field)
                            .is_some()
                        {
                            return Err(SchemaError::duplicate_model_field(
                                target.name.clone(),
                                reverse_relation_name,
                            ));
                        }

                        let field = model::Field {
                            name: format!("{}Id", source.name).into(),
                            r#type: model::field::Type::Name("Int".into()),
                            modifier: Modifier::None,
                            attributes: vec![attribute::Field::unique()],
                        };

                        if target
                            .fields
                            .insert(format!("{reverse_relation_name}Id"), field)
                            .is_some()
                        {
                            return Err(SchemaError::duplicate_model_field(
                                target.name.clone(),
                                reverse_relation_name,
                            ));
                        }
                    }
                    ir::model::model_relation::Type::ManyToOne => {}
                }
            } else {
                return Err(SchemaError::unknown_model(source.name()));
            }
        }

        Ok(())
    }
}

impl Default for Schema<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl Print for Schema<'_> {
    const TAB_SIZE: usize = TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            data_source,
            enums,
            generators,
            models,
        } = self;

        for generator in generators.values() {
            generator.print(level, f)?;
            writeln!(f)?;
        }

        if let Some(data_source) = data_source {
            data_source.print(level, f)?;
            writeln!(f)?;
        }

        for r#enum in enums.values() {
            r#enum.print(level, f)?;
            writeln!(f)?;
        }

        for model in models.values() {
            model.print(level, f)?;
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<'a> TryFrom<ir::Ir<'a>> for Schema<'a> {
    type Error = SchemaError<'a>;

    fn try_from(value: ir::Ir<'a>) -> Result<Self, Self::Error> {
        let mut schema = Self::new();

        for (name, model) in value.models.clone() {
            let model = Model::try_from(model)?;

            if schema.models.insert(name.clone(), model).is_some() {
                return Err(SchemaError::duplicate_model(name));
            }
        }

        for model in value.models.into_values() {
            if schema.add_foreign_keys(&model).is_err() {
                return Err(SchemaError::duplicate_model(model.name()));
            }
        }

        for (name, r#enum) in value.enums {
            let r#enum = Enum::from(r#enum);

            if schema.enums.insert(name.clone(), r#enum).is_some() {
                return Err(SchemaError::duplicate_enum(name));
            }
        }

        Ok(schema)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        token_set::TokenSet,
    };

    #[test]
    fn test_new() {
        assert_eq!(
            Schema::new(),
            Schema {
                data_source: None,
                enums: OrdStrMap::new(),
                generators: OrdStrMap::new(),
                models: OrdStrMap::new(),
            }
        );
    }

    #[test]
    fn test_default() {
        assert_eq!(
            Schema::default(),
            Schema {
                data_source: None,
                enums: OrdStrMap::new(),
                generators: OrdStrMap::new(),
                models: OrdStrMap::new(),
            }
        );
    }

    #[test]
    fn test_from_ir() {
        let mut ir = ir::Ir::default();

        let ir_enum = ir::Enum {
            name: "Role".into(),
            values: TokenSet::from_iter(["USER", "ADMIN"]),
        };

        let mut user = ir::Model::new("User");

        user.insert_enum_relation("role", "Role").unwrap();
        user.insert_enums_relation("roles", "Role").unwrap();
        ir.insert_enum(ir_enum).unwrap();
        ir.insert_model(user).unwrap();

        let schema = Schema::try_from(ir).unwrap();
        let mut f = Vec::new();

        schema.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "enum Role {
  USER
  ADMIN
}

model User {
  id        Int      @id @default(autoincrement())
  createdAt DateTime @default(now())
  role      Role
  roles     Role[]
}

"
        );
    }

    #[test]
    #[allow(clippy::too_many_lines)]
    fn test_print() {
        let schema = Schema {
            data_source: Some(DataSource {
                name: "db".into(),
                provider: data_source::Provider::PostgreSql {
                    user: "user".into(),
                    password: "password".into(),
                    host: "localhost".into(),
                    port: 5432,
                    database: "database".into(),
                    schema: "public".into(),
                    extensions: Vec::new(),
                },
                shadow_database_url: Some(
                    "postgresql://user:password@localhost:5432/database".into(),
                ),
                direct_url: Some(
                    "postgresql://user:password@localhost:5432/database".into(),
                ),
                relation_mode: data_source::RelationMode::ForeignKeys,
            }),
            enums: OrdStrMap::from_iter([
                (
                    "Role",
                    Enum {
                        name: "Role".into(),
                        values: TokenSet::from_iter(["USER", "ADMIN"]),
                        attributes: Vec::new(),
                    },
                ),
                (
                    "Status",
                    Enum {
                        name: "Status".into(),
                        values: TokenSet::from_iter(["ACTIVE", "INACTIVE"]),
                        attributes: Vec::new(),
                    },
                ),
            ]),
            generators: OrdStrMap::from_iter([(
                "client",
                Generator {
                    name: "client".into(),
                    provider: generator::Provider::PrismaClientJs,
                    output: Some("path/to/client".into()),
                    binary_targets: vec![
                        generator::BinaryTarget::AlpineOpenSsl3_0,
                    ],
                    preview_features: vec![
                        generator::PreviewFeature::ExtendedWhereUnique,
                        generator::PreviewFeature::FullTextIndex,
                        generator::PreviewFeature::FullTextSearch,
                    ],
                    engine_type: Some(generator::EngineType::Binary),
                },
            )]),
            models: OrdStrMap::from_iter([(
                "User",
                Model {
                    name: "User".into(),
                    fields: OrdStrMap::from_iter([
                        ("id", model::Field::id()),
                        ("createdAt", model::Field::created_at()),
                    ]),
                    attributes: Vec::new(),
                },
            )]),
        };

        let mut f = Vec::new();

        schema.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "generator client {
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
  id        Int      @id @default(autoincrement())
  createdAt DateTime @default(now())
}

"
        );
    }

    #[test]
    fn test_one_to_one() {
        let source = "

model A {
    b: @B
}

model B {
    foo: String
    bar: Int
}

"
        .trim();

        let (ast, _) = ast::Ast::parse(source).unwrap();
        let ir = ir::Ir::try_from(ast).unwrap();
        let schema = Schema::try_from(ir).unwrap();
        let mut f = Vec::new();

        schema.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "model A {
  id        Int      @id @default(autoincrement())
  createdAt DateTime @default(now())
  b         B?       @relation(name: \"bOnA\")
}

model B {
  id        Int      @id @default(autoincrement())
  createdAt DateTime @default(now())
  foo       String
  bar       Int
  a         A        @relation(name: \"bOnA\", fields: [aId], references: [id])
  aId       Int      @unique
}

"
        );
    }

    #[test]
    fn test_one_to_many() {
        let source = "

model A {
    b: [@B]
}

model B {
    foo: String
    bar: Int
}

"
        .trim();

        let (ast, _) = ast::Ast::parse(source).unwrap();
        let ir = ir::Ir::try_from(ast).unwrap();
        let schema = Schema::try_from(ir).unwrap();
        let mut f = Vec::new();

        schema.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "model A {
  id        Int      @id @default(autoincrement())
  createdAt DateTime @default(now())
  b         B[]      @relation(name: \"bOnA\")
}

model B {
  id        Int      @id @default(autoincrement())
  createdAt DateTime @default(now())
  foo       String
  bar       Int
  a         A?       @relation(name: \"bOnA\", fields: [aId], references: [id])
  aId       Int?     @unique
}

"
        );
    }

    #[test]
    fn test_many_to_many() {
        let source = "

model A {
    b: [B]
}

model B {
    foo: String
    bar: Int
}

"
        .trim();

        let (ast, _) = ast::Ast::parse(source).unwrap();
        let ir = ir::Ir::try_from(ast).unwrap();
        let schema = Schema::try_from(ir).unwrap();
        let mut f = Vec::new();

        schema.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "model A {
  id        Int      @id @default(autoincrement())
  createdAt DateTime @default(now())
  b         B[]      @relation(name: \"bOnA\")
}

model B {
  id        Int      @id @default(autoincrement())
  createdAt DateTime @default(now())
  foo       String
  bar       Int
  a         A[]      @relation(name: \"bOnA\")
}

"
        );
    }
}
