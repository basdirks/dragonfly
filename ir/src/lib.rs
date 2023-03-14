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
    rust_2018_idioms,
    variant_size_differences
)]

//! The intermediate representation (IR) of the AST.
//!
//! Conversion from AST to IR also performs type checking.

pub use self::{
    cardinality::Cardinality,
    model::{
        EnumRelation,
        Field,
        Model,
        Relation as ModelRelation,
        RelationType,
    },
    query::{
        Argument as QueryArgument,
        ArgumentType as QueryArgumentType,
        Condition as QueryCondition,
        Operator as QueryOperator,
        Query,
        ReturnType as QueryReturnType,
        Schema as QuerySchema,
        SchemaNode as QuerySchemaNode,
        Where as QueryWhere,
    },
    r#enum::Enum,
    r#type::Type,
    type_error::TypeError,
};
use {
    ord_str_map::OrdStrMap,
    std::{
        borrow::Cow,
        collections::{
            BTreeSet,
            VecDeque,
        },
    },
};

/// Cardinality.
pub mod cardinality;
/// Enums.
pub mod r#enum;
/// Models.
pub mod model;
/// Queries.
pub mod query;
/// Types.
pub mod r#type;
/// Type errors.
pub mod type_error;

/// The intermediate representation (IR) of the AST.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ir<'a> {
    /// The models in the data model.
    pub models: OrdStrMap<Model<'a>>,
    /// The enums in the data model.
    pub enums: OrdStrMap<Enum<'a>>,
    /// Queries.
    pub queries: OrdStrMap<Query<'a>>,
}

impl<'a> Ir<'a> {
    /// Resolve the type of a model field.
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the model.
    /// * `path` - The path to the field.
    #[must_use]
    pub fn field_type<S>(
        &self,
        model_name: S,
        mut path: VecDeque<Cow<'a, str>>,
    ) -> Option<Type>
    where
        S: AsRef<str>,
    {
        let mut current_model = self.models.get(model_name.as_ref())?;

        while let Some(segment) = path.pop_front() {
            if path.is_empty() {
                return current_model.field(&segment).map(|field| field.r#type);
            }

            if let Some(ModelRelation {
                model_name: name, ..
            }) = current_model.model_relation(&segment)
            {
                if let Some(model) = self.models.get(&name) {
                    current_model = model;
                    continue;
                }
            }

            return None;
        }

        None
    }

    /// Resolve the enum type of a model field.
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the model.
    /// * `path` - The path to the enum.
    #[must_use]
    pub fn enum_type<S>(
        &self,
        model_name: S,
        mut path: VecDeque<Cow<'a, str>>,
    ) -> Option<Cow<'a, str>>
    where
        S: AsRef<str>,
    {
        let mut current_model = self.models.get(model_name.as_ref())?;

        while let Some(segment) = path.pop_front() {
            if path.is_empty() {
                return current_model.enum_relation(&segment).map(|r| r.name);
            }

            if let Some(ModelRelation {
                model_name: name, ..
            }) = current_model.model_relation(&segment)
            {
                if let Some(model) = self.models.get(&name) {
                    current_model = model;
                    continue;
                }
            }

            return None;
        }

        None
    }

    /// Insert a model.
    ///
    /// # Arguments
    ///
    /// * `model` - The model to insert.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if a model with the same name already exists.
    pub fn insert_model(
        &mut self,
        model: Model<'a>,
    ) -> Result<(), TypeError<'a>> {
        let model_name = model.name();

        if self.models.insert(model_name.clone(), model).is_some() {
            return Err(TypeError::duplicate_model(model_name));
        }

        Ok(())
    }

    /// Insert a query.
    ///
    /// # Arguments
    ///
    /// * `query` - The query to insert.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if a query with the same name already exists.
    pub fn insert_query(
        &mut self,
        query: Query<'a>,
    ) -> Result<(), TypeError<'a>> {
        let query_name = query.name.clone();

        if self.queries.insert(query_name.clone(), query).is_some() {
            return Err(TypeError::duplicate_query(query_name));
        }

