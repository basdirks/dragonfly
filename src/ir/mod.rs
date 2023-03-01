use {
    crate::ast::{
        self,
        Ast,
        TypeError,
    },
    std::{
        collections::{
            HashMap,
            HashSet,
            VecDeque,
        },
        path::PathBuf,
    },
};

/// Cardinality.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Cardinality {
    /// One.
    One,
    /// Many.
    Many,
}

/// A scalar type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    /// A boolean.
    Boolean,
    /// A date time.
    DateTime,
    /// A floating point number.
    Float,
    /// An integer.
    Int,
    /// A string.
    String,
}

/// A data field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub r#type: Type,
    /// The cardinality of the field.
    pub cardinality: Cardinality,
}

impl Field {
    /// Create a field with a single boolean.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::boolean("is_admin"),
    ///     Field {
    ///         name: "is_admin".to_owned(),
    ///         r#type: Type::Boolean,
    ///         cardinality: Cardinality::One,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn boolean(name: String) -> Self {
        Self {
            name,
            r#type: Type::Boolean,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with a single date time.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::datetime("created_at"),
    ///     Field {
    ///         name: "created_at".to_owned(),
    ///         r#type: Type::DateTime,
    ///         cardinality: Cardinality::One,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn datetime(name: String) -> Self {
        Self {
            name,
            r#type: Type::DateTime,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with a single floating point number.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::float("price"),
    ///     Field {
    ///         name: "price".to_owned(),
    ///         r#type: Type::Float,
    ///         cardinality: Cardinality::One,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn float(name: String) -> Self {
        Self {
            name,
            r#type: Type::Float,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with a single integer.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::int("age"),
    ///     Field {
    ///         name: "age".to_owned(),
    ///         r#type: Type::Int,
    ///         cardinality: Cardinality::One,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn int(name: String) -> Self {
        Self {
            name,
            r#type: Type::Int,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with a single string.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::string("name"),
    ///     Field {
    ///         name: "name".to_owned(),
    ///         r#type: Type::String,
    ///         cardinality: Cardinality::One,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn string(name: String) -> Self {
        Self {
            name,
            r#type: Type::String,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with an array of booleans.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::booleans("bits"),
    ///     Field {
    ///         name: "bits".to_owned(),
    ///         r#type: Type::Boolean,
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn booleans(name: String) -> Self {
        Self {
            name,
            r#type: Type::Boolean,
            cardinality: Cardinality::Many,
        }
    }

    /// Create a field with an array of date times.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::datetimes("events"),
    ///     Field {
    ///         name: "events".to_owned(),
    ///         r#type: Type::DateTime,
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn datetimes(name: String) -> Self {
        Self {
            name,
            r#type: Type::DateTime,
            cardinality: Cardinality::Many,
        }
    }

    /// Create a field with an array of floating point numbers.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::floats("intervals"),
    ///     Field {
    ///         name: "intervals".to_owned(),
    ///         r#type: Type::Float,
    ///         cardinality: Cardinality::Many,
    ///     },
    /// );
    /// ```
    #[must_use]
    pub const fn floats(name: String) -> Self {
        Self {
            name,
            r#type: Type::Float,
            cardinality: Cardinality::Many,
        }
    }

    /// Create a field with an array of integers.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::ints("ages"),
    ///     Field {
    ///         name: "ages".to_owned(),
    ///         r#type: Type::Int,
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn ints(name: String) -> Self {
        Self {
            name,
            r#type: Type::Int,
            cardinality: Cardinality::Many,
        }
    }

    /// Create a field with an array of strings.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Field;
    ///
    /// assert_eq!(
    ///     Field::strings("names"),
    ///     Field {
    ///         name: "names".to_owned(),
    ///         r#type: Type::String,
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn strings(name: String) -> Self {
        Self {
            name,
            r#type: Type::String,
            cardinality: Cardinality::Many,
        }
    }
}

/// A model relation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ModelRelation {
    /// The name of the model.
    pub name: String,
    /// The cardinality of the relation.
    pub cardinality: Cardinality,
}

impl ModelRelation {
    /// Create a model to-many relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::{
    ///     Cardinality,
    ///     ModelRelation,
    /// };
    ///
    /// assert_eq!(
    ///     ModelRelation::many("users"),
    ///     ModelRelation {
    ///         name: "users".to_owned(),
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn many(name: String) -> Self {
        Self {
            name,
            cardinality: Cardinality::Many,
        }
    }

    /// Create a model to-one relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::{
    ///     Cardinality,
    ///     ModelRelation,
    /// };
    ///
    /// assert_eq!(
    ///     ModelRelation::one("user"),
    ///     ModelRelation {
    ///         name: "user".to_owned(),
    ///         cardinality: Cardinality::One,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn one(name: String) -> Self {
        Self {
            name,
            cardinality: Cardinality::One,
        }
    }
}

/// An enum relation.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct EnumRelation {
    /// The name of the enum.
    pub name: String,
    /// The cardinality of the relation.
    pub cardinality: Cardinality,
}

impl EnumRelation {
    /// Create an enum to-many relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::{
    ///     Cardinality,
    ///     EnumRelation,
    /// };
    ///
    /// assert_eq!(
    ///     EnumRelation::many("roles"),
    ///     EnumRelation {
    ///         name: "roles".to_owned(),
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn many(name: String) -> Self {
        Self {
            name,
            cardinality: Cardinality::Many,
        }
    }

    /// Create an enum to-one relation.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::{
    ///     Cardinality,
    ///     EnumRelation,
    /// };
    ///
    /// assert_eq!(
    ///     EnumRelation::one("role"),
    ///     EnumRelation {
    ///         name: "role".to_owned(),
    ///         cardinality: Cardinality::One,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn one(name: String) -> Self {
        Self {
            name,
            cardinality: Cardinality::One,
        }
    }
}

/// A model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    /// The name of the model.
    pub name: String,
    /// The data fields of the model.
    pub fields: HashMap<String, Field>,
    /// Relations to models that this model owns.
    pub owned_models: HashMap<String, ModelRelation>,
    /// Relations to models that this model references.
    pub models: HashMap<String, ModelRelation>,
    /// Relations to enum values.
    pub enums: HashMap<String, EnumRelation>,
}

