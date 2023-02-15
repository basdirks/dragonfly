use {
    self::{
        component::Component,
        model::{
            field::Field,
            Model,
        },
        query::{
            Argument,
            Condition,
            Query,
        },
        r#enum::Enum,
        r#type::Type,
        route::Route,
    },
    crate::{
        map,
        parser::{
            char_range::spaces,
            choice,
            map,
            ParseError,
            ParseResult,
        },
    },
    std::collections::{
        HashMap,
        HashSet,
    },
};

/// A JSX component.
pub mod component;
/// An enumerated type.
pub mod r#enum;
/// A data model.
pub mod model;
/// A data query.
pub mod query;
/// A route.
pub mod route;
/// Types used inside models and queries.
pub mod r#type;

/// A type check error.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum TypeError {
    /// The schema of a query is empty. This means that this query would not
    /// return any data.
    EmptyQuerySchema {
        /// The name of the empty query.
        query_name: String,
    },
    /// The structure of the schema of a query does not match the return type
    /// of the query.
    IncompatibleQuerySchema {
        /// The actual, inferred return type of the query.
        actual: Type,
        /// The expected return type of the query.
        expected: Type,
        /// The name of the query.
        query_name: String,
    },
    /// The type of a condition (as given by the corresponding argument) does
    /// not match the type of the field.
    IncompatibleQueryConditionType {
        /// The name of the query.
        query_name: String,
        /// The condition that was not satisfied.
        condition: Condition,
        /// The type of the condition as given by the argument.
        expected: Type,
    },
    /// The structure of a where clause of a query does not match the structure
    /// of the model and its relations.
    IncompatibleQueryWhere {
        /// The name of the query.
        query_name: String,
    },
    /// The name of the root node of the where clause of a query does not match
    /// the name of the root node of the schema.
    IncompatibleQueryRootNodes {
        /// The name of the schema root node.
        schema_root: String,
        /// The name of the query root node.
        where_root: String,
        /// The name of the query.
        query_name: String,
    },
    /// The type of an argument may not be an array or a model.
    InvalidQueryArgumentType {
        /// The argument that has an invalid type.
        argument: Argument,
        /// The name of the query.
        query_name: String,
    },
    /// The type of a field of a model is undefined.
    UnknownModelFieldType {
        /// The field whose type is undefined.
        field: Field,
        /// The name of the model.
        model_name: String,
    },
    /// The type of a query argument is undefined.
    UnknownQueryArgumentType {
        /// The argument whose type is undefined.
        argument: Argument,
        /// The name of the query.
        query_name: String,
    },
    /// The return type of a query is undefined.
    UnknownQueryReturnType {
        /// The name of the query.
        query_name: String,
        /// The return type of the query.
        r#type: Type,
    },
    /// A condition mentions an undefined argument.
    UnknownQueryConditionName {
        /// The condition that mentions an undefined argument.
        condition: Condition,
        /// The name of the query.
        query_name: String,
    },
    /// The root component of a route is undefined.
    UnknownRouteRoot {
        /// The name of the route.
        route_name: String,
        /// The name of the component.
        root: String,
    },
    /// An argument of a query is not used in the where clause.
    UnusedQueryArgument {
        /// The argument that is not used.
        argument: Argument,
        /// The name of the query.
        query_name: String,
    },
}

/// A declaration of a component, enum, model, query, or route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Declaration {
    /// The declaration of a JSX component.
    Component(Component),
    /// The declaration of an enumerated type.
    Enum(Enum),
    /// The declaration of a data model.
    Model(Model),
    /// The declaration of a data query.
    Query(Query),
    /// The declaration of a route.
    Route(Route),
}