        Ok(())
    }

    /// Insert an enum.
    ///
    /// # Arguments
    ///
    /// * `r#enum` - The enum to insert.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if an enum with the same name already exists.
    pub fn insert_enum(
        &mut self,
        r#enum: Enum<'a>,
    ) -> Result<(), TypeError<'a>> {
        let enum_name = r#enum.name.clone();

        if self.enums.insert(enum_name.clone(), r#enum).is_some() {
            return Err(TypeError::duplicate_enum(enum_name));
        }

        Ok(())
    }

    /// Check compatibility between argument type and field type.
    ///
    /// # Arguments
    ///
    /// * `model_name` - The name of the model.
    /// * `path` - The path to the field.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the types are incompatible.
    #[must_use]
    pub fn check_argument_type<S>(
        &self,
        model_name: S,
        path: VecDeque<Cow<'a, str>>,
        argument_type: QueryArgumentType<'a>,
    ) -> bool
    where
        S: AsRef<str>,
    {
        match argument_type {
            QueryArgumentType::Enum(rhs) => {
                self.enum_type(model_name, path)
                    .map_or(false, |lhs| lhs == rhs)
            }
            QueryArgumentType::Type(rhs) => {
                self.field_type(model_name, path)
                    .map_or(false, |lhs| lhs == rhs)
            }
        }
    }

    /// Build a schema from an AST schema.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `ast_schema` - The AST schema.
    /// * `model` - The current model.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the schema is invalid.
    pub fn query_schema<S>(
        &self,
        query_name: &S,
        ast_schema: &ast::QuerySchema<'a>,
        model: &Model<'a>,
    ) -> Result<QuerySchema<'a>, TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
    {
        let mut nodes = Vec::new();

        for node in ast_schema.nodes.clone() {
            nodes.push(self.query_schema_node(
                query_name.clone(),
                node,
                model,
                VecDeque::new(),
            )?);
        }

        Ok(QuerySchema {
            alias: ast_schema.name.clone(),
            nodes,
        })
    }

    /// Create a query schema node from an AST query schema node.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `ast_node` - The AST query schema node.
    /// * `model` - The current model.
    /// * `path` - The path to the current node.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the schema node is invalid.
    pub fn query_schema_node<S>(
        &self,
        query_name: S,
        ast_node: ast::QuerySchemaNode<'a>,
        model: &Model<'a>,
        mut path: VecDeque<Cow<'a, str>>,
    ) -> Result<QuerySchemaNode<'a>, TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
    {
        match ast_node {
            ast::QuerySchemaNode::Field { name } => {
                path.push_back(name.clone());

                if self.field_type(model.name(), path.clone()).is_some() {
                    Ok(QuerySchemaNode::Field { name })
                } else {
                    Err(TypeError::undefined_query_field(
                        query_name,
                        path.into_iter().collect::<Vec<_>>().join("."),
                    ))
                }
            }
            ast::QuerySchemaNode::Relation {
                name: ast_name,
                nodes: ast_nodes,
            } => {
                let mut nodes = Vec::new();

                for ast_node in ast_nodes {
                    let mut path = path.clone();

                    path.push_back(ast_name.clone());

                    if let Some(model) = self.models.get(&model.name()) {
                        nodes.push(self.query_schema_node(
                            query_name.clone(),
                            ast_node,
                            model,
                            path,
                        )?);
                    } else {
                        return Err(TypeError::undefined_query_field(
                            query_name,
                            path.into_iter().collect::<Vec<_>>().join("."),
                        ));
                    }
                }

                Ok(QuerySchemaNode::Relation {
                    name: ast_name,
                    nodes,
                })
            }
        }
    }

    /// Create a query return type from an AST query return type.
    ///
    /// # Arguments
    ///
    /// * `query_name` - The name of the query.
    /// * `ast_return_type` - The AST query return type.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the return type is invalid.
    pub fn query_return_type<S>(
        &self,
        query_name: S,
        ast_return_type: ast::QueryReturnType<'a>,
    ) -> Result<(QueryReturnType<'a>, Model<'a>), TypeError<'a>>
    where
        S: Into<Cow<'a, str>>,
    {
        let return_type = QueryReturnType::from(ast_return_type.clone());

        self.models.get(&return_type.model_name).map_or_else(
            || {
                Err(TypeError::undefined_query_return_type(
                    query_name,
                    match ast_return_type {
                        ast::QueryReturnType::Model(name)
                        | ast::QueryReturnType::Array(name) => name,
                    },
                ))
            },
            |model| Ok((return_type, model.clone())),
        )
    }

    /// Add an AST model to the IR.
    ///
    /// # Arguments
    ///
    /// * `ast_model` - The AST model.
    /// * `enum_names` - The names of the enums.
    /// * `model_names` - The names of the models.
    #[allow(clippy::too_many_lines)]
    fn add_model(
        &mut self,
        ast_model: &ast::Model<'a>,
        enum_names: &BTreeSet<Cow<'a, str>>,
        model_names: &BTreeSet<Cow<'a, str>>,
    ) -> Result<(), TypeError<'a>> {
        let ast::Model { fields, .. } = ast_model;
        let mut model = Model::new(ast_model.name.clone());

        for field in fields.values() {
            let field_name = field.name.clone();

            match &field.r#type {
                ast::Type::Scalar(ast::Scalar::Boolean) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::Boolean,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::Scalar::DateTime) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::DateTime,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::Scalar::Float) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::Float,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::Scalar::Int) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::Int,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::Scalar::String) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::String,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::Scalar::Reference(name)) => {
                    if enum_names.contains(name) {
                        model.insert_enum_relation(field_name, name.clone())
                    } else if model_names.contains(name) {
                        model.insert_many_to_one(field_name, name.clone())
                    } else {
                        Err(TypeError::unknown_model_field_type(
                            model.name(),
                            field_name,
                            field.r#type.to_string(),
                        ))
                    }
                }
                ast::Type::Scalar(ast::Scalar::Owned(name)) => {
                    if model_names.contains(name) {
                        model.insert_one_to_one(field_name, name.clone())
                    } else {
                        Err(TypeError::unknown_model_field_type(
                            model.name(),
                            field_name,
                            field.r#type.to_string(),
                        ))
                    }
                }
                ast::Type::Array(ast::Scalar::Boolean) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::Boolean,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::Scalar::DateTime) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::DateTime,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::Scalar::Float) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::Float,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::Scalar::Int) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::Int,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::Scalar::String) => {
                    model.insert_field(Field {
                        name: field_name,
                        r#type: Type::String,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::Scalar::Reference(name)) => {
                    if enum_names.contains(name) {
                        model.insert_enums_relation(field_name, name.clone())
                    } else if model_names.contains(name) {
                        model.insert_many_to_many(field_name, name.clone())
                    } else {
                        Err(TypeError::unknown_model_field_type(
                            model.name(),
                            field_name,
                            field.r#type.to_string(),
                        ))
                    }
                }
                ast::Type::Array(ast::Scalar::Owned(name)) => {
                    if model_names.contains(name) {
                        model.insert_one_to_many(field_name, name.clone())
                    } else {
                        Err(TypeError::unknown_model_field_type(
                            model.name(),
                            field_name,
                            field.r#type.to_string(),
                        ))
                    }
                }
            }?;
        }

        self.insert_model(model)
    }

    /// Add an AST query to the IR.
    ///
    /// # Arguments
    ///
    /// * `ast_query` - The AST query.
    /// * `enum_names` - The names of the enums.
    fn add_query(
        &mut self,
        ast_query: &ast::Query<'a>,
        enum_names: &BTreeSet<Cow<'a, str>>,
    ) -> Result<(), TypeError<'a>> {
        if let Some(ast_where) = ast_query.r#where.clone() {
            if ast_query.schema.name != ast_where.name {
                return Err(TypeError::invalid_query_where_name(
                    ast_query.name.clone(),
                    ast_query.schema.name.clone(),
                    ast_where.name,
                ));
            }
        }

        let (return_type, model) = self.query_return_type(
            ast_query.name.clone(),
            ast_query.r#type.clone(),
        )?;

        let mut query = Query::new(
            ast_query.name.clone(),
            return_type,
            ast_query.schema.name.clone(),
        );

        for (argument_name, ast_argument) in ast_query.arguments.iter() {
            if let Some(argument) =
                QueryArgument::from_ast_type(&ast_argument, enum_names)
            {
                let _: Option<QueryArgument<'a>> =
                    query.arguments.insert(argument_name, argument);
            };
        }

        query.schema =
            self.query_schema(&ast_query.name, &ast_query.schema, &model)?;

        if let Some(ast::query::Where {
            conditions: ast_conditions,
            name: alias,
        }) = ast_query.r#where.clone()
        {
            let mut conditions = Vec::new();

            for ast_condition in ast_conditions {
                let path = ast_condition.path.0.clone();
                let model_name = query.r#type.model_name.clone();

                if !query.arguments.iter().any(|(name, argument)| {
                    name == ast_condition.argument_name
                        && self.check_argument_type(
                            &model_name,
                            path.clone(),
                            argument.r#type,
                        )
                }) {
                    return Err(TypeError::invalid_query_condition(
                        ast_query.name.clone(),
                        ast_condition.path.clone().to_string(),
                        ast_condition.argument_name,
                        ast_condition.operator.to_string(),
                    ));
                }

                conditions.push(QueryCondition {
                    lhs: path.clone(),
                    operator: ast_condition.operator.into(),
                    rhs: ast_condition.argument_name.clone(),
                });
            }

            query.r#where = Some(QueryWhere { alias, conditions });
        }

        if self.queries.insert(ast_query.name.clone(), query).is_some() {
            return Err(TypeError::duplicate_query(ast_query.name.clone()));
        }

        Ok(())
    }
}