impl Model {
    /// Create an empty model.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Model;
    ///
    /// let model = Model::new("User");
    ///
    /// assert_eq!(model.name, "User");
    /// assert!(model.fields.is_empty());
    /// assert!(model.owned_models.is_empty());
    /// assert!(model.models.is_empty());
    /// assert!(model.enums.is_empty());
    /// ```
    #[must_use]
    pub fn new(name: String) -> Self {
        Self {
            name,
            fields: HashMap::new(),
            owned_models: HashMap::new(),
            models: HashMap::new(),
            enums: HashMap::new(),
        }
    }
}

/// An enum type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Enum {
    /// The name of the enum.
    pub name: String,
    /// The values of the enum.
    pub values: Vec<String>,
}

impl Enum {
    /// Create a new enum.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the enum.
    /// * `values` - The values of the enum.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::Enum;
    ///
    /// let ir_enum = Enum::new("Role", vec!["Admin", "User"]);
    ///
    /// assert_eq!(
    ///     ir_enum,
    ///     Enum {
    ///         name: "Role".to_owned(),
    ///         values: vec!["Admin".to_owned(), "User".to_owned()],
    ///     }
    /// );
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        values: &[&str],
    ) -> Self {
        Self {
            name: name.to_owned(),
            values: values.iter().map(ToString::to_string).collect(),
        }
    }
}

impl From<ast::Enum> for Enum {
    fn from(ast_enum: ast::Enum) -> Self {
        Self {
            name: ast_enum.name,
            values: ast_enum.variants,
        }
    }
}