impl Declaration {
    /// Parse a declaration from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// declaration.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     component::Component,
    ///     Declaration,
    /// };
    ///
    /// let input = "component Foo {
    ///     path: Foo
    /// }";
    ///
    /// let expected = Declaration::Component(Component {
    ///     name: "Foo".to_string(),
    ///     path: "Foo".to_string(),
    /// });
    ///
    /// assert_eq!(Declaration::parse(input), Ok((expected, "".to_string())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     r#enum::Enum,
    ///     Declaration,
    /// };
    ///
    /// let input = "enum Foo {
    ///     Bar
    ///     Baz
    /// }";
    ///
    /// let expected = Declaration::Enum(Enum {
    ///     name: "Foo".to_string(),
    ///     variants: vec!["Bar".to_string(), "Baz".to_string()],
    /// });
    ///
    /// assert_eq!(Declaration::parse(input), Ok((expected, "".to_string())));
    /// ```
    ///
    /// ```rust
    /// use {
    ///     dragonfly::ast::{
    ///         model::{
    ///             field::Field,
    ///             Model,
    ///         },
    ///         query::Query,
    ///         r#type::{
    ///             Basic,
    ///             Type,
    ///         },
    ///         route::Route,
    ///         Declaration,
    ///     },
    ///     std::collections::HashMap,
    /// };
    ///
    /// let input = "model Foo {
    ///     foo: String
    ///     bar: [Bar]
    /// }";
    ///
    /// let mut fields = HashMap::new();
    ///
    /// fields.insert(
    ///     "foo".to_string(),
    ///     Field {
    ///         name: "foo".to_string(),
    ///         r#type: Type::One(Basic::String),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "bar".to_string(),
    ///     Field {
    ///         name: "bar".to_string(),
    ///         r#type: Type::Array(Basic::Identifier("Bar".to_string())),
    ///     },
    /// );
    ///
    /// let expected = Declaration::Model(Model {
    ///     name: "Foo".to_string(),
    ///     fields,
    /// });
    ///
    /// assert_eq!(Declaration::parse(input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice(
            input,
            vec![
                map!(Component::parse, Declaration::Component),
                map!(Enum::parse, Declaration::Enum),
                map!(Model::parse, Declaration::Model),
                map!(Query::parse, Declaration::Query),
                map!(Route::parse, Declaration::Route),
            ],
        )
    }
}

/// The root of an AST.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ast {
    /// Component declarations.
    pub components: HashMap<String, Component>,
    /// Enum declarations.
    pub enums: HashMap<String, Enum>,
    /// Model declarations.
    pub models: HashMap<String, Model>,
    /// Query declarations.
    pub queries: HashMap<String, Query>,
    /// Route declarations.
    pub routes: HashMap<String, Route>,
}

impl Ast {
    /// Create a new AST.
    #[must_use]
    pub fn new() -> Self {
        Self {
            components: HashMap::new(),
            enums: HashMap::new(),
            models: HashMap::new(),
            queries: HashMap::new(),
            routes: HashMap::new(),
        }
    }