impl Default for Ir<'_> {
    fn default() -> Self {
        Self {
            enums: OrdStrMap::new(),
            models: OrdStrMap::new(),
            queries: OrdStrMap::new(),
        }
    }
}

impl<'a> TryFrom<ast::Ast<'a>> for Ir<'a> {
    type Error = TypeError<'a>;

    fn try_from(value: ast::Ast<'a>) -> Result<Self, Self::Error> {
        let ast::Ast {
            models: ast_models,
            enums: ast_enums,
            queries: ast_queries,
        } = value;

        let enum_names = ast_enums
            .values()
            .map(|e| e.name.clone())
            .collect::<BTreeSet<_>>();

        let model_names = ast_models
            .values()
            .map(|m| m.name.clone())
            .collect::<BTreeSet<_>>();

        let mut ir = Self::default();

        for model in ast_models.values() {
            ir.add_model(model, &enum_names, &model_names)?;
        }

        for ast_enum in ast_enums.into_values() {
            ir.insert_enum(ast_enum.into())?;
        }

        for ast_query in ast_queries.values() {
            ir.add_query(ast_query, &enum_names)?;
        }

        Ok(ir)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::iter::once,
        token_set::TokenSet,
    };

    #[test]
    fn test_new() {
        let ir = Ir::default();

        assert!(ir.models.is_empty());
        assert!(ir.enums.is_empty());
        assert!(ir.queries.is_empty());
    }

    #[test]
    fn test_resolve_model_field() -> Result<(), TypeError<'static>> {
        let mut ir = Ir::default();
        let mut user_model = Model::new("User");
        let mut address_model = Model::new("Address");
        let mut postbox_model = Model::new("Postbox");

        user_model.insert_field(Field {
            name: "name".into(),
            r#type: Type::String,
            cardinality: Cardinality::One,
        })?;

        address_model.insert_field(Field {
            name: "street".into(),
            r#type: Type::String,
            cardinality: Cardinality::One,
        })?;

        postbox_model.insert_field(Field {
            name: "number".into(),
            r#type: Type::Int,
            cardinality: Cardinality::One,
        })?;

        address_model.insert_one_to_one("postbox", "Postbox")?;
        user_model.insert_many_to_one("address", "Address")?;
        ir.insert_model(user_model)?;
        ir.insert_model(address_model)?;
        ir.insert_model(postbox_model)?;

        assert_eq!(
            ir.field_type("User", once("name").map(Into::into).collect()),
            Some(Type::String),
        );

        assert_eq!(
            ir.field_type(
                "User",
                ["address", "postbox", "number"]
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
            Some(Type::Int),
        );

        assert_eq!(
            ir.field_type(
                "User",
                ["address", "street"].into_iter().map(Into::into).collect(),
            ),
            Some(Type::String),
        );

        assert_eq!(
            ir.field_type(
                "User",
                ["address", "postbox", "street"]
                    .into_iter()
                    .map(Into::into)
                    .collect(),
            ),
            None,
        );

        assert_eq!(ir.field_type("User", VecDeque::new()), None);

        Ok(())
    }

    #[test]
    fn test_resolve_model_enum() -> Result<(), TypeError<'static>> {
        let mut ir = Ir::default();
        let mut user_model = Model::new("User");
        let mut address_model = Model::new("Address");

        let address_type = Enum {
            name: "AddressType".into(),
            values: TokenSet::from_iter(["home", "work"]),
        };

        user_model.insert_field(Field {
            name: "name".into(),
            r#type: Type::String,
            cardinality: Cardinality::One,
        })?;

        user_model.insert_many_to_one("address", "Address")?;
        user_model.insert_one_to_one("socials", "Socials")?;
        address_model.insert_enum_relation("type", "AddressType")?;
        ir.insert_model(user_model)?;
        ir.insert_model(address_model)?;
        ir.insert_enum(address_type)?;

        assert_eq!(
            ir.enum_type("User", once("name").map(Into::into).collect()),
            None,
        );

        assert_eq!(
            ir.enum_type(
                "User",
                ["address", "type"].into_iter().map(Into::into).collect()
            ),
            Some("AddressType".into()),
        );

        assert_eq!(
            ir.enum_type(
                "User",
                ["address", "street"].into_iter().map(Into::into).collect()
            ),
            None,
        );

        assert_eq!(
            ir.enum_type(
                "User",
                ["socials", "facebook"]
                    .into_iter()
                    .map(Into::into)
                    .collect()
            ),
            None,
        );

        Ok(())
    }

    #[test]
    fn test_check_argument_type() -> Result<(), TypeError<'static>> {
        let mut ir = Ir::default();
        let mut user_model = Model::new("User");
        let mut address_model = Model::new("Address");

        let address_type = Enum {
            name: "AddressType".into(),
            values: TokenSet::from_iter(["home", "work"]),
        };

        user_model.insert_field(Field {
            name: "name".into(),
            r#type: Type::String,
            cardinality: Cardinality::One,
        })?;

        user_model.insert_many_to_one("address", "Address")?;
        user_model.insert_one_to_one("socials", "Socials")?;
        address_model.insert_enum_relation("type", "AddressType")?;
        ir.insert_model(user_model)?;
        ir.insert_model(address_model)?;
        ir.insert_enum(address_type)?;

        assert!(ir.check_argument_type(
            "User",
            once("name").map(Into::into).collect(),
            QueryArgumentType::Type(Type::String)
        ));

        assert!(ir.check_argument_type(
            "User",
            ["address", "type"].into_iter().map(Into::into).collect(),
            QueryArgumentType::Enum("AddressType".into())
        ));

        assert!(!ir.check_argument_type(
            "User",
            ["address", "street"].into_iter().map(Into::into).collect(),
            QueryArgumentType::Type(Type::String)
        ));

        assert!(!ir.check_argument_type(
            "User",
            ["socials", "facebook"]
                .into_iter()
                .map(Into::into)
                .collect(),
            QueryArgumentType::Type(Type::String)
        ));

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_from_ast_full() -> Result<(), TypeError<'static>> {
        let source = "

