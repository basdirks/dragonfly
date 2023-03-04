pub use self::{
    component::Component,
    model::{
        Field,
        Model,
    },
    query::{
        Argument as QueryArgument,
        Condition as QueryCondition,
        Operator as QueryOperator,
        Path as QueryPath,
        Query,
        ReturnType as QueryReturnType,
        Schema as QuerySchema,
        SchemaNode as QuerySchemaNode,
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
use crate::{
    map,
    parser::{
        choice,
        map,
        spaces,
        ParseError,
        ParseResult,
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
/// Type errors.
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
    /// Returns `ParseError` if the input does not start with a valid
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
    /// let input = "
    ///
    /// component Foo {
    ///     path: Foo
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Declaration::Component(Component {
    ///     name: "Foo".to_owned(),
    ///     path: "Foo".to_owned().into(),
    /// });
    ///
    /// assert_eq!(Declaration::parse(input), Ok((expected, String::new())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Declaration,
    ///     Enum,
    /// };
    ///
    /// let input = "
    ///
    /// enum Foo {
    ///     Bar
    ///     Baz
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Declaration::Enum(Enum {
    ///     name: "Foo".to_owned(),
    ///     variants: vec!["Bar".to_owned(), "Baz".to_owned()],
    /// });
    ///
    /// assert_eq!(Declaration::parse(input), Ok((expected, String::new())));
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
    /// let input = "
    ///
    /// model Foo {
    ///     foo: String
    ///     bar: [Bar]
    ///     baz: @Bar
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Declaration::Model(Model {
    ///     name: "Foo".to_owned(),
    ///     fields: vec![
    ///         Field::string("foo"),
    ///         Field::references("bar", "Bar"),
    ///         Field::owned_reference("baz", "Bar"),
    ///     ],
    /// });
    ///
    /// assert_eq!(Declaration::parse(input), Ok((expected, String::new())));
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
    pub components: Vec<Component>,
    /// Enum declarations.
    pub enums: Vec<Enum>,
    /// Model declarations.
    pub models: Vec<Model>,
    /// Query declarations.
    pub queries: Vec<Query>,
    /// Route declarations.
    pub routes: Vec<Route>,
}

impl Ast {
    /// Create a new AST.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Ast;
    ///
    /// let ast = Ast::new();
    ///
    /// assert!(ast.components.is_empty());
    /// assert!(ast.enums.is_empty());
    /// assert!(ast.models.is_empty());
    /// assert!(ast.queries.is_empty());
    /// assert!(ast.routes.is_empty());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            components: Vec::new(),
            enums: Vec::new(),
            models: Vec::new(),
            queries: Vec::new(),
            routes: Vec::new(),
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
    /// Returns `ParseError` if the input does not start with a valid AST.
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
    ///     QueryOperator,
    ///     QueryPath,
    ///     QueryReturnType,
    ///     QuerySchema,
    ///     QuerySchemaNode,
    ///     QueryWhere,
    ///     Route,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "
    ///
    /// route / {
    ///   root: Home
    ///   title: Home
    /// }
    ///
    /// component Home {
    ///   path: Home
    /// }
    ///
    /// model Image {
    ///   title: String
    ///   country: Country
    ///   category: [Category]
    ///   dimensions: @Dimensions
    /// }
    ///
    /// model Dimensions {
    ///   width: Int
    ///   height: Int
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
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let mut expected = Ast::new();
    ///
    /// expected.routes = vec![Route {
    ///     path: "/".to_owned(),
    ///     root: "Home".to_owned(),
    ///     title: "Home".to_owned(),
    /// }];
    ///
    /// expected.components = vec![Component {
    ///     name: "Home".to_owned(),
    ///     path: "Home".to_owned().into(),
    /// }];
    ///
    /// expected.models = vec![
    ///     Model {
    ///         name: "Image".to_owned(),
    ///         fields: vec![
    ///             Field::string("title"),
    ///             Field::reference("country", "Country"),
    ///             Field::references("category", "Category"),
    ///             Field::owned_reference("dimensions", "Dimensions"),
    ///         ],
    ///     },
    ///     Model {
    ///         name: "Dimensions".to_owned(),
    ///         fields: vec![Field::int("width"), Field::int("height")],
    ///     },
    ///     Model {
    ///         name: "Country".to_owned(),
    ///         fields: vec![
    ///             Field::string("domain"),
    ///             Field::reference("drivingSide", "DrivingSide"),
    ///             Field::string("flag"),
    ///             Field::reference("name", "CountryName"),
    ///         ],
    ///     },
    /// ];
    ///
    /// expected.enums = vec![
    ///     Enum::new("DrivingSide", &["Left", "Right"]),
    ///     Enum::new(
    ///         "CountryName",
    ///         &[
    ///             "Albania", "Andorra", "Austria", "Yemen", "Zambia", "Zimbabwe",
    ///         ],
    ///     ),
    ///     Enum::new(
    ///         "Category",
    ///         &[
    ///             "Architecture",
    ///             "Bollard",
    ///             "Chevron",
    ///             "TrafficLight",
    ///             "TrafficSign",
    ///             "UtilityPole",
    ///         ],
    ///     ),
    /// ];
    ///
    /// expected.queries = vec![
    ///     Query {
    ///         name: "images".to_owned(),
    ///         r#type: QueryReturnType::array("Image"),
    ///         schema: QuerySchema {
    ///             name: "image".to_owned(),
    ///             nodes: vec![
    ///                 QuerySchemaNode::field("title"),
    ///                 QuerySchemaNode::relation(
    ///                     "country",
    ///                     &[QuerySchemaNode::field("name")],
    ///                 ),
    ///                 QuerySchemaNode::field("category"),
    ///             ],
    ///         },
    ///         r#where: None,
    ///         arguments: vec![],
    ///     },
    ///     Query {
    ///         name: "imagesByCountryName".to_owned(),
    ///         r#type: QueryReturnType::array("Image"),
    ///         schema: QuerySchema::new(
    ///             "image",
    ///             &[
    ///                 QuerySchemaNode::field("title"),
    ///                 QuerySchemaNode::field("category"),
    ///             ],
    ///         ),
    ///         r#where: Some(QueryWhere::new(
    ///             "image",
    ///             &[QueryCondition {
    ///                 path: QueryPath::new(&["country", "name"]),
    ///                 operator: QueryOperator::Equals,
    ///                 argument_name: "name".to_owned(),
    ///             }],
    ///         )),
    ///         arguments: vec![QueryArgument::reference("name", "CountryName")],
    ///     },
    /// ];
    ///
    /// assert_eq!(Ast::parse(&input), Ok((expected, String::new())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Ast,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// asset catalogue {
    ///   path: /catalogue.pdf
    ///   type: pdf
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Ast::parse(&input),
    ///     Err(ParseError::Custom {
    ///         message: "Expected a component, model, query, enum or page."
    ///             .to_owned(),
    ///     })
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let mut input = input.to_owned();
        let mut ast = Self::new();

        while !input.is_empty() {
            let (_, new_input) = spaces(&input)?;

            if let Ok((declaration, new_input)) = Component::parse(&new_input) {
                ast.components.push(declaration);

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Model::parse(&new_input)
            {
                ast.models.push(declaration);

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Query::parse(&new_input)
            {
                ast.queries.push(declaration);

                input = new_input;
            } else if let Ok((declaration, new_input)) = Enum::parse(&new_input)
            {
                ast.enums.push(declaration);

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Route::parse(&new_input)
            {
                ast.routes.push(declaration);

                input = new_input;
            } else {
                return Err(ParseError::Custom {
                    message: "Expected a component, model, query, enum or \
                              page."
                        .to_owned(),
                });
            }

            let (_, new_input) = spaces(&input)?;

            input = new_input;
        }

        let (_, input) = spaces(&input)?;

        Ok((ast, input))
    }
}
