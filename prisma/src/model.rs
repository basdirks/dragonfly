pub use field::Field;
use {
    crate::{
        attribute,
        schema_error::SchemaError,
        Argument,
        Value,
    },
    ir::{
        self,
        Cardinality,
    },
    ord_str_map::OrdStrMap,
    print::{
        Print,
        PrintInline,
    },
    std::{
        borrow::Cow,
        io,
    },
};

/// Model fields.
pub mod field;

/// A Prisma model.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Model<'a> {
    /// The name of the model. Must adhere to `[A-Za-z][A-Za-z0-9_]*`. Usually
    /// pascal case. May not be a reserved Prisma keyword or a JavaScript
    /// reserved keyword. Should be singular.
    pub name: Cow<'a, str>,
    /// The fields of the model.
    pub fields: OrdStrMap<Field<'a>>,
    /// Block attributes.
    pub attributes: Vec<attribute::Block<'a>>,
}

impl<'a> Model<'a> {
    /// Insert a field into the model.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `field` - The field to insert.
    ///
    /// # Errors
    ///
    /// Returns a `SchemaError::DuplicateModelField` if the field already
    /// exists.
    pub fn insert_field<S>(
        &mut self,
        name: S,
        field: Field<'a>,
    ) -> Result<(), SchemaError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
    {
        if self.fields.insert(name.clone().into(), field).is_some() {
            return Err(SchemaError::duplicate_model_field(
                self.name.clone(),
                name.into(),
            ));
        }

        Ok(())
    }
}

impl Print for Model<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            name,
            fields,
            attributes,
        } = self;

        let indent_outer = Self::indent(level);
        let indent_inner = Self::indent(level + 1);

        writeln!(f, "{indent_outer}model {name} {{")?;

        let mut max_field_type_length = 0;
        let mut max_field_name_length = 0;

        for field in fields.values() {
            let mut f = Vec::new();

            field.print_type(&mut f)?;

            max_field_type_length = max_field_type_length.max(f.len());
            max_field_name_length = max_field_name_length.max(field.name.len());
        }

        max_field_name_length += 1;

        for (field_name, field) in fields.iter() {
            write!(f, "{indent_inner}{field_name:<max_field_name_length$}")?;

            let mut g = Vec::new();

            field.print_type(&mut g)?;

            let r#type = String::from_utf8_lossy(&g);

            if field.attributes.is_empty() {
                write!(f, "{type}")?;
            } else {
                write!(f, "{type:<max_field_type_length$}")?;

                for attribute in field.attributes {
                    attribute.print(f)?;
                }
            }

            writeln!(f)?;
        }

        if !attributes.is_empty() {
            writeln!(f)?;

            for attribute in attributes {
                attribute.print(level + 1, f)?;
            }
        }

        writeln!(f, "{indent_outer}}}")?;

        Ok(())
    }
}

