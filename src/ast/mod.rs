use self::{
    component::Component, model::Model, query::Query, r#enum::Enum, r#type::Type, route::Route,
};
use crate::parser::{choice, map_ok, spaces, ParseError, ParseResult};
use std::collections::HashMap;

pub mod component;
pub mod r#enum;
pub mod model;
pub mod query;
pub mod route;
pub mod r#type;

pub enum TypeError {
    DuplicateEnumVariant(String),
    DuplicateModelField(String),
    IncompatibleQuerySchema,
    IncompatibleQueryWhere,
    UnknownModelType(Type),
    UnknownQueryArgumentType(Type),
    UnknownQueryReturnType(Type),
    UnknownRouteRoot(String),
    UnknownSelectorField(String),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Statement {
    Component(Component),
    Enum(Enum),
    Model(Model),
    Query(Query),
    Route(Route),
}

impl Statement {
    /// Parse a statement from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid statement.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Statement;
    /// use dragonfly::ast::component::Component;
    /// use dragonfly::ast::r#enum::Enum;
    /// use dragonfly::ast::model::{field::Field, Model};
    /// use dragonfly::ast::query::Query;
    /// use dragonfly::ast::route::Route;
    /// use dragonfly::ast::r#type::{Primitive, Type};
    /// use std::collections::HashSet;
    ///
    /// let input = "component Foo {
    ///     path: /foo
    /// }";
    ///
    /// let expected = Statement::Component(Component {
    ///     name: "Foo".to_string(),
    ///     path: "/foo".to_string(),
    /// });
    ///
    /// assert_eq!(Statement::parse(input), Ok((expected, "".to_string())));
    ///
    /// let input = "enum Foo {
    ///     Bar
    ///     Baz
    /// }";
    ///
    /// let mut variants = HashSet::new();
    ///
    /// variants.insert("Bar".to_string());
    /// variants.insert("Baz".to_string());
    ///
    /// let expected = Statement::Enum(Enum {
    ///     name: "Foo".to_string(),
    ///     variants,
    /// });
    ///
    /// assert_eq!(Statement::parse(input), Ok((expected, "".to_string())));
    ///
    /// let input = "model Foo {
    ///     foo: String
    ///     bar: [Bar]
    /// }";
    ///
    /// let expected = Statement::Model(Model {
    ///     name: "Foo".to_string(),
    ///     fields: vec![
    ///         Field {
    ///             name: "foo".to_string(),
    ///             r#type: Type::One(Primitive::String),
    ///         },
    ///         Field {
    ///             name: "bar".to_string(),
    ///             r#type: Type::Array(Primitive::Identifier("Bar".to_string())),
    ///         },
    ///      ],
    /// });
    ///
    /// assert_eq!(Statement::parse(input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice(
            input,
            vec![
                |input| map_ok(input, Component::parse, Statement::Component),
                |input| map_ok(input, Enum::parse, Statement::Enum),
                |input| map_ok(input, Model::parse, Statement::Model),
                |input| map_ok(input, Query::parse, Statement::Query),
                |input| map_ok(input, Route::parse, Statement::Route),
            ],
        )
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ast {
    pub components: HashMap<String, Component>,
    pub enums: HashMap<String, Enum>,
    pub models: HashMap<String, Model>,
    pub queries: HashMap<String, Query>,
    pub routes: HashMap<String, Route>,
}

impl Ast {
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
    /// * If the input is not a valid AST.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Ast;
    /// use dragonfly::ast::component::Component;
    /// use dragonfly::ast::r#enum::Enum;
    /// use dragonfly::ast::model::{field::Field, Model};
    /// use dragonfly::ast::query::{Argument, Query, Schema, Selector, Where};
    /// use dragonfly::ast::route::Route;
    /// use dragonfly::ast::Statement;
    /// use dragonfly::ast::r#type::{Primitive, Type};
    /// use std::collections::HashSet;
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
    ///             Field {
    ///                 name: "id".to_string(),
    ///                 r#type: Type::One(Primitive::Identifier("ID".to_string())),
    ///             },
    ///             Field {
    ///                 name: "title".to_string(),
    ///                 r#type: Type::One(Primitive::String),
    ///             },
    ///             Field {
    ///                 name: "country".to_string(),
    ///                 r#type: Type::One(Primitive::Identifier("Country".to_string())),
    ///             },
    ///             Field {
    ///                 name: "category".to_string(),
    ///                 r#type: Type::Array(Primitive::Identifier("Category".to_string())),
    ///             },
    ///         ],
    ///     },
    /// );
    ///
    /// expected.models.insert(
    ///     "Country".to_string(),
    ///     Model {
    ///         name: "Country".to_string(),
    ///         fields: vec![
    ///             Field {
    ///                 name: "id".to_string(),
    ///                 r#type: Type::One(Primitive::Identifier("ID".to_string())),
    ///             },
    ///             Field {
    ///                 name: "domain".to_string(),
    ///                 r#type: Type::One(Primitive::String),
    ///             },
    ///             Field {
    ///                 name: "drivingSide".to_string(),
    ///                 r#type: Type::One(Primitive::Identifier("DrivingSide".to_string())),
    ///             },
    ///             Field {
    ///                 name: "flag".to_string(),
    ///                 r#type: Type::One(Primitive::String),
    ///             },
    ///             Field {
    ///                 name: "name".to_string(),
    ///                 r#type: Type::One(Primitive::Identifier("CountryName".to_string())),
    ///             },
    ///         ],
    ///     },
    /// );
    ///
    /// let mut variants = HashSet::new();
    ///
    /// variants.insert("Left".to_string());
    /// variants.insert("Right".to_string());
    ///
    /// expected.enums.insert(
    ///     "DrivingSide".to_string(),
    ///     Enum {
    ///         name: "DrivingSide".to_string(),
    ///         variants,
    ///     },
    /// );
    ///
    /// let mut variants = HashSet::new();
    ///
    /// variants.insert("Albania".to_string());
    /// variants.insert("Andorra".to_string());
    /// variants.insert("Austria".to_string());
    /// variants.insert("Yemen".to_string());
    /// variants.insert("Zambia".to_string());
    /// variants.insert("Zimbabwe".to_string());
    ///
    /// expected.enums.insert(
    ///     "CountryName".to_string(),
    ///     Enum {
    ///         name: "CountryName".to_string(),
    ///         variants,
    ///     },
    /// );
    ///
    /// let mut variants = HashSet::new();
    ///
    /// variants.insert("Architecture".to_string());
    /// variants.insert("Bollard".to_string());
    /// variants.insert("Chevron".to_string());
    /// variants.insert("TrafficLight".to_string());
    /// variants.insert("TrafficSign".to_string());
    /// variants.insert("UtilityPole".to_string());
    ///
    /// expected.enums.insert(
    ///     "Category".to_string(),
    ///     Enum {
    ///         name: "Category".to_string(),
    ///         variants,
    ///     },
    /// );
    ///
    /// expected.queries.insert(
    ///     "images".to_string(),
    ///     Query {
    ///         name: "images".to_string(),
    ///         r#type: Type::Array(Primitive::Identifier("Image".to_string())),
    ///         schema: Schema::Node(
    ///             "image".to_string(),
    ///             vec![
    ///                 Schema::Identifier("title".to_string()),
    ///                 Schema::Node(
    ///                     "country".to_string(),
    ///                     vec![Schema::Identifier("name".to_string())],
    ///                 ),
    ///                 Schema::Identifier("category".to_string()),
    ///             ],
    ///         ),
    ///         r#where: None,
    ///         arguments: vec![],
    ///     },
    /// );
    ///
    /// expected.queries.insert(
    ///     "imagesByCountryName".to_string(),
    ///     Query {
    ///         name: "imagesByCountryName".to_string(),
    ///         r#type: Type::Array(Primitive::Identifier("Image".to_string())),
    ///         schema: Schema::Node(
    ///             "image".to_string(),
    ///             vec![
    ///                 Schema::Identifier("title".to_string()),
    ///                 Schema::Identifier("category".to_string()),
    ///             ],
    ///         ),
    ///         r#where: Some(
    ///             Where::Node(
    ///                 "image".to_string(),
    ///                 vec![Where::Node(
    ///                     "country".to_string(),
    ///                     vec![Where::Node(
    ///                         "name".to_string(),
    ///                         vec![Where::Selector(
    ///                             Selector::Equals("name".to_string()),
    ///                         )],
    ///                     )],         
    ///                 )],
    ///             ),
    ///         ),
    ///         arguments: vec![Argument {
    ///             name: "name".to_string(),
    ///             r#type: Type::One(Primitive::Identifier("CountryName".to_string())),
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