model User {
  name: String
  age: Int
  daBoi: Boolean
  addresses: [Address]
  profile: @Profile
}

model Profile {
  bio: String
  createdAt: DateTime
}

model Address {
  street: String
  number: Int
  type: AddressType
}

enum AddressType {
  Home
  Work
  Other
}

query users($addressType: AddressType): [User] {
  user {
    name
    age
    daBoi
    addresses {
      street
      number
    }
  }
  where {
    user {
      addresses {
        type {
          equals: $addressType
        }
      }
    }
  }
}

                "
        .trim();

        let (ast, _) = ast::Ast::parse(source).unwrap();
        let ir = Ir::try_from(ast).unwrap();

        assert_eq!(
            ir,
            Ir {
                queries: OrdStrMap::from_iter([(
                    "users",
                    Query {
                        name: "users".into(),
                        arguments: OrdStrMap::from_iter([(
                            "addressType",
                            QueryArgument {
                                name: "addressType".into(),
                                r#type: QueryArgumentType::Enum(
                                    "AddressType".into()
                                ),
                                cardinality: Cardinality::One,
                            }
                        )]),
                        r#type: QueryReturnType {
                            model_name: "User".into(),
                            cardinality: Cardinality::Many,
                        },
                        schema: QuerySchema {
                            alias: "user".into(),
                            nodes: vec![
                                QuerySchemaNode::Field {
                                    name: "name".into()
                                },
                                QuerySchemaNode::Field { name: "age".into() },
                                QuerySchemaNode::Field {
                                    name: "daBoi".into()
                                },
                                QuerySchemaNode::Relation {
                                    name: "addresses".into(),
                                    nodes: vec![
                                        QuerySchemaNode::Field {
                                            name: "street".into()
                                        },
                                        QuerySchemaNode::Field {
                                            name: "number".into()
                                        },
                                    ],
                                },
                            ]
                        },
                        r#where: Some(QueryWhere {
                            alias: "user".into(),
                            conditions: vec![QueryCondition {
                                lhs: ["addresses", "type"]
                                    .into_iter()
                                    .map(Into::into)
                                    .collect(),
                                operator: QueryOperator::Equals,
                                rhs: "addressType".into(),
                            }]
                        }),
                    }
                )]),
                models: OrdStrMap::from_iter([
                    ("User", {
                        let mut model = Model::new("User");

                        model.insert_field(Field {
                            name: "name".into(),
                            r#type: Type::String,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_field(Field {
                            name: "age".into(),
                            r#type: Type::Int,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_field(Field {
                            name: "daBoi".into(),
                            r#type: Type::Boolean,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_many_to_many("addresses", "Address")?;
                        model.insert_one_to_one("profile", "Profile")?;

                        model
                    }),
                    ("Profile", {
                        let mut model = Model::new("Profile");

                        model.insert_field(Field {
                            name: "bio".into(),
                            r#type: Type::String,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_field(Field {
                            name: "createdAt".into(),
                            r#type: Type::DateTime,
                            cardinality: Cardinality::One,
                        })?;

                        model
                    }),
                    ("Address", {
                        let mut model = Model::new("Address");

                        model.insert_field(Field {
                            name: "street".into(),
                            r#type: Type::String,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_field(Field {
                            name: "number".into(),
                            r#type: Type::Int,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_enum_relation("type", "AddressType")?;

                        model
                    })
                ]),
                enums: OrdStrMap::from_iter([(
                    "AddressType",
                    Enum {
                        name: "AddressType".into(),
                        values: TokenSet::from_iter(["Home", "Work", "Other"])
                    }
                )])
            }
        );

        Ok(())
    }

    #[test]
    fn test_duplicate_model() {
        let mut ir = Ir::default();

        ir.insert_model(Model::new("User")).unwrap();

        assert_eq!(
            ir.insert_model(Model::new("User")),
            Err(TypeError::duplicate_model("User"))
        );
    }

    #[test]
    fn test_duplicate_enum() {
        let mut ir = Ir::default();

        ir.insert_enum(Enum {
            name: "AddressType".into(),
            values: TokenSet::from_iter(["Home", "Work", "Other"]),
        })
        .unwrap();

        assert_eq!(
            ir.insert_enum(Enum {
                name: "AddressType".into(),
                values: TokenSet::new()
            }),
            Err(TypeError::duplicate_enum("AddressType"))
        );
    }

    #[test]
    fn test_duplicate_query() {
        let mut ir = Ir::default();

        let query = Query {
            name: "users".into(),
            arguments: OrdStrMap::new(),
            schema: QuerySchema {
                alias: "user".into(),
                nodes: Vec::new(),
            },
            r#where: None,
            r#type: QueryReturnType {
                model_name: "User".into(),
                cardinality: Cardinality::Many,
            },
        };

        ir.insert_query(query.clone()).unwrap();

        assert_eq!(
            ir.insert_query(query),
            Err(TypeError::duplicate_query("users"))
        );
    }

    #[test]
    fn test_undefined_query_field() {
        let source = "

model User {
    name: String
    age: Int
}

query users: [User] {
    user {
        name
        age
        address
    }
}

"
        .trim();

        let (ast, _) = ast::Ast::parse(source).unwrap();
        let ir = Ir::try_from(ast);

        assert_eq!(
            ir,
            Err(TypeError::undefined_query_field("users", "address"))
        );
    }

    #[test]
    fn test_undefined_query_field_relation() {
        let source = "

model User {
    name: String
    age: Int
}

query users: [User] {
    user {
        name
        age
        address {
            street
        }
    }
}

"
        .trim();

        let (ast, _) = ast::Ast::parse(source).unwrap();
        let ir = Ir::try_from(ast);

        assert_eq!(
            ir,
            Err(TypeError::undefined_query_field("users", "address.street"))
        );
    }

    #[test]
    fn test_undefined_query_return_type() {
        let source = "

query users: [User] {
    user {
        name
        age
    }
}

"
        .trim();

        let (ast, _) = ast::Ast::parse(source).unwrap();
        let ir = Ir::try_from(ast);

        assert_eq!(
            ir,
            Err(TypeError::undefined_query_return_type("users", "User"))
        );
    }
}
