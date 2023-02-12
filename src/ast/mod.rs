use crate::parser::{choice, map_ok, spaces, ParseResult};

use self::{component::Component, model::Model, query::Query, r#enum::Enum, route::Route};

pub mod component;
pub mod r#enum;
pub mod model;
pub mod query;
pub mod route;
pub mod r#type;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct Ast {
    pub statements: Vec<Statement>,
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
    /// let expected = Statement::Enum(Enum {
    ///     name: "Foo".to_string(),
    ///     variants: vec!["Bar".to_string(), "Baz".to_string()],
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

impl Ast {
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
    /// let expected = Ast {
    ///     statements: vec![
    ///         Statement::Route(Route {
    ///             path: "/".to_string(),
    ///             root: "Home".to_string(),
    ///             title: "Home".to_string(),
    ///         }),
    ///         Statement::Component(Component {
    ///             name: "Home".to_string(),
    ///             path: "Home".to_string(),
    ///         }),
    ///         Statement::Model(Model {
    ///             name: "Image".to_string(),
    ///             fields: vec![
    ///                 Field {
    ///                     name: "id".to_string(),
    ///                     r#type: Type::One(Primitive::Identifier("ID".to_string())),
    ///                 },
    ///                 Field {
    ///                     name: "title".to_string(),
    ///                     r#type: Type::One(Primitive::String),
    ///                 },
    ///                 Field {
    ///                     name: "country".to_string(),
    ///                     r#type: Type::One(Primitive::Identifier("Country".to_string())),
    ///                 },
    ///                 Field {
    ///                     name: "category".to_string(),
    ///                     r#type: Type::Array(Primitive::Identifier("Category".to_string())),
    ///                 },
    ///             ],
    ///         }),
    ///         Statement::Query(Query {
    ///             name: "images".to_string(),
    ///             r#type: Type::Array(Primitive::Identifier("Image".to_string())),
    ///             schema: Schema::Node(
    ///                 "image".to_string(),
    ///                 vec![
    ///                     Schema::Identifier("title".to_string()),
    ///                     Schema::Node(
    ///                         "country".to_string(),
    ///                         vec![Schema::Identifier("name".to_string())],
    ///                     ),
    ///                     Schema::Identifier("category".to_string()),
    ///                 ],
    ///             ),
    ///             r#where: None,
    ///             arguments: vec![],
    ///         }),
    ///         Statement::Query(Query {
    ///             name: "imagesByCountryName".to_string(),
    ///             r#type: Type::Array(Primitive::Identifier("Image".to_string())),
    ///             schema: Schema::Node(
    ///                 "image".to_string(),
    ///                 vec![
    ///                     Schema::Identifier("title".to_string()),
    ///                     Schema::Identifier("category".to_string()),
    ///                 ],
    ///             ),
    ///             r#where: Some(
    ///                 Where::Node(
    ///                     "image".to_string(),
    ///                     vec![Where::Node(
    ///                         "country".to_string(),
    ///                         vec![Where::Node(
    ///                             "name".to_string(),
    ///                             vec![Where::Selector(
    ///                                 Selector::Equals("name".to_string()),
    ///                             )],
    ///                         )],         
    ///                     )],
    ///                 ),
    ///             ),
    ///             arguments: vec![Argument {
    ///                 name: "name".to_string(),
    ///                 r#type: Type::One(Primitive::Identifier("CountryName".to_string())),
    ///             }],
    ///         }),
    ///         Statement::Enum(Enum {
    ///             name: "DrivingSide".to_string(),
    ///             variants: vec![
    ///                 "Left".to_string(),
    ///                 "Right".to_string(),
    ///             ],
    ///         }),
    ///         Statement::Model(Model {
    ///             name: "Country".to_string(),
    ///             fields: vec![
    ///                 Field {
    ///                     name: "id".to_string(),
    ///                     r#type: Type::One(Primitive::Identifier("ID".to_string())),
    ///                 },
    ///                 Field {
    ///                     name: "domain".to_string(),
    ///                     r#type: Type::One(Primitive::String),
    ///                 },
    ///                 Field {
    ///                     name: "drivingSide".to_string(),
    ///                     r#type: Type::One(Primitive::Identifier("DrivingSide".to_string())),
    ///                 },
    ///                 Field {
    ///                     name: "flag".to_string(),
    ///                     r#type: Type::One(Primitive::String),
    ///                 },
    ///                 Field {
    ///                     name: "name".to_string(),
    ///                     r#type: Type::One(Primitive::Identifier("CountryName".to_string())),
    ///                 },
    ///             ],
    ///         }),
    ///         Statement::Enum(Enum {
    ///             name: "CountryName".to_string(),
    ///             variants: vec![
    ///                 "Albania".to_string(),
    ///                 "Andorra".to_string(),
    ///                 "Austria".to_string(),
    ///                 "Yemen".to_string(),
    ///                 "Zambia".to_string(),
    ///                 "Zimbabwe".to_string(),
    ///             ],
    ///         }),
    ///         Statement::Enum(Enum {
    ///             name: "Category".to_string(),
    ///             variants: vec![
    ///                 "Architecture".to_string(),
    ///                 "Bollard".to_string(),
    ///                 "Chevron".to_string(),
    ///                 "TrafficLight".to_string(),
    ///                 "TrafficSign".to_string(),
    ///                 "UtilityPole".to_string(),
    ///             ],
    ///         }),
    ///     ],
    /// };
    ///                                 
    /// assert_eq!(Ast::parse(&input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let mut input = input.to_string();
        let mut statements = Vec::new();

        while !input.is_empty() {
            let (_, new_input) = spaces(&input)?;
            let (statement, new_input) = Statement::parse(&new_input)?;
            let (_, new_input) = spaces(&new_input)?;
            statements.push(statement);
            input = new_input;
        }

        let (_, input) = spaces(&input)?;

        Ok((Self { statements }, input))
    }
}