    /// Parse an AST from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input is not a valid AST.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     dragonfly::ast::{
    ///         component::Component,
    ///         model::{
    ///             field::Field,
    ///             Model,
    ///         },
    ///         query::{
    ///             Argument,
    ///             Condition,
    ///             Query,
    ///             Schema,
    ///             Where,
    ///         },
    ///         r#enum::Enum,
    ///         r#type::{
    ///             Basic,
    ///             Type,
    ///         },
    ///         route::Route,
    ///         Ast,
    ///         Declaration,
    ///     },
    ///     std::collections::HashMap,
    /// };
    ///
    /// let input = "route / {
    ///   root: Home
    ///   title: Home
    /// }
    ///
    /// component Home {
    ///   path: Home
    /// }
    ///
    /// model Image {
    ///   id: ID
    ///   title: String
    ///   country: Country
    ///   category: [Category]
    /// }
    ///
    /// query images: [Image] {
    ///   image {
    ///     title
    ///     country {
    ///       name
    ///     }
    ///     category
    ///   }
    /// }
    ///
    /// query imagesByCountryName($name: CountryName): [Image] {
    ///   image {
    ///     title
    ///     category
    ///   }
    ///   where {
    ///     image {
    ///       country {
    ///         name {
    ///           equals: $name
    ///         }
    ///       }
    ///     }
    ///   }
    /// }
    ///
    /// enum DrivingSide {
    ///   Left
    ///   Right
    /// }
    ///
    /// model Country {
    ///   id: ID
    ///   domain: String
    ///   drivingSide: DrivingSide
    ///   flag: String
    ///   name: CountryName
    /// }
    ///
    /// enum CountryName {
    ///   Albania
    ///   Andorra
    ///   Austria
    ///   Yemen
    ///   Zambia
    ///   Zimbabwe
    /// }
    ///
    /// enum Category {
    ///   Architecture
    ///   Bollard
    ///   Chevron
    ///   TrafficLight
    ///   TrafficSign
    ///   UtilityPole
    /// }";
    ///
    /// let mut expected = Ast::new();
    ///
    /// expected.routes.insert(
    ///     "/".to_string(),
    ///     Route {
    ///         path: "/".to_string(),
    ///         root: "Home".to_string(),
    ///         title: "Home".to_string(),
    ///     },
    /// );
    ///
    /// expected.components.insert(
    ///     "Home".to_string(),
    ///     Component {
    ///         name: "Home".to_string(),
    ///         path: "Home".to_string(),
    ///     },
    /// );
    ///
    /// let mut fields = HashMap::new();
    ///
    /// fields.insert(
    ///     "id".to_string(),
    ///     Field {
    ///         name: "id".to_string(),
    ///         r#type: Type::One(Basic::Identifier("ID".to_string())),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "title".to_string(),
    ///     Field {
    ///         name: "title".to_string(),
    ///         r#type: Type::One(Basic::String),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "country".to_string(),
    ///     Field {
    ///         name: "country".to_string(),
    ///         r#type: Type::One(Basic::Identifier("Country".to_string())),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "category".to_string(),
    ///     Field {
    ///         name: "category".to_string(),
    ///         r#type: Type::Array(Basic::Identifier("Category".to_string())),
    ///     },
    /// );
    ///
    /// expected.models.insert(
    ///     "Image".to_string(),
    ///     Model {
    ///         name: "Image".to_string(),
    ///         fields,
    ///     },
    /// );
    ///
    /// let mut fields = HashMap::new();
    ///
    /// fields.insert(
    ///     "id".to_string(),
    ///     Field {
    ///         name: "id".to_string(),
    ///         r#type: Type::One(Basic::Identifier("ID".to_string())),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "domain".to_string(),
    ///     Field {
    ///         name: "domain".to_string(),
    ///         r#type: Type::One(Basic::String),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "drivingSide".to_string(),
    ///     Field {
    ///         name: "drivingSide".to_string(),
    ///         r#type: Type::One(Basic::Identifier("DrivingSide".to_string())),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "flag".to_string(),
    ///     Field {
    ///         name: "flag".to_string(),
    ///         r#type: Type::One(Basic::String),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "name".to_string(),
    ///     Field {
    ///         name: "name".to_string(),
    ///         r#type: Type::One(Basic::Identifier("CountryName".to_string())),
    ///     },
    /// );
    ///
    /// expected.models.insert(
    ///     "Country".to_string(),
    ///     Model {
    ///         name: "Country".to_string(),
    ///         fields,
    ///     },
    /// );
    ///
    /// expected.enums.insert(
    ///     "DrivingSide".to_string(),
    ///     Enum {
    ///         name: "DrivingSide".to_string(),
    ///         variants: vec!["Left".to_string(), "Right".to_string()],
    ///     },
    /// );
    ///
    /// expected.enums.insert(
    ///     "CountryName".to_string(),
    ///     Enum {
    ///         name: "CountryName".to_string(),
    ///         variants: vec![
    ///             "Albania".to_string(),
    ///             "Andorra".to_string(),
    ///             "Austria".to_string(),
    ///             "Yemen".to_string(),
    ///             "Zambia".to_string(),
    ///             "Zimbabwe".to_string(),
    ///         ],
    ///     },
    /// );
    ///
    /// expected.enums.insert(
    ///     "Category".to_string(),
    ///     Enum {
    ///         name: "Category".to_string(),
    ///         variants: vec![
    ///             "Architecture".to_string(),
    ///             "Bollard".to_string(),
    ///             "Chevron".to_string(),
    ///             "TrafficLight".to_string(),
    ///             "TrafficSign".to_string(),
    ///             "UtilityPole".to_string(),
    ///         ],
    ///     },
    /// );
    ///
    /// expected.queries.insert(
    ///     "images".to_string(),
    ///     Query {
    ///         name: "images".to_string(),
    ///         r#type: Type::Array(Basic::Identifier("Image".to_string())),
    ///         schema: Schema::Model {
    ///             name: "image".to_string(),
    ///             nodes: vec![
    ///                 Schema::Field("title".to_string()),
    ///                 Schema::Model {
    ///                     name: "country".to_string(),
    ///                     nodes: vec![Schema::Field("name".to_string())],
    ///                 },
    ///                 Schema::Field("category".to_string()),
    ///             ],
    ///         },
    ///         r#where: None,
    ///         arguments: vec![],
    ///     },
    /// );
    ///
    /// expected.queries.insert(
    ///     "imagesByCountryName".to_string(),
    ///     Query {
    ///         name: "imagesByCountryName".to_string(),
    ///         r#type: Type::Array(Basic::Identifier("Image".to_string())),
    ///         schema: Schema::Model {
    ///             name: "image".to_string(),
    ///             nodes: vec![
    ///                 Schema::Field("title".to_string()),
    ///                 Schema::Field("category".to_string()),
    ///             ],
    ///         },
    ///         r#where: Some(Where::Node {
    ///             name: "image".to_string(),
    ///             nodes: vec![Where::Node {
    ///                 name: "country".to_string(),
    ///                 nodes: vec![Where::Node {
    ///                     name: "name".to_string(),
    ///                     nodes: vec![Where::Condition(Condition::Equals(
    ///                         "name".to_string(),
    ///                     ))],
    ///                 }],
    ///             }],
    ///         }),
    ///         arguments: vec![Argument {
    ///             name: "name".to_string(),
    ///             r#type: Type::One(Basic::Identifier("CountryName".to_string())),
    ///         }],
    ///     },
    /// );
    ///
    /// assert_eq!(Ast::parse(&input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let mut input = input.to_string();
        let mut ast = Self::new();