            if let Ok((statement, new_input)) = Component::parse(&new_input) {
                let name = statement.name.clone();

                if ast.components.insert(name.clone(), statement).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!("Component {name} already defined"),
                    });
                }

                input = new_input;
            } else if let Ok((statement, new_input)) = Model::parse(&new_input) {
                let name = statement.name.clone();

                if ast.models.insert(name.clone(), statement).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!("Model {name} already defined"),
                    });
                }

                input = new_input;
            } else if let Ok((statement, new_input)) = Query::parse(&new_input) {
                let name = statement.name.clone();

                if ast.queries.insert(name.clone(), statement).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!("Query {name} already defined"),
                    });
                }

                input = new_input;
            } else if let Ok((statement, new_input)) = Enum::parse(&new_input) {
                let name = statement.name.clone();

                if ast.enums.insert(name.clone(), statement).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!("Enum {name} already defined"),
                    });
                }

                input = new_input;
            } else if let Ok((statement, new_input)) = Route::parse(&new_input) {
                let path = statement.path.clone();

                if ast.routes.insert(path.clone(), statement).is_some() {
                    return Err(ParseError::CustomError {
                        message: format!("Route with path {path} already defined"),
                    });
                }

                input = new_input;
            } else {
                return Err(ParseError::CustomError {
                    message: "Expected a component, model, query, enum or page".to_string(),
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
    /// * `TypeError` - If the AST is not valid.
    pub const fn typecheck(&self) -> Result<(), TypeError> {
        Ok(())
    }
}