/// A component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Component {
    /// The name of the component.
    pub name: String,
    /// The path to the component source file.
    pub path: PathBuf,
}

impl From<ast::Component> for Component {
    fn from(value: ast::Component) -> Self {
        Self {
            name: value.name,
            path: value.path,
        }
    }
}

/// A route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Route {
    /// The path of the route.
    pub path: String,
    /// The root component of the route.
    pub root: String,
    /// The title of the page at the route.
    pub title: String,
}

impl From<ast::Route> for Route {
    fn from(value: ast::Route) -> Self {
        Self {
            root: value.root,
            path: value.path,
            title: value.title,
        }
    }
}

/// The return type of a query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryReturnType {
    /// The name of the model.
    pub model_name: String,
    /// The cardinality of the return type.
    pub cardinality: Cardinality,
}

/// The type of an argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QueryArgumentType {
    /// A reference to an enum.
    Enum(String),
    /// A scalar type.
    Type(Type),
}

/// An argument to a query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryArgument {
    /// The name of the argument.
    pub name: String,
    /// The type of the argument.
    pub r#type: QueryArgumentType,
    /// The cardinality of the argument.
    pub cardinality: Cardinality,
}

impl QueryArgument {
    /// Create an argument with a single boolean.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::boolean("is_admin"),
    ///     QueryArgument {
    ///         name: "is_admin".to_owned(),
    ///         r#type: ArgumentType::Type(Type::Boolean),
    ///         cardinality: Cardinality::Single,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn boolean(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::Boolean),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with a single date time.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::datetime("created_at"),
    ///     QueryArgument {
    ///         name: "created_at".to_owned(),
    ///         r#type: ArgumentType::Type(Type::DateTime),
    ///         cardinality: Cardinality::Single,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn datetime(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::DateTime),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with a single float.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::float("price"),
    ///     QueryArgument {
    ///         name: "price".to_owned(),
    ///         r#type: ArgumentType::Type(Type::Float),
    ///         cardinality: Cardinality::Single,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn float(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::Float),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with a single integer.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::int("age"),
    ///     QueryArgument {
    ///         name: "age".to_owned(),
    ///         r#type: ArgumentType::Type(Type::Integer),
    ///         cardinality: Cardinality::Single,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn int(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::Int),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with a single string.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::string("name"),
    ///     QueryArgument {
    ///         name: "name".to_owned(),
    ///         r#type: ArgumentType::Type(Type::String),
    ///         cardinality: Cardinality::Single,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn string(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::String),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with an array of booleans.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::booleans("is_admin"),
    ///     QueryArgument {
    ///         name: "is_admin".to_owned(),
    ///         r#type: ArgumentType::Type(Type::Boolean),
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn booleans(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::Boolean),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument with an array of date times.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::datetimes("created_at"),
    ///     QueryArgument {
    ///         name: "created_at".to_owned(),
    ///         r#type: ArgumentType::Type(Type::DateTime),
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn datetimes(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::DateTime),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument with an array of floats.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::floats("price"),
    ///     QueryArgument {
    ///         name: "price".to_owned(),
    ///         r#type: ArgumentType::Type(Type::Float),
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn floats(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::Float),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument with an array of integers.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::ints("age"),
    ///     QueryArgument {
    ///         name: "age".to_owned(),
    ///         r#type: ArgumentType::Type(Type::Integer),
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn ints(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::Int),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument with an array of strings.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::QueryArgument;
    ///
    /// assert_eq!(
    ///     QueryArgument::strings("name"),
    ///     QueryArgument {
    ///         name: "name".to_owned(),
    ///         r#type: ArgumentType::Type(Type::String),
    ///         cardinality: Cardinality::Many,
    ///     }
    /// );
    /// ```
    #[must_use]
    pub const fn strings(name: String) -> Self {
        Self {
            name,
            r#type: QueryArgumentType::Type(Type::String),
            cardinality: Cardinality::Many,
        }
    }
}

