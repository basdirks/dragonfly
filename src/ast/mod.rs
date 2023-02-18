pub use self::{
    component::Component,
    model::{
        Field,
        Model,
    },
    query::{
        Argument as QueryArgument,
        Condition as QueryCondition,
        Query,
        Schema as QuerySchema,
        Where as QueryWhere,
    },
    r#enum::Enum,
    r#type::{
        Scalar,
        Type,
    },
    route::Route,
    type_error::TypeError,
};
use {
    crate::{
        map,
        parser::{
            choice,
            map,
            spaces,
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
/// Type checking errors.
pub mod type_error;

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
    ///     Component,
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
    ///     Declaration,
    ///     Enum,
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
    /// use dragonfly::ast::{
    ///     Declaration,
    ///     Field,
    ///     Model,
    ///     Query,
    ///     Route,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "model Foo {
    ///     foo: String
    ///     bar: [Bar]
    /// }";
    ///
    /// let expected = Declaration::Model(Model {
    ///     name: "Foo".to_string(),
    ///     fields: vec![
    ///         (
    ///             "foo".to_string(),
    ///             Field {
    ///                 name: "foo".to_string(),
    ///                 r#type: Type::Scalar(Scalar::String),
    ///             },
    ///         ),
    ///         (
    ///             "bar".to_string(),
    ///             Field {
    ///                 name: "bar".to_string(),
    ///                 r#type: Type::Array(Scalar::Reference("Bar".to_string())),
    ///             },
    ///         ),
    ///     ]
    ///     .into_iter()
    ///     .collect(),
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
    /// use dragonfly::ast::{
    ///     Ast,
    ///     Component,
    ///     Declaration,
    ///     Enum,
    ///     Field,
    ///     Model,
    ///     Query,
    ///     QueryArgument,
    ///     QueryCondition,
    ///     QuerySchema,
    ///     QueryWhere,
    ///     Route,
    ///     Scalar,
    ///     Type,
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
    /// expected.models.insert(
    ///     "Image".to_string(),
    ///     Model {
    ///         name: "Image".to_string(),
    ///         fields: vec![
    ///             (
    ///                 "id".to_string(),
    ///                 Field {
    ///                     name: "id".to_string(),
    ///                     r#type: Type::Scalar(Scalar::Reference(
    ///                         "ID".to_string(),
    ///                     )),
    ///                 },
    ///             ),
    ///             (
    ///                 "title".to_string(),
    ///                 Field {
    ///                     name: "title".to_string(),
    ///                     r#type: Type::Scalar(Scalar::String),
    ///                 },
    ///             ),
    ///             (
    ///                 "country".to_string(),
    ///                 Field {
    ///                     name: "country".to_string(),
    ///                     r#type: Type::Scalar(Scalar::Reference(
    ///                         "Country".to_string(),
    ///                     )),
    ///                 },
    ///             ),
    ///             (
    ///                 "category".to_string(),
    ///                 Field {
    ///                     name: "category".to_string(),
    ///                     r#type: Type::Array(Scalar::Reference(
    ///                         "Category".to_string(),
    ///                     )),
    ///                 },
    ///             ),
    ///         ]
    ///         .into_iter()
    ///         .collect(),
    ///     },
    /// );
    ///
    /// expected.models.insert(
    ///     "Country".to_string(),
    ///     Model {
    ///         name: "Country".to_string(),
    ///         fields: vec![
    ///             (
    ///                 "id".to_string(),
    ///                 Field {
    ///                     name: "id".to_string(),
    ///                     r#type: Type::Scalar(Scalar::Reference(
    ///                         "ID".to_string(),
    ///                     )),
    ///                 },
    ///             ),
    ///             (
    ///                 "domain".to_string(),
    ///                 Field {
    ///                     name: "domain".to_string(),
    ///                     r#type: Type::Scalar(Scalar::String),
    ///                 },
    ///             ),
    ///             (
    ///                 "drivingSide".to_string(),
    ///                 Field {
    ///                     name: "drivingSide".to_string(),
    ///                     r#type: Type::Scalar(Scalar::Reference(
    ///                         "DrivingSide".to_string(),
    ///                     )),
    ///                 },
    ///             ),
    ///             (
    ///                 "flag".to_string(),
    ///                 Field {
    ///                     name: "flag".to_string(),
    ///                     r#type: Type::Scalar(Scalar::String),
    ///                 },
    ///             ),
    ///             (
    ///                 "name".to_string(),
    ///                 Field {
    ///                     name: "name".to_string(),
    ///                     r#type: Type::Scalar(Scalar::Reference(
    ///                         "CountryName".to_string(),
    ///                     )),
    ///                 },
    ///             ),
    ///         ]
    ///         .into_iter()
    ///         .collect(),
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
    ///         r#type: Type::Array(Scalar::Reference("Image".to_string())),
    ///         schema: QuerySchema::Model {
    ///             name: "image".to_string(),
    ///             nodes: vec![
    ///                 QuerySchema::Field("title".to_string()),
    ///                 QuerySchema::Model {
    ///                     name: "country".to_string(),
    ///                     nodes: vec![QuerySchema::Field("name".to_string())],
    ///                 },
    ///                 QuerySchema::Field("category".to_string()),
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
    ///         r#type: Type::Array(Scalar::Reference("Image".to_string())),
    ///         schema: QuerySchema::Model {
    ///             name: "image".to_string(),
    ///             nodes: vec![
    ///                 QuerySchema::Field("title".to_string()),
    ///                 QuerySchema::Field("category".to_string()),
    ///             ],
    ///         },
    ///         r#where: Some(QueryWhere::Node {
    ///             name: "image".to_string(),
    ///             nodes: vec![QueryWhere::Node {
    ///                 name: "country".to_string(),
    ///                 nodes: vec![QueryWhere::Node {
    ///                     name: "name".to_string(),
    ///                     nodes: vec![QueryWhere::Condition(
    ///                         QueryCondition::Equals("name".to_string()),
    ///                     )],
    ///                 }],
    ///             }],
    ///         }),
    ///         arguments: vec![QueryArgument {
    ///             name: "name".to_string(),
    ///             r#type: Type::Scalar(Scalar::Reference(
    ///                 "CountryName".to_string(),
    ///             )),
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

    /// Check for errors in individual entities.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError::EmptyQuerySchema` if the schema of any query does
    /// not contain any fields.
    ///
    /// Returns a `TypeError::UnusedQueryArgument` if any query argument is not
    /// used in the query's `where` clause.
    ///
    /// Returns a `TypeError::IncompatibleQueryRootNodes` if the root nodes of
    /// any query's schema and `where` clause do not match.
    ///
    /// Returns a `TypeError::UnknownQueryConditionReference` if any query
    /// selector references an argument that does not exist.
    pub fn check_entities(&self) -> Result<(), TypeError> {
        // Many of these checks could be combined into a single pass over the
        // AST, but doing them separately is easier to understand.

        for query in self.queries.values() {
            // Self::check_query_schema(query, self)?;
            // Self::check_query_where(query, self)?;
            // Self::check_query_condition_types(query, self)?;
            query.check_unused_arguments()?;
            query.check_empty_schema()?;
            query.check_root_nodes()?;
            query.check_condition_references()?;
        }

        Ok(())
    }

    /// Check for cross-entity type errors.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError::InvalidQueryArgumentType` if the type of any query
    /// is not a primitive or a reference to an enum.
    ///
    /// Returns a `TypeError::InvalidQueryReturnType` if the return type of any
    /// query is not a reference to a known model.
    ///
    /// Returns a `TypeError::UnknownRouteRoot` if the root of any route is not
    /// a reference to a known component.
    ///
    /// Returns a `TypeError::InvalidModelFieldType` if the type of any model
    /// field is not a primitive, a reference to a known enum or model, or an
    /// array of any such a type.
    pub fn check_types(&self) -> Result<(), TypeError> {
        // We could return the model relations during this pass, but it's
        // easier to understand if we do it separately.

        let model_names = self.models.keys().cloned().collect::<HashSet<_>>();
        let component_names =
            self.components.keys().cloned().collect::<HashSet<_>>();
        let enum_names = self.enums.keys().cloned().collect::<HashSet<_>>();

        for query in self.queries.values() {
            query.check_argument_types(&self.enums)?;
            query.check_return_type(&model_names)?;
        }

        for model in self.models.values() {
            model.check_field_types(&model_names, &enum_names)?;
        }

        for route in self.routes.values() {
            route.check_root(&component_names)?;
        }

        Ok(())
    }

    /// Return the names of all top-level types in the AST.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Ast;
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
    /// assert_eq!(
    ///     Ast::parse(input).unwrap().0.type_names(),
    ///     vec!["User", "Country", "CountryName"]
    ///         .iter()
    ///         .map(ToString::to_string)
    ///         .collect()
    /// )
    /// ```
    #[must_use]
    pub fn type_names(&self) -> HashSet<String> {
        let mut names = self
            .models
            .values()
            .map(|model| model.name.clone())
            .collect::<HashSet<_>>();

        names.extend(self.enums.values().map(|r#enum| r#enum.name.clone()));

        names
    }
}
