pub use self::{
    cardinality::Cardinality,
    component::Component,
    model::{
        EnumRelation,
        Field,
        Model,
        Relation as ModelRelation,
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
    route::Route,
};
use {
    crate::ast::{
        self,
        TypeError,
    },
    std::collections::{
        BTreeMap,
        BTreeSet,
        VecDeque,
    },
};

/// Cardinality.
pub mod cardinality;
/// Components.
pub mod component;
/// Enums.
pub mod r#enum;
/// Models.
pub mod model;
/// Queries.
pub mod query;
/// Routes.
pub mod route;
/// Types.
pub mod r#type;

/// The intermediate representation (IR) of the AST.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ir {
    /// The models in the data model.
    pub models: BTreeMap<String, Model>,
    /// The enums in the data model.
    pub enums: BTreeMap<String, Enum>,
    /// Components.
    pub components: BTreeMap<String, Component>,
    /// Routes.
    pub routes: BTreeMap<String, Route>,
    /// Queries.
    pub queries: BTreeMap<String, Query>,
}

impl Ir {
    /// Create an empty IR.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            models: BTreeMap::new(),
            enums: BTreeMap::new(),
            components: BTreeMap::new(),
            routes: BTreeMap::new(),
            queries: BTreeMap::new(),
        }
    }

    /// Resolve the type of a model field.
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the model.
    /// * `path` - The path to the field.
    #[must_use]
    pub fn field_type(
        &self,
        model: &str,
        path: &VecDeque<String>,
    ) -> Option<Type> {
        let mut path = path.iter().cloned().collect::<VecDeque<_>>();
        let mut current_model = self.models.get(model)?;

        while let Some(segment) = path.pop_front() {
            if path.is_empty() {
                return current_model
                    .fields
                    .get(&segment)
                    .map(|field| field.r#type);
            }

            if let Some(ModelRelation { name, .. }) =
                current_model.owned_models.get(&segment)
            {
                if let Some(model) = self.models.get(name) {
                    current_model = model;
                    continue;
                }
            }

            if let Some(ModelRelation { name, .. }) =
                current_model.models.get(&segment)
            {
                if let Some(model) = self.models.get(name) {
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
    pub fn enum_type(
        &self,
        model: &str,
        path: &VecDeque<String>,
    ) -> Option<String> {
        let mut path = path.iter().cloned().collect::<VecDeque<_>>();
        let mut current_model = self.models.get(model)?;

        while let Some(segment) = path.pop_front() {
            if path.is_empty() {
                return current_model
                    .enums
                    .get(&segment)
                    .map(|r| r.name.clone());
            }

            if let Some(ModelRelation { name, .. }) =
                current_model.owned_models.get(&segment)
            {
                if let Some(model) = self.models.get(name) {
                    current_model = model;
                    continue;
                }
            }

            if let Some(ModelRelation { name, .. }) =
                current_model.models.get(&segment)
            {
                if let Some(model) = self.models.get(name) {
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
        model: &Model,
    ) -> Result<(), TypeError> {
        if self
            .models
            .insert(model.name.clone(), model.clone())
            .is_some()
        {
            return Err(TypeError::duplicate_model(&model.name));
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
        r#enum: &Enum,
    ) -> Result<(), TypeError> {
        if self
            .enums
            .insert(r#enum.name.clone(), r#enum.clone())
            .is_some()
        {
            return Err(TypeError::duplicate_enum(&r#enum.name));
        }

        Ok(())
    }

    /// Insert a component.
    ///
    /// # Arguments
    ///
    /// * `component` - The component to insert.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if a component with the same name already exists.
    pub fn insert_component(
        &mut self,
        component: &Component,
    ) -> Result<(), TypeError> {
        if self
            .components
            .insert(component.name.clone(), component.clone())
            .is_some()
        {
            return Err(TypeError::duplicate_component(&component.name));
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
    pub fn check_argument_type(
        &self,
        model_name: &str,
        path: &VecDeque<String>,
        argument_type: &QueryArgumentType,
    ) -> bool {
        match argument_type {
            QueryArgumentType::Enum(rhs) => {
                self.enum_type(model_name, path)
                    .map_or(false, |lhs| &lhs == rhs)
            }
            QueryArgumentType::Type(rhs) => {
                self.field_type(model_name, path)
                    .map_or(false, |lhs| &lhs == rhs)
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
    pub fn query_schema(
        &self,
        query_name: &str,
        ast_schema: &ast::QuerySchema,
        model: &Model,
    ) -> Result<QuerySchema, TypeError> {
        let mut nodes = Vec::new();

        for node in &ast_schema.nodes {
            nodes.push(self.query_schema_node(
                query_name,
                node,
                model,
                &[].into(),
            )?);
        }

        Ok(QuerySchema::new(&ast_schema.name, &nodes))
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
    pub fn query_schema_node(
        &self,
        query_name: &str,
        ast_node: &ast::QuerySchemaNode,
        model: &Model,
        path: &VecDeque<String>,
    ) -> Result<QuerySchemaNode, TypeError> {
        match ast_node {
            ast::QuerySchemaNode::Field(name) => {
                let mut path = path.clone();

                path.push_back(name.clone());

                if self.field_type(model.name.as_str(), &path).is_some() {
                    Ok(QuerySchemaNode::field(name))
                } else {
                    Err(TypeError::undefined_query_field(query_name, name))
                }
            }
            ast::QuerySchemaNode::Relation(ast_name, ast_schema) => {
                let mut nodes = Vec::new();

                for ast_node in ast_schema {
                    let mut path = path.clone();

                    path.push_back(ast_name.clone());

                    if let Some(model) = self.models.get(&model.name) {
                        nodes.push(self.query_schema_node(
                            query_name, ast_node, model, &path,
                        )?);
                    } else {
                        return Err(TypeError::undefined_query_field(
                            query_name, ast_name,
                        ));
                    }
                }

                Ok(QuerySchemaNode::relation(ast_name, &nodes))
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
    pub fn query_return_type(
        &self,
        query_name: &str,
        ast_return_type: &ast::QueryReturnType,
    ) -> Result<(QueryReturnType, Model), TypeError> {
        let return_type = QueryReturnType::from(ast_return_type);

        self.models.get(&return_type.model_name).map_or_else(
            || {
                Err(TypeError::undefined_query_return_type(
                    query_name,
                    ast_return_type,
                ))
            },
            |model| Ok((return_type, model.clone())),
        )
    }

    /// Create an IR from an AST.
    ///
    /// # Arguments
    ///
    /// * `value` - The AST to convert.
    ///
    /// # Errors
    ///
    /// _
    #[allow(clippy::too_many_lines)]
    pub fn from(value: &ast::Ast) -> Result<Self, TypeError> {
        let ast::Ast {
            models: ast_models,
            enums: ast_enums,
            components: ast_components,
            routes: ast_routes,
            queries: ast_queries,
        } = value;

        let enum_names = ast_enums
            .iter()
            .map(|e| e.name.clone())
            .collect::<BTreeSet<_>>();

        let model_names = ast_models
            .iter()
            .map(|m| m.name.clone())
            .collect::<BTreeSet<_>>();

        let mut ir = Self::new();

        for model in ast_models {
            let ast::Model { fields, .. } = model;
            let mut model = Model::new(&model.name);
            let mut field_names = BTreeSet::new();

            for field in fields {
                if !field_names.insert(field.name.clone()) {
                    return Err(TypeError::duplicate_model_field(
                        &model.name,
                        &field.name,
                    ));
                }

                let ast::Field { r#type, .. } = field.clone();

                match r#type {
                    ast::Type::Scalar(ast::Scalar::Boolean) => {
                        model.insert_field(Field::boolean(&field.name));
                    }
                    ast::Type::Scalar(ast::Scalar::DateTime) => {
                        model.insert_field(Field::date_time(&field.name));
                    }
                    ast::Type::Scalar(ast::Scalar::Float) => {
                        model.insert_field(Field::float(&field.name));
                    }
                    ast::Type::Scalar(ast::Scalar::Int) => {
                        model.insert_field(Field::int(&field.name));
                    }
                    ast::Type::Scalar(ast::Scalar::String) => {
                        model.insert_field(Field::string(&field.name));
                    }
                    ast::Type::Scalar(ast::Scalar::Reference(name)) => {
                        if enum_names.contains(&name) {
                            model.insert_enum(&field.name, &name);
                        } else if model_names.contains(&name) {
                            model.insert_model(&field.name, &name);
                        } else {
                            return Err(TypeError::unknown_model_field_type(
                                &model.name,
                                field,
                            ));
                        }
                    }
                    ast::Type::Scalar(ast::Scalar::Owned(name)) => {
                        if model_names.contains(&name) {
                            model.insert_owned_model(&field.name, &name);
                        } else {
                            return Err(TypeError::unknown_model_field_type(
                                &model.name,
                                field,
                            ));
                        }
                    }
                    ast::Type::Array(ast::Scalar::Boolean) => {
                        model.insert_field(Field::booleans(&field.name));
                    }
                    ast::Type::Array(ast::Scalar::DateTime) => {
                        model.insert_field(Field::date_times(&field.name));
                    }
                    ast::Type::Array(ast::Scalar::Float) => {
                        model.insert_field(Field::floats(&field.name));
                    }
                    ast::Type::Array(ast::Scalar::Int) => {
                        model.insert_field(Field::ints(&field.name));
                    }
                    ast::Type::Array(ast::Scalar::String) => {
                        model.insert_field(Field::strings(&field.name));
                    }
                    ast::Type::Array(ast::Scalar::Reference(name)) => {
                        if enum_names.contains(&name) {
                            model.insert_enums(&field.name, &name);
                        } else if model_names.contains(&name) {
                            model.insert_models(&field.name, &name);
                        } else {
                            return Err(TypeError::unknown_model_field_type(
                                &model.name,
                                field,
                            ));
                        }
                    }
                    ast::Type::Array(ast::Scalar::Owned(name)) => {
                        if model_names.contains(&name) {
                            model.insert_owned_models(&field.name, &name);
                        } else {
                            return Err(TypeError::unknown_model_field_type(
                                &model.name,
                                field,
                            ));
                        }
                    }
                }
            }

            let _ = ir.insert_model(&model);
        }

        for ast_enum in ast_enums {
            let _ = ir.insert_enum(&ast_enum.clone().into());
        }

        for ast_component in ast_components {
            let _ = ir.insert_component(&ast_component.clone().into());
        }

        for ast::Route { path, root, title } in ast_routes {
            if ir.components.contains_key(root) {
                let route = Route {
                    path: path.clone(),
                    root: root.clone(),
                    title: title.clone(),
                };

                if ir.routes.insert(path.clone(), route).is_some() {
                    return Err(TypeError::duplicate_route(path));
                }
            } else {
                return Err(TypeError::undefined_route_component(path, root));
            }
        }

        for ast_query in ast_queries {
            let ast::Query {
                arguments: ast_arguments,
                schema: ast_schema,
                r#type: ast_type,
                r#where: ast_where,
                name: ast_name,
            } = ast_query;

            if let Some(ast_where) = ast_where {
                if ast_schema.name != ast_where.name {
                    return Err(TypeError::invalid_query_where(
                        ast_name,
                        &ast_schema.name,
                        &ast_where.name,
                    ));
                }
            }

            let (return_type, model) =
                ir.query_return_type(ast_name, ast_type)?;

            let mut query =
                Query::new(ast_name, return_type.clone(), &ast_schema.name);

            // TODO: check uniqueness of argument names
            for ast_argument in ast_arguments {
                if let Some(argument) =
                    QueryArgument::from_ast_type(ast_argument, &enum_names)
                {
                    query.arguments.push(argument);
                };
            }

            query.schema = ir.query_schema(ast_name, ast_schema, &model)?;

            if let Some(ast::query::Where {
                conditions: ast_conditions,
                name: alias,
            }) = &ast_where
            {
                let mut conditions = Vec::new();

                for ast_condition @ ast::query::r#where::Condition {
                    path: ast::query::r#where::Path(path),
                    operator,
                    argument_name,
                } in ast_conditions
                {
                    let model_name = query.r#type.model_name.clone();

                    if !query.arguments.iter().any(|argument| {
                        argument.name == *argument_name
                            && ir.check_argument_type(
                                &model_name,
                                path,
                                &argument.r#type,
                            )
                    }) {
                        return Err(TypeError::invalid_query_condition(
                            ast_name,
                            ast_condition,
                        ));
                    }

                    conditions.push(QueryCondition {
                        lhs: path.clone(),
                        operator: (*operator).into(),
                        rhs: argument_name.clone(),
                    });
                }

                query.where_clause = Some(QueryWhere::new(alias, &conditions));
            }

            if ir.queries.insert(ast_name.clone(), query).is_some() {
                return Err(TypeError::duplicate_query(ast_name));
            }
        }

        Ok(ir)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::iter::once,
    };

    #[test]
    fn test_new() {
        let ir = Ir::new();

        assert!(ir.models.is_empty());
        assert!(ir.enums.is_empty());
        assert!(ir.components.is_empty());
        assert!(ir.routes.is_empty());
        assert!(ir.queries.is_empty());
    }

    #[test]
    fn test_resolve_model_field() {
        let mut ir = Ir::new();
        let mut user_model = Model::new("User");
        let mut address_model = Model::new("Address");
        let mut postbox_model = Model::new("Postbox");

        let _ = user_model
            .fields
            .insert("name".to_owned(), Field::string("name"));

        let _ = address_model
            .fields
            .insert("street".to_owned(), Field::string("street"));

        let _ = postbox_model
            .fields
            .insert("number".to_owned(), Field::int("number"));

        let _ = address_model
            .owned_models
            .insert("postbox".to_owned(), ModelRelation::one("Postbox"));

        let _ = user_model
            .models
            .insert("address".to_owned(), ModelRelation::one("Address"));

        let _ = ir.models.insert("User".to_owned(), user_model);
        let _ = ir.models.insert("Address".to_owned(), address_model);
        let _ = ir.models.insert("Postbox".to_owned(), postbox_model);

        assert_eq!(
            ir.field_type(
                "User",
                &once("name").map(ToString::to_string).collect()
            ),
            Some(Type::String),
        );

        assert_eq!(
            ir.field_type(
                "User",
                &["address", "street"]
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            ),
            Some(Type::String),
        );

        assert_eq!(
            ir.field_type(
                "User",
                &["address", "postbox", "number"]
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            ),
            Some(Type::Int),
        );

        assert_eq!(
            ir.field_type(
                "User",
                &["address", "postbox", "street",]
                    .iter()
                    .map(ToString::to_string)
                    .collect(),
            ),
            None,
        );
    }

    #[test]
    fn test_resolve_model_enum() {
        let mut ir = Ir::new();
        let mut user_model = Model::new("User");
        let mut address_model = Model::new("Address");
        let address_type = Enum::new("AddressType", &["Home", "Work", "Other"]);

        let _ = user_model
            .fields
            .insert("name".to_owned(), Field::string("name"));

        let _ = user_model
            .models
            .insert("address".to_owned(), ModelRelation::one("Address"));

        let _ = address_model
            .enums
            .insert("type".to_owned(), EnumRelation::one("AddressType"));

        let _ = ir.models.insert("User".to_owned(), user_model);
        let _ = ir.models.insert("Address".to_owned(), address_model);
        let _ = ir.enums.insert("AddressType".to_owned(), address_type);

        assert_eq!(
            ir.enum_type(
                "User",
                &once("name").map(ToString::to_string).collect()
            ),
            None,
        );

        assert_eq!(
            ir.enum_type(
                "User",
                &["address", "type"]
                    .iter()
                    .map(ToString::to_string)
                    .collect()
            ),
            Some("AddressType".to_owned()),
        );
    }

    #[allow(clippy::too_many_lines)]
    #[test]
    fn test_from_ast_full() {
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

route /users {
  root: UserList
  title: Users
}

component UserList {
  path: /UserList
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
        let ir = Ir::from(&ast);

        assert_eq!(
            ir,
            Ok(Ir {
                queries: [(
                    "users".to_owned(),
                    Query {
                        name: "users".to_owned(),
                        arguments: vec![QueryArgument::r#enum(
                            "addressType",
                            "AddressType",
                        )],
                        r#type: QueryReturnType::many("User"),
                        schema: QuerySchema::new(
                            "user",
                            &[
                                QuerySchemaNode::field("name"),
                                QuerySchemaNode::field("age"),
                                QuerySchemaNode::field("daBoi"),
                                QuerySchemaNode::relation(
                                    "addresses",
                                    &[
                                        QuerySchemaNode::field("street"),
                                        QuerySchemaNode::field("number"),
                                    ]
                                ),
                            ]
                        ),
                        where_clause: Some(QueryWhere::new(
                            "user",
                            &[QueryCondition {
                                lhs: ["addresses", "type"]
                                    .iter()
                                    .map(ToString::to_string)
                                    .collect(),
                                operator: QueryOperator::Equals,
                                rhs: "addressType".to_owned(),
                            }]
                        )),
                    }
                )]
                .into(),
                routes: [(
                    "/users".to_owned(),
                    Route::new("/users", "UserList", "Users")
                )]
                .into(),
                components: [(
                    "UserList".to_owned(),
                    Component::new("UserList", "UserList")
                )]
                .into(),
                models: [
                    (
                        "User".to_owned(),
                        Model {
                            name: "User".to_owned(),
                            fields: [
                                ("name".to_owned(), Field::string("name")),
                                ("age".to_owned(), Field::int("age")),
                                ("daBoi".to_owned(), Field::boolean("daBoi")),
                            ]
                            .into(),
                            models: [(
                                "addresses".to_owned(),
                                ModelRelation::many("Address"),
                            )]
                            .into(),
                            enums: BTreeMap::new(),
                            owned_models: [(
                                "profile".to_owned(),
                                ModelRelation::one("Profile"),
                            )]
                            .into()
                        }
                    ),
                    (
                        "Profile".to_owned(),
                        Model {
                            name: "Profile".to_owned(),
                            fields: [("bio".to_owned(), Field::string("bio"))]
                                .into(),
                            models: BTreeMap::new(),
                            enums: BTreeMap::new(),
                            owned_models: BTreeMap::new(),
                        },
                    ),
                    (
                        "Address".to_owned(),
                        Model {
                            name: "Address".to_owned(),
                            fields: [
                                ("street".to_owned(), Field::string("street")),
                                ("number".to_owned(), Field::int("number")),
                            ]
                            .into(),
                            models: BTreeMap::new(),
                            enums: [(
                                "type".to_owned(),
                                EnumRelation::one("AddressType"),
                            )]
                            .into(),
                            owned_models: BTreeMap::new(),
                        },
                    )
                ]
                .into(),
                enums: [(
                    "AddressType".to_owned(),
                    Enum::new("AddressType", &["Home", "Work", "Other"]),
                )]
                .into(),
            })
        );
    }
}