/// A query condition operator.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum QueryConditionOperator {
    /// Equals.
    Equals,
    /// Contains.
    Contains,
}

impl From<ast::query::r#where::Operator> for QueryConditionOperator {
    fn from(value: ast::query::r#where::Operator) -> Self {
        match value {
            ast::query::r#where::Operator::Equals => Self::Equals,
            ast::query::r#where::Operator::Contains => Self::Contains,
        }
    }
}

/// A query condition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryCondition {
    /// The lhs operand (the path to the field).
    pub lhs: VecDeque<String>,
    /// The condition operator.
    pub operator: QueryConditionOperator,
    /// The rhs operand (the argument name).
    pub rhs: String,
}

/// A query schema node.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuerySchemaNode {
    /// A field.
    Field(String),
    /// A relation.
    Relation(String, Vec<QuerySchemaNode>),
}

/// A query where clause.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryWhereClause {
    /// The alias of the where clause.
    pub alias: String,
    /// The conditions of the where clause.
    pub conditions: Vec<QueryCondition>,
}

/// A query schema.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QuerySchema {
    /// The alias of the schema.
    pub alias: String,
    /// The nodes of the schema.
    pub nodes: Vec<QuerySchemaNode>,
}

/// A query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query {
    /// The name of the query.
    pub name: String,
    /// The return type of the query.
    pub r#type: QueryReturnType,
    /// The arguments to the query.
    pub arguments: Vec<QueryArgument>,
    /// The schema of the return type.
    pub schema: QuerySchema,
    /// The where clause of the query.
    pub where_clause: Option<QueryWhereClause>,
}

impl Query {
    /// Create a new query.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the query.
    /// * `r#type` - The return type of the query.
    /// * `alias` - The alias of the root node of the schema and the where
    ///   clause.
    ///
    /// # Examples
    ///
    /// ```
    /// use dragonfly::ir::{
    ///     Query,
    ///     QueryReturnType,
    ///     QuerySchema,
    ///     QuerySchemaNode,
    ///     QueryWhereClause,
    /// };
    ///
    /// let query = Query::new(
    ///     "get_user",
    ///     QueryReturnType {
    ///         model: "User".to_owned(),
    ///         cardinality: Cardinality::One,
    ///     },
    ///     "user".to_owned(),
    /// );
    ///
    /// assert_eq!(query.name, "get_user");
    /// assert_eq!(query.r#type.model, "User");
    /// assert_eq!(query.r#type.cardinality, Cardinality::One);
    /// assert_eq!(query.args, vec![]);
    /// assert_eq!(query.schema.alias, "user");
    /// assert_eq!(query.schema.nodes, vec![]);
    /// assert_eq!(query.where_clause, None);
    /// ```
    #[must_use]
    pub const fn new(
        name: String,
        r#type: QueryReturnType,
        alias: String,
    ) -> Self {
        Self {
            name,
            r#type,
            arguments: vec![],
            schema: QuerySchema {
                alias,
                nodes: vec![],
            },
            where_clause: None,
        }
    }
}

/// The intermediate representation (IR) of the AST.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ir {
    /// The models in the data model.
    pub models: HashMap<String, Model>,
    /// The enums in the data model.
    pub enums: HashMap<String, Enum>,
    /// Components.
    pub components: HashMap<String, Component>,
    /// Routes.
    pub routes: HashMap<String, Route>,
    /// Queries.
    pub queries: HashMap<String, Query>,
}

impl Ir {
    /// Create an empty IR.
    ///
    /// # Examples
    ///
    /// ```
    /// use dragonfly::ir::Ir;
    ///
    /// let ir = Ir::new();
    ///
    /// assert!(ir.models.is_empty());
    /// assert!(ir.enums.is_empty());
    /// assert!(ir.components.is_empty());
    /// assert!(ir.routes.is_empty());
    /// assert!(ir.queries.is_empty());
    /// ```
    #[must_use]
    pub fn new() -> Self {
        Self {
            models: HashMap::new(),
            enums: HashMap::new(),
            components: HashMap::new(),
            routes: HashMap::new(),
            queries: HashMap::new(),
        }
    }