impl<'a> TryFrom<ir::Model<'a>> for Model<'a> {
    type Error = SchemaError<'a>;

    #[allow(clippy::too_many_lines)]
    fn try_from(ir_model: ir::Model<'a>) -> Result<Self, Self::Error> {
        let name = ir_model.name();

        let mut model = Self {
            name: name.clone(),
            fields: OrdStrMap::from_iter([
                ("id", Field::id()),
                ("createdAt", Field::created_at()),
            ]),
            attributes: Vec::new(),
        };

        for (name, field) in ir_model.fields {
            if model.fields.insert(name.clone(), field.into()).is_some() {
                return Err(SchemaError::duplicate_model_field(
                    model.name, name,
                ));
            }
        }

        for (relation_name, enum_relation) in ir_model.enums {
            let ir::model::EnumRelation { name, cardinality } = enum_relation;

            let modifier = match cardinality {
                Cardinality::One => field::Modifier::None,
                Cardinality::Many => field::Modifier::List,
            };

            let field = Field {
                r#type: field::Type::Name(name.clone()),
                name: name.clone(),
                modifier,
                attributes: Vec::new(),
            };

            model.insert_field(relation_name, field)?;
        }

        for (relation_name, relation) in ir_model.relations {
            match relation.r#type {
                ir::model::model_relation::Type::OneToOne => {
                    let field = Field {
                        name: relation_name.clone().into(),
                        r#type: field::Type::Name(relation.model_name.clone()),
                        modifier: field::Modifier::Optional,
                        attributes: vec![{
                            attribute::Field {
                                group: None,
                                name: "relation".into(),
                                arguments: vec![Argument {
                                    name: Some("name".into()),
                                    value: Value::String(
                                        format!("{relation_name}On{name}")
                                            .into(),
                                    ),
                                }],
                            }
                        }],
                    };

                    model.insert_field(relation_name, field)?;
                }
                ir::model::model_relation::Type::OneToMany
                | ir::model::model_relation::Type::ManyToMany => {
                    let field = Field {
                        name: relation_name.clone().into(),
                        r#type: field::Type::Name(relation.model_name.clone()),
                        modifier: field::Modifier::List,
                        attributes: vec![{
                            attribute::Field {
                                group: None,
                                name: "relation".into(),
                                arguments: vec![Argument {
                                    name: Some("name".into()),
                                    value: Value::String(
                                        format!("{relation_name}On{name}")
                                            .into(),
                                    ),
                                }],
                            }
                        }],
                    };

                    model.insert_field(relation_name, field)?;
                }
                ir::model::model_relation::Type::ManyToOne => {
                    let field = Field {
                        name: relation_name.clone().into(),
                        r#type: field::Type::Name(relation.model_name.clone()),
                        modifier: field::Modifier::Optional,
                        attributes: vec![{
                            attribute::Field {
                                group: None,
                                name: "relation".into(),
                                arguments: vec![
                                    Argument {
                                        name: Some("name".into()),
                                        value: Value::String(
                                            format!("{relation_name}On{name}")
                                                .into(),
                                        ),
                                    },
                                    Argument {
                                        name: Some("fields".into()),
                                        value: Value::Array(vec![
                                            Value::Keyword(
                                                format!("{relation_name}Id")
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
                            }
                        }],
                    };

                    model.insert_field(relation_name.clone(), field)?;

                    let field_name = format!("{relation_name}Id");

                    let field = Field {
                        name: field_name.clone().into(),
                        r#type: field::Type::Name("Int".into()),
                        modifier: field::Modifier::Optional,
                        attributes: vec![attribute::Field::unique()],
                    };

                    model.insert_field(field_name, field)?;
                }
            };
        }

        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_ir_model() {
        let mut ir_model = ir::Model::new("User");

        ir_model.insert_enums_relation("roles", "Role").unwrap();
        ir_model.insert_enum_relation("role", "Role").unwrap();

        ir_model
            .insert_field(ir::model::Field {
                name: "age".into(),
                r#type: ir::Type::Int,
                cardinality: ir::Cardinality::One,
            })
            .unwrap();

        ir_model.insert_one_to_one("profile", "Profile").unwrap();
        ir_model.insert_one_to_many("posts", "Post").unwrap();
        ir_model.insert_many_to_one("country", "Country").unwrap();
        ir_model.insert_many_to_many("friends", "User").unwrap();

        let model: Model = ir_model.try_into().unwrap();
        let mut f = Vec::new();

        model.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "model User {
  id        Int      @id @default(autoincrement())
  createdAt DateTime @default(now())
  age       Int
  roles     Role[]
  role      Role
  profile   Profile? @relation(name: \"profileOnUser\")
  posts     Post[]   @relation(name: \"postsOnUser\")
  country   Country? @relation(name: \"countryOnUser\", fields: [countryId], \
             references: [id])
  countryId Int?     @unique
  friends   User[]   @relation(name: \"friendsOnUser\")
}
"
        );
    }

    #[test]
    fn test_try_from_ir_model_duplicate_field() {
        let mut ir_model = ir::Model::new("User");

        ir_model
            .insert_field(ir::model::Field {
                name: "id".into(),
                r#type: ir::Type::Int,
                cardinality: Cardinality::One,
            })
            .unwrap();

        let model: Result<Model, _> = ir_model.try_into();

        assert_eq!(
            model.unwrap_err(),
            SchemaError::duplicate_model_field("User", "id")
        );
    }

    #[test]
    fn test_try_from_ir_model_duplicate_enum_field() {
        let mut ir_model = ir::Model::new("User");

        ir_model.insert_enum_relation("id", "ID").unwrap();

        let model: Result<Model, _> = ir_model.try_into();

        assert_eq!(
            model.unwrap_err(),
            SchemaError::duplicate_model_field("User", "id")
        );
    }

    #[test]
    fn test_try_from_ir_model_duplicate_relation_field() {
        let mut ir_model = ir::Model::new("User");

        ir_model.insert_one_to_one("id", "Identity").unwrap();

        let model: Result<Model, _> = ir_model.try_into();

        assert_eq!(
            model.unwrap_err(),
            SchemaError::duplicate_model_field("User", "id")
        );
    }
}
