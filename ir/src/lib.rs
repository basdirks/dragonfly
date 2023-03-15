//! The intermediate representation (IR) of the AST.
//!
//! Conversion from `ast::Ast` to `ir::Ir` also performs type checking.
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

pub use self::{
    cardinality::Cardinality,
    model::Model,
    query::Query,
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

            if let Some(model::ModelRelation {
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

            if let Some(model::ModelRelation {
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
        argument_type: query::ArgumentType<'a>,
    ) -> bool
    where
        S: AsRef<str>,
    {
        match argument_type {
            query::ArgumentType::Enum(rhs) => {
                self.enum_type(model_name, path)
                    .map_or(false, |lhs| lhs == rhs)
            }
            query::ArgumentType::Type(rhs) => {
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
        ast_schema: &ast::query::Schema<'a>,
        model: &Model<'a>,
    ) -> Result<query::Schema<'a>, TypeError<'a>>
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

        Ok(query::Schema {
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
        ast_node: ast::query::schema::Node<'a>,
        model: &Model<'a>,
        mut path: VecDeque<Cow<'a, str>>,
    ) -> Result<query::schema::Node<'a>, TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
    {
        match ast_node {
            ast::query::schema::Node::Field { name } => {
                path.push_back(name.clone());

                if self.field_type(model.name(), path.clone()).is_some()
                    || self.enum_type(model.name(), path.clone()).is_some()
                {
                    Ok(query::schema::Node::Field { name })
                } else {
                    Err(TypeError::undefined_query_field(
                        query_name,
                        path.into_iter().collect::<Vec<_>>().join("."),
                    ))
                }
            }
            ast::query::schema::Node::Relation {
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

                Ok(query::schema::Node::Relation {
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
        ast_return_type: ast::query::ReturnType<'a>,
    ) -> Result<(query::ReturnType<'a>, Model<'a>), TypeError<'a>>
    where
        S: Into<Cow<'a, str>>,
    {
        let return_type = query::ReturnType::from(ast_return_type.clone());

        self.models.get(&return_type.model_name).map_or_else(
            || {
                Err(TypeError::undefined_query_return_type(
                    query_name,
                    match ast_return_type {
                        ast::query::ReturnType::Model(name)
                        | ast::query::ReturnType::Array(name) => name,
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
                ast::Type::Scalar(ast::r#type::Scalar::Boolean) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::Boolean,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::r#type::Scalar::DateTime) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::DateTime,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::r#type::Scalar::Float) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::Float,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::r#type::Scalar::Int) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::Int,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::r#type::Scalar::String) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::String,
                        cardinality: Cardinality::One,
                    })
                }
                ast::Type::Scalar(ast::r#type::Scalar::Reference(name)) => {
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
                ast::Type::Scalar(ast::r#type::Scalar::Owned(name)) => {
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
                ast::Type::Array(ast::r#type::Scalar::Boolean) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::Boolean,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::r#type::Scalar::DateTime) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::DateTime,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::r#type::Scalar::Float) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::Float,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::r#type::Scalar::Int) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::Int,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::r#type::Scalar::String) => {
                    model.insert_field(model::Field {
                        name: field_name,
                        r#type: Type::String,
                        cardinality: Cardinality::Many,
                    })
                }
                ast::Type::Array(ast::r#type::Scalar::Reference(name)) => {
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
                ast::Type::Array(ast::r#type::Scalar::Owned(name)) => {
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
                query::Argument::from_ast_type(&ast_argument, enum_names)
            {
                let _: Option<query::Argument<'a>> =
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

                conditions.push(query::Condition {
                    lhs: path.clone(),
                    operator: ast_condition.operator.into(),
                    rhs: ast_condition.argument_name.clone(),
                });
            }

            query.r#where = Some(query::Where { alias, conditions });
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

        user_model.insert_field(model::Field {
            name: "name".into(),
            r#type: Type::String,
            cardinality: Cardinality::One,
        })?;

        address_model.insert_field(model::Field {
            name: "street".into(),
            r#type: Type::String,
            cardinality: Cardinality::One,
        })?;

        postbox_model.insert_field(model::Field {
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

        user_model.insert_field(model::Field {
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

        user_model.insert_field(model::Field {
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
            query::ArgumentType::Type(Type::String)
        ));

        assert!(ir.check_argument_type(
            "User",
            ["address", "type"].into_iter().map(Into::into).collect(),
            query::ArgumentType::Enum("AddressType".into())
        ));

        assert!(!ir.check_argument_type(
            "User",
            ["address", "street"].into_iter().map(Into::into).collect(),
            query::ArgumentType::Type(Type::String)
        ));

        assert!(!ir.check_argument_type(
            "User",
            ["socials", "facebook"]
                .into_iter()
                .map(Into::into)
                .collect(),
            query::ArgumentType::Type(Type::String)
        ));

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_try_from_ast_full() -> Result<(), TypeError<'static>> {
        let source = "

model Alpha {
    myBoolean: Boolean
    myDateTime: DateTime
    myFloat: Float
    myInt: Int
    myString: String
    myBooleans: [Boolean]
    myDateTimes: [DateTime]
    myFloats: [Float]
    myInts: [Int]
    myStrings: [String]
    myZeta: Zeta
    myZetas: [Zeta]
    myBeta: @Beta
    myGammas: [@Gamma]
    myDelta: Delta
    myEpsilons: [Epsilon]
}

model Beta {
    foo: String
    myGamma: Gamma
}

model Gamma {
    foo: String
}

model Delta {
    foo: String
}

model Epsilon {
    foo: String
}

enum Zeta {
    Theta
    Iota
}

query myQuery($booleanArgument: Boolean): [Alpha] {
  alpha {
    myBoolean
    myDateTime
    myFloat
    myInt
    myString
    myBooleans
    myDateTimes
    myFloats
    myInts
    myStrings
    myZeta
    myZetas
    myBeta {
      foo
      myGamma {
        foo
      }
    }
    myGammas {
      foo
    }
    myDelta {
      foo
    }
    myEpsilons {
      foo
    }
  }
  where {
    alpha {
        myBoolean {
            equals: $booleanArgument
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
                    "myQuery",
                    Query {
                        name: "myQuery".into(),
                        arguments: OrdStrMap::from_iter([(
                            "booleanArgument",
                            query::Argument {
                                name: "booleanArgument".into(),
                                r#type: query::ArgumentType::Type(
                                    Type::Boolean
                                ),
                                cardinality: Cardinality::One,
                            }
                        )]),
                        r#type: query::ReturnType {
                            model_name: "Alpha".into(),
                            cardinality: Cardinality::Many,
                        },
                        schema: query::Schema {
                            alias: "alpha".into(),
                            nodes: vec![
                                query::schema::Node::Field {
                                    name: "myBoolean".into()
                                },
                                query::schema::Node::Field {
                                    name: "myDateTime".into()
                                },
                                query::schema::Node::Field {
                                    name: "myFloat".into()
                                },
                                query::schema::Node::Field {
                                    name: "myInt".into()
                                },
                                query::schema::Node::Field {
                                    name: "myString".into()
                                },
                                query::schema::Node::Field {
                                    name: "myBooleans".into()
                                },
                                query::schema::Node::Field {
                                    name: "myDateTimes".into()
                                },
                                query::schema::Node::Field {
                                    name: "myFloats".into()
                                },
                                query::schema::Node::Field {
                                    name: "myInts".into()
                                },
                                query::schema::Node::Field {
                                    name: "myStrings".into()
                                },
                                query::schema::Node::Field {
                                    name: "myZeta".into()
                                },
                                query::schema::Node::Field {
                                    name: "myZetas".into()
                                },
                                query::schema::Node::Relation {
                                    name: "myBeta".into(),
                                    nodes: vec![
                                        query::schema::Node::Field {
                                            name: "foo".into()
                                        },
                                        query::schema::Node::Relation {
                                            name: "myGamma".into(),
                                            nodes: vec![
                                                query::schema::Node::Field {
                                                    name: "foo".into()
                                                }
                                            ]
                                        },
                                    ],
                                },
                                query::schema::Node::Relation {
                                    name: "myGammas".into(),
                                    nodes: vec![query::schema::Node::Field {
                                        name: "foo".into()
                                    }],
                                },
                                query::schema::Node::Relation {
                                    name: "myDelta".into(),
                                    nodes: vec![query::schema::Node::Field {
                                        name: "foo".into()
                                    }],
                                },
                                query::schema::Node::Relation {
                                    name: "myEpsilons".into(),
                                    nodes: vec![query::schema::Node::Field {
                                        name: "foo".into()
                                    }],
                                },
                            ]
                        },
                        r#where: Some(query::Where {
                            alias: "alpha".into(),
                            conditions: vec![query::Condition {
                                lhs: once("myBoolean".into()).collect(),
                                operator: query::Operator::Equals,
                                rhs: "booleanArgument".into(),
                            }]
                        }),
                    }
                )]),
                models: OrdStrMap::from_iter([
                    ("Alpha", {
                        let mut model = Model::new("Alpha");

                        model.insert_field(model::Field {
                            name: "myBoolean".into(),
                            r#type: Type::Boolean,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_field(model::Field {
                            name: "myDateTime".into(),
                            r#type: Type::DateTime,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_field(model::Field {
                            name: "myFloat".into(),
                            r#type: Type::Float,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_field(model::Field {
                            name: "myInt".into(),
                            r#type: Type::Int,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_field(model::Field {
                            name: "myString".into(),
                            r#type: Type::String,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_field(model::Field {
                            name: "myBooleans".into(),
                            r#type: Type::Boolean,
                            cardinality: Cardinality::Many,
                        })?;

                        model.insert_field(model::Field {
                            name: "myDateTimes".into(),
                            r#type: Type::DateTime,
                            cardinality: Cardinality::Many,
                        })?;

                        model.insert_field(model::Field {
                            name: "myFloats".into(),
                            r#type: Type::Float,
                            cardinality: Cardinality::Many,
                        })?;

                        model.insert_field(model::Field {
                            name: "myInts".into(),
                            r#type: Type::Int,
                            cardinality: Cardinality::Many,
                        })?;

                        model.insert_field(model::Field {
                            name: "myStrings".into(),
                            r#type: Type::String,
                            cardinality: Cardinality::Many,
                        })?;

                        model.insert_enum_relation("myZeta", "Zeta")?;
                        model.insert_enums_relation("myZetas", "Zeta")?;
                        model.insert_one_to_one("myBeta", "Beta")?;
                        model.insert_one_to_many("myGammas", "Gamma")?;
                        model.insert_many_to_one("myDelta", "Delta")?;
                        model.insert_many_to_many("myEpsilons", "Epsilon")?;

                        model
                    }),
                    ("Beta", {
                        let mut model = Model::new("Beta");

                        model.insert_field(model::Field {
                            name: "foo".into(),
                            r#type: Type::String,
                            cardinality: Cardinality::One,
                        })?;

                        model.insert_many_to_one("myGamma", "Gamma")?;

                        model
                    }),
                    ("Gamma", {
                        let mut model = Model::new("Gamma");

                        model.insert_field(model::Field {
                            name: "foo".into(),
                            r#type: Type::String,
                            cardinality: Cardinality::One,
                        })?;

                        model
                    }),
                    ("Delta", {
                        let mut model = Model::new("Delta");

                        model.insert_field(model::Field {
                            name: "foo".into(),
                            r#type: Type::String,
                            cardinality: Cardinality::One,
                        })?;

                        model
                    }),
                    ("Epsilon", {
                        let mut model = Model::new("Epsilon");

                        model.insert_field(model::Field {
                            name: "foo".into(),
                            r#type: Type::String,
                            cardinality: Cardinality::One,
                        })?;

                        model
                    }),
                ]),
                enums: OrdStrMap::from_iter([(
                    "Zeta",
                    Enum {
                        name: "Zeta".into(),
                        values: TokenSet::from_iter(["Theta", "Iota"])
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
            schema: query::Schema {
                alias: "user".into(),
                nodes: Vec::new(),
            },
            r#where: None,
            r#type: query::ReturnType {
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

    #[test]
    fn test_enum_type_none() {
        let mut ir = Ir::default();

        let _: Option<Model<'static>> =
            ir.models.insert("Foo", Model::new("Foo"));

        assert_eq!(
            ir.enum_type("Foo", VecDeque::from_iter(["bar".into()])),
            None
        );
    }

    #[test]
    fn test_enum_type_empty() {
        let mut ir = Ir::default();

        let _: Option<Model<'static>> =
            ir.models.insert("Foo", Model::new("Foo"));

        assert_eq!(ir.enum_type("Foo", VecDeque::from_iter([])), None);
    }
}