        while !input.is_empty() {
            let (_, new_input) = spaces(&input)?;

            if let Ok((declaration, new_input)) = Component::parse(&new_input) {
                let name = declaration.name.clone();

                if ast.components.insert(name.clone(), declaration).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!("Component {name} already defined"),
                    });
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Model::parse(&new_input)
            {
                let name = declaration.name.clone();

                if ast.models.insert(name.clone(), declaration).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!("Model {name} already defined"),
                    });
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Query::parse(&new_input)
            {
                let name = declaration.name.clone();

                if ast.queries.insert(name.clone(), declaration).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!("Query {name} already defined"),
                    });
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) = Enum::parse(&new_input)
            {
                let name = declaration.name.clone();

                if ast.enums.insert(name.clone(), declaration).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!("Enum {name} already defined"),
                    });
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Route::parse(&new_input)
            {
                let path = declaration.path.clone();

                if ast.routes.insert(path.clone(), declaration).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!(
                            "Route with path {path} already defined"
                        ),
                    });
                }

                input = new_input;
            } else {
                return Err(ParseError::CustomError {
                    message: "Expected a component, model, query, enum or page"
                        .to_string(),
                });
            }

            let (_, new_input) = spaces(&input)?;

            input = new_input;
        }

        let (_, input) = spaces(&input)?;

        Ok((ast, input))
    }

    /// Check if the AST is valid.
    ///
    /// # Errors
    ///
    /// ## Query errors
    ///
    /// * `TypeError::EmptyQuerySchema`
    /// if the schema of a query is empty.
    ///
    /// * `TypeError::IncompatibleQuerySchema`
    /// if the schema of a query is incompatible with the structure of the
    /// models and their relations.
    ///
    /// * `TypeError::IncompatibleQueryConditionType`
    /// if the type of an argument in a condition does not match the type of the
    /// corresponding model field.
    ///
    /// * `TypeError::IncompatibleQueryWhere`
    /// if the where clause of a query is incompatible to the structure of the
    /// models and their relations.
    ///
    /// * `TypeError::IncompatibleQueryRootNodes`
    /// if the top-level schema node name does not match the name of the root
    /// node in the where clause.
    ///
    /// * `TypeError::InvalidQueryArgumentType`
    /// if the type of a query argument is an array or a model.
    ///
    /// * `TypeError::UnknownQueryArgumentType`
    /// if the type of a query argument is undefined.
    ///
    /// * `TypeError::UnknownQueryReturnType`
    /// if the return type of a query is does not match the (inferred) type of
    /// the schema.
    ///
    /// * `TypeError::UnknownQueryConditionName`
    /// if a condition refers to an undefined query argument.
    ///
    /// * `TypeError::UnusedQueryArgument`
    /// if a query argument is not used at least once in the where clause.
    ///
    /// ## Route errors
    ///
    /// * `TypeError::UnknownRouteRoot`
    /// if the root component of a route is undefined.
    ///
    /// ## Model errors
    ///
    /// * `TypeError::UnknownModelFieldType`
    /// if the type of a field in a model is undefined.
    pub fn check(&self) -> Result<(), TypeError> {
        for query in self.queries.values() {
            // Self::check_query_schema(query, self)?;
            // Self::check_query_where(query, self)?;
            // Self::check_query_condition_types(query, self)?;
            // Self::check_query_condition_names(query, self)?;
            // Self::check_query_argument_types(query, self)?;
            // Self::check_query_return_type(query, self)?;
            // query.check_argument_types(self)?;
            query.check_unused_arguments()?;
            query.check_non_empty_schema()?;
            query.check_root_nodes()?;
        }

        // for model in self.models.values() {
        //     Self::check_model_field_types(model, self)?;
        // }

        // for route in self.routes.values() {
        //     Self::check_route_root(route, self)?;
        // }

        Ok(())
    }

    /// Return the names of all top-level types in the AST.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     dragonfly::ast::Ast,
    ///     std::collections::HashSet,
    /// };
    ///
    /// let input = "model User {
    ///     name: String
    /// }
    ///
    /// enum CountryName {
    ///     Germany
    ///     France
    /// }
    ///
    /// model Country {
    ///     name: CountryName
    /// }";
    ///
    /// let mut expected = HashSet::new();
    ///
    /// expected.insert("User".to_string());
    /// expected.insert("Country".to_string());
    /// expected.insert("CountryName".to_string());
    ///
    /// assert_eq!(Ast::parse(input).unwrap().0.type_names(), expected);
    /// ```
    #[must_use]
    pub fn type_names(&self) -> HashSet<String> {
        let mut names = HashSet::new();

        for model in self.models.values() {
            names.insert(model.name.clone());
        }

        for r#enum in self.enums.values() {
            names.insert(r#enum.name.clone());
        }

        names
    }
}