    /// Resolve the type of a model field.
    ///
    /// # Arguments
    ///
    /// * `model` - The name of the model.
    /// * `path` - The path to the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::{
    ///     Ir,
    ///     Model,
    ///     Type,
    /// };
    ///
    /// let mut ir = Ir::new();
    /// let mut userModel = Model::new("User".to_owned());
    /// let mut addressModel = Model::new("Address".to_owned());
    /// let mut postboxModel = Model::new("Postbox".to_owned());
    ///
    /// userModel
    ///     .fields
    ///     .insert("name".to_owned(), Field::string("name".to_owned()));
    ///
    /// addressModel
    ///     .fields
    ///     .insert("street".to_owned(), Field::string("street".to_owned()));
    ///
    /// postboxModel
    ///     .fields
    ///     .insert("number".to_owned(), Field::int("number".to_owned()));
    ///
    /// addressModel.owned_models.insert(
    ///     "postbox".to_owned(),
    ///     OwnedModel {
    ///         model: "Postbox".to_owned(),
    ///         cardinality: Cardinality::One,
    ///     },
    /// );
    ///
    /// userModel
    ///     .models
    ///     .insert("address".to_owned(), Model::new("Address".to_owned()));
    ///
    /// ir.models.insert("User".to_owned(), userModel);
    /// ir.models.insert("Address".to_owned(), addressModel);
    /// ir.models.insert("Postbox".to_owned(), postboxModel);
    ///
    /// assert_eq!(
    ///     ir.resolve_model_field("User", &["name".to_owned()]),
    ///     Some(Type::String),
    /// );
    ///
    /// assert_eq!(
    ///     ir.resolve_model_field(
    ///         "User",
    ///         &["address".to_owned(), "street".to_owned()]
    ///     ),
    ///     Some(Type::String),
    /// );
    ///
    /// assert_eq!(
    ///     ir.resolve_model_field(
    ///         "User",
    ///         &[
    ///             "address".to_owned(),
    ///             "postbox".to_owned(),
    ///             "number".to_owned()
    ///         ]
    ///     ),
    ///     Some(Type::Int),
    /// );
    ///
    /// assert_eq!(
    ///     ir.resolve_model_field(
    ///         "User",
    ///         &[
    ///             "address".to_owned(),
    ///             "postbox".to_owned(),
    ///             "street".to_owned()
    ///         ]
    ///     ),
    ///     None,
    /// );
    /// ```
    #[must_use]
    pub fn resolve_model_field(
        &self,
        model: &str,
        path: &mut VecDeque<String>,
    ) -> Option<Type> {
        let mut current_model = self.models.get(model)?;

        while let Some(segment) = path.pop_back() {
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ir::{
    ///     Ir,
    ///     Model,
    ///     Type,
    /// };
    ///
    /// let mut ir = Ir::new();
    /// let mut userModel = Model::new("User".to_owned());
    /// let mut addressModel = Model::new("Address".to_owned());
    ///
    /// let mut addressType = Enum {
    ///     name: "AddressType".to_owned(),
    ///     variants: vec![
    ///         "Home".to_owned(),
    ///         "Work".to_owned(),
    ///         "Other".to_owned(),
    ///     ],
    /// };
    ///
    /// userModel
    ///     .fields
    ///     .insert("name".to_owned(), Field::string("name".to_owned()));
    ///
    /// addressModel.enums.insert(
    ///     "type".to_owned(),
    ///     EnumRelation {
    ///         name: "AddressType".to_owned(),
    ///         cardinality: Cardinality::One,
    ///     },
    /// );
    ///
    /// userModel
    ///     .models
    ///     .insert("address".to_owned(), Model::new("Address".to_owned()));
    ///
    /// ir.models.insert("User".to_owned(), userModel);
    /// ir.models.insert("Address".to_owned(), addressModel);
    /// ir.enums.insert("AddressType".to_owned(), addressType);
    ///
    /// assert_eq!(ir.resolve_model_enum("User", &["name".to_owned()]), None,);
    ///
    /// assert_eq!(
    ///     ir.resolve_model_enum(
    ///         "User",
    ///         &["address".to_owned(), "type".to_owned()]
    ///     ),
    ///     Some("AddressType".to_owned()),
    /// );
    /// ```
    #[must_use]
    pub fn resolve_model_enum(
        &self,
        model: &str,
        path: &mut VecDeque<String>,
    ) -> Option<String> {
        let mut current_model = self.models.get(model)?;

        while let Some(segment) = path.pop_back() {
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
    pub fn from(value: Ast) -> Result<Self, TypeError> {
        let Ast {
            models: ast_models,
            enums: ast_enums,
            components: ast_components,
            routes: ast_routes,
            queries: ast_queries,
        } = value;

        let enum_names = ast_enums.keys().collect::<HashSet<_>>();
        let model_names = ast_models.keys().collect::<HashSet<_>>();
        let mut ir = Self::new();

        for (model_name, model) in &ast_models {
            let ast::Model { fields, .. } = model;
            let mut model = Model::new(model_name.clone());

            let mut insert_field = |field_name: &str, field: Field| {
                let _ = model.fields.insert(field_name.to_owned(), field);
            };

            let unknown_field_type = |field_name: &str, type_name: &str| {
                Err::<Self, TypeError>(TypeError::UnknownFieldType {
                    model_name: model_name.clone(),
                    field_name: field_name.to_owned(),
                    type_name: type_name.to_owned(),
                })
            };

            for (field_name, field) in fields {
                let ast::Field { r#type, .. } = field.clone();

                match r#type {
                    ast::Type::Scalar(ast::Scalar::Boolean) => {
                        insert_field(
                            field_name,
                            Field::boolean(field_name.clone()),
                        );
                    }
                    ast::Type::Scalar(ast::Scalar::DateTime) => {
                        insert_field(
                            field_name,
                            Field::datetime(field_name.clone()),
                        );
                    }
                    ast::Type::Scalar(ast::Scalar::Float) => {
                        insert_field(
                            field_name,
                            Field::float(field_name.clone()),
                        );
                    }
                    ast::Type::Scalar(ast::Scalar::Int) => {
                        insert_field(
                            field_name,
                            Field::int(field_name.clone()),
                        );
                    }
                    ast::Type::Scalar(ast::Scalar::String) => {
                        insert_field(
                            field_name,
                            Field::string(field_name.clone()),
                        );
                    }
                    ast::Type::Scalar(ast::Scalar::Reference(name)) => {
                        if enum_names.contains(&name) {
                            let _ = model.enums.insert(
                                field_name.clone(),
                                EnumRelation::one(name),
                            );
                        } else if model_names.contains(&name) {
                            let _ = model.models.insert(
                                field_name.clone(),
                                ModelRelation::one(name),
                            );
                        } else {
                            return unknown_field_type(field_name, &name);
                        }
                    }
                    ast::Type::Scalar(ast::Scalar::Owned(name)) => {
                        if model_names.contains(&field_name) {
                            let _ = model.owned_models.insert(
                                field_name.clone(),
                                ModelRelation::one(name),
                            );
                        } else {
                            return unknown_field_type(field_name, &name);
                        }
                    }
                    ast::Type::Array(ast::Scalar::Boolean) => {
                        insert_field(
                            field_name,
                            Field::booleans(field_name.clone()),
                        );
                    }
                    ast::Type::Array(ast::Scalar::DateTime) => {
                        insert_field(
                            field_name,
                            Field::datetimes(field_name.clone()),
                        );
                    }
                    ast::Type::Array(ast::Scalar::Float) => {
                        insert_field(
                            field_name,
                            Field::floats(field_name.clone()),
                        );
                    }
                    ast::Type::Array(ast::Scalar::Int) => {
                        insert_field(
                            field_name,
                            Field::ints(field_name.clone()),
                        );
                    }
                    ast::Type::Array(ast::Scalar::String) => {
                        insert_field(
                            field_name,
                            Field::strings(field_name.clone()),
                        );
                    }
                    ast::Type::Array(ast::Scalar::Reference(name)) => {
                        if enum_names.contains(&name) {
                            let _ = model.enums.insert(
                                field_name.clone(),
                                EnumRelation::many(name),
                            );
                        } else if model_names.contains(&name) {
                            let _ = model.models.insert(
                                field_name.clone(),
                                ModelRelation::many(name),
                            );
                        } else {
                            return unknown_field_type(field_name, &name);
                        }
                    }
                    ast::Type::Array(ast::Scalar::Owned(name)) => {
                        if model_names.contains(&field_name) {
                            let _ = model.owned_models.insert(
                                field_name.clone(),
                                ModelRelation::many(name),
                            );
                        } else {
                            return unknown_field_type(field_name, &name);
                        }
                    }
                }
            }

            let _ = ir.models.insert(model_name.clone(), model);
        }

        for (enum_name, r#enum) in &ast_enums {
            let _ = ir.enums.insert(enum_name.clone(), r#enum.clone().into());
        }

        for (component_name, component) in &ast_components {
            let _ = ir
                .components
                .insert(component_name.clone(), component.clone().into());
        }

        for (route_name, ast::Route { path, root, title }) in &ast_routes {
            if ir.components.contains_key(root) {
                let route = Route {
                    path: path.clone(),
                    root: root.clone(),
                    title: title.clone(),
                };

                let _ = ir.routes.insert(route_name.clone(), route);
            } else {
                return Err(TypeError::UnknownRouteRoot {
                    route_name: route_name.clone(),
                    root: root.clone(),
                });
            }
        }

        for (query_name, ast_query) in &ast_queries {
            let ast::Query {
                arguments: ast_arguments,
                schema: ast_schema,
                r#type: ast_type,
                r#where: ast_where,
                ..
            } = ast_query;

            if let Some(ast_where) = ast_where {
                if ast_schema.name != ast_where.name {
                    return Err(TypeError::QuerySchemaMismatch {
                        query_name: query_name.clone(),
                        schema_root: ast_schema.name.clone(),
                        where_root: ast_where.name.clone(),
                    });
                }
            }

            let (model_name, cardinality) = match ast_type {
                ast::query::ReturnType::Array(model_name) => {
                    (model_name, Cardinality::Many)
                }
                ast::query::ReturnType::Model(model_name) => {
                    (model_name, Cardinality::One)
                }
            };

            let r#type = if ir.models.contains_key(model_name) {
                QueryReturnType {
                    model_name: model_name.clone(),
                    cardinality,
                }
            } else {
                return Err(TypeError::UnknownQueryReturnType {
                    query_name: query_name.clone(),
                    model_name: model_name.clone(),
                });
            };

            let mut query =
                Query::new(query_name.clone(), r#type, ast_schema.name.clone());

            for ast_argument in ast_arguments {
                let ast::query::Argument {
                    r#type: ast_type,
                    name: argument_name,
                } = ast_argument;

                let invalid_query_argument = || {
                    Err(TypeError::InvalidQueryArgumentType {
                        query_name: query_name.clone(),
                        argument: ast_argument.clone(),
                    })
                };

                let argument = match ast_type {
                    ast::r#type::Type::Scalar(ast::r#type::Scalar::Boolean) => {
                        QueryArgument::boolean(argument_name.clone())
                    }
                    ast::r#type::Type::Scalar(
                        ast::r#type::Scalar::DateTime,
                    ) => QueryArgument::datetime(argument_name.clone()),
                    ast::r#type::Type::Scalar(ast::r#type::Scalar::Float) => {
                        QueryArgument::float(argument_name.clone())
                    }
                    ast::r#type::Type::Scalar(ast::r#type::Scalar::Int) => {
                        QueryArgument::int(argument_name.clone())
                    }
                    ast::r#type::Type::Scalar(ast::r#type::Scalar::String) => {
                        QueryArgument::string(argument_name.clone())
                    }
                    ast::r#type::Type::Scalar(
                        ast::r#type::Scalar::Reference(name),
                    ) => {
                        if enum_names.contains(&name) {
                            QueryArgument {
                                r#type: QueryArgumentType::Enum(name.clone()),
                                name: argument_name.clone(),
                                cardinality: Cardinality::One,
                            }
                        } else {
                            return invalid_query_argument();
                        }
                    }
                    ast::r#type::Type::Array(ast::r#type::Scalar::Boolean) => {
                        QueryArgument::booleans(argument_name.clone())
                    }
                    ast::r#type::Type::Array(ast::r#type::Scalar::DateTime) => {
                        QueryArgument::datetimes(argument_name.clone())
                    }
                    ast::r#type::Type::Array(ast::r#type::Scalar::Float) => {
                        QueryArgument::floats(argument_name.clone())
                    }
                    ast::r#type::Type::Array(ast::r#type::Scalar::Int) => {
                        QueryArgument::ints(argument_name.clone())
                    }
                    ast::r#type::Type::Array(ast::r#type::Scalar::String) => {
                        QueryArgument::strings(argument_name.clone())
                    }
                    ast::r#type::Type::Array(
                        ast::r#type::Scalar::Reference(name),
                    ) => {
                        if enum_names.contains(&name) {
                            QueryArgument {
                                r#type: QueryArgumentType::Enum(name.clone()),
                                name: argument_name.clone(),
                                cardinality: Cardinality::Many,
                            }
                        } else {
                            return invalid_query_argument();
                        }
                    }
                    _ => {
                        return invalid_query_argument();
                    }
                };

                query.arguments.push(argument);
            }

            if let Some(ast::query::Where {
                conditions: ast_conditions,
                name: alias,
            }) = &ast_where
            {
                let mut conditions = Vec::new();

                for ast::query::r#where::Condition {
                    path,
                    operator,
                    argument_name,
                } in ast_conditions
                {
                    let ast::query::r#where::Path(path_segments) = path;
                    let model_name = query.r#type.model_name.clone();

                    for argument in &query.arguments {
                        let invalid_condition = || {
                            Err(TypeError::InvalidQueryCondition {
                                query_name: query_name.clone(),
                                operator: *operator,
                                argument_name: argument_name.clone(),
                                field_name: path.to_string(),
                            })
                        };

                        if argument.name == *argument_name {
                            match &argument.r#type {
                                QueryArgumentType::Enum(argument_enum_name) => {
                                    if let Some(field_enum_name) = ir
                                        .resolve_model_enum(
                                            &model_name,
                                            &mut path_segments.clone(),
                                        )
                                    {
                                        if *argument_enum_name
                                            != field_enum_name
                                        {
                                            return invalid_condition();
                                        }
                                    } else {
                                        return invalid_condition();
                                    }
                                }
                                QueryArgumentType::Type(argument_type) => {
                                    if let Some(field_type) = ir
                                        .resolve_model_field(
                                            &model_name,
                                            &mut path_segments.clone(),
                                        )
                                    {
                                        if *argument_type != field_type {
                                            return invalid_condition();
                                        }
                                    } else {
                                        return invalid_condition();
                                    }
                                }
                            }
                        }
                    }

                    let condition = QueryCondition {
                        lhs: path_segments.clone(),
                        operator: (*operator).into(),
                        rhs: argument_name.clone(),
                    };

                    conditions.push(condition);
                }

                query.where_clause = Some(QueryWhereClause {
                    alias: alias.clone(),
                    conditions,
                });
            }
        }

        Ok(ir)
    }
}
