pub use self::{
    component::Component,
    model::{
        Field,
        Model,
    },
    query::Query,
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
    std::collections::BTreeMap,
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
    ///     path: "Foo".to_owned(),
    /// });
    ///
    /// assert_eq!(Declaration::parse(input), Ok((expected, "".to_owned())));
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
    /// assert_eq!(Declaration::parse(input), Ok((expected, "".to_owned())));
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
    ///         (
    ///             "foo".to_owned(),
    ///             Field {
    ///                 name: "foo".to_owned(),
    ///                 r#type: Type::Scalar(Scalar::String),
    ///             },
    ///         ),
    ///         (
    ///             "bar".to_owned(),
    ///             Field {
    ///                 name: "bar".to_owned(),
    ///                 r#type: Type::Array(Scalar::Reference("Bar".to_owned())),
    ///             },
    ///         ),
    ///         (
    ///             "baz".to_owned(),
    ///             Field {
    ///                 name: "baz".to_owned(),
    ///                 r#type: Type::Scalar(Scalar::Owned("Bar".to_owned())),
    ///             },
    ///         ),
    ///     ]
    ///     .into_iter()
    ///     .collect(),
    /// });
    ///
    /// assert_eq!(Declaration::parse(input), Ok((expected, "".to_owned())));
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
    pub components: BTreeMap<String, Component>,
    /// Enum declarations.
    pub enums: BTreeMap<String, Enum>,
    /// Model declarations.
    pub models: BTreeMap<String, Model>,
    /// Query declarations.
    pub queries: BTreeMap<String, Query>,
    /// Route declarations.
    pub routes: BTreeMap<String, Route>,
}

impl Ast {
    /// Create a new AST.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            components: BTreeMap::new(),
            enums: BTreeMap::new(),
            models: BTreeMap::new(),
            queries: BTreeMap::new(),
            routes: BTreeMap::new(),
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
    ///     FieldPath,
    ///     Model,
    ///     Query,
    ///     QueryArgument,
    ///     QueryCondition,
    ///     QueryOperator,
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
    /// expected.routes.insert(
    ///     "/".to_owned(),
    ///     Route {
    ///         path: "/".to_owned(),
    ///         root: "Home".to_owned(),
    ///         title: "Home".to_owned(),
    ///     },
    /// );
    ///
    /// expected.components.insert(
    ///     "Home".to_owned(),
    ///     Component {
    ///         name: "Home".to_owned(),
    ///         path: "Home".to_owned(),
    ///     },
    /// );
    ///
    /// expected.models.insert(
    ///     "Image".to_owned(),
    ///     Model {
    ///         name: "Image".to_owned(),
    ///         fields: vec![
    ///             (
    ///                 "title".to_owned(),
    ///                 Field {
    ///                     name: "title".to_owned(),
    ///                     r#type: Type::Scalar(Scalar::String),
    ///                 },
    ///             ),
    ///             (
    ///                 "country".to_owned(),
    ///                 Field {
    ///                     name: "country".to_owned(),
    ///                     r#type: Type::Scalar(Scalar::Reference(
    ///                         "Country".to_owned(),
    ///                     )),
    ///                 },
    ///             ),
    ///             (
    ///                 "category".to_owned(),
    ///                 Field {
    ///                     name: "category".to_owned(),
    ///                     r#type: Type::Array(Scalar::Reference(
    ///                         "Category".to_owned(),
    ///                     )),
    ///                 },
    ///             ),
    ///             (
    ///                 "dimensions".to_owned(),
    ///                 Field {
    ///                     name: "dimensions".to_owned(),
    ///                     r#type: Type::Scalar(Scalar::Owned(
    ///                         "Dimensions".to_owned(),
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
    ///     "Dimensions".to_owned(),
    ///     Model {
    ///         name: "Dimensions".to_owned(),
    ///         fields: vec![
    ///             (
    ///                 "width".to_owned(),
    ///                 Field {
    ///                     name: "width".to_owned(),
    ///                     r#type: Type::Scalar(Scalar::Int),
    ///                 },
    ///             ),
    ///             (
    ///                 "height".to_owned(),
    ///                 Field {
    ///                     name: "height".to_owned(),
    ///                     r#type: Type::Scalar(Scalar::Int),
    ///                 },
    ///             ),
    ///         ]
    ///         .into_iter()
    ///         .collect(),
    ///     },
    /// );
    ///
    /// expected.models.insert(
    ///     "Country".to_owned(),
    ///     Model {
    ///         name: "Country".to_owned(),
    ///         fields: vec![
    ///             (
    ///                 "domain".to_owned(),
    ///                 Field {
    ///                     name: "domain".to_owned(),
    ///                     r#type: Type::Scalar(Scalar::String),
    ///                 },
    ///             ),
    ///             (
    ///                 "drivingSide".to_owned(),
    ///                 Field {
    ///                     name: "drivingSide".to_owned(),
    ///                     r#type: Type::Scalar(Scalar::Reference(
    ///                         "DrivingSide".to_owned(),
    ///                     )),
    ///                 },
    ///             ),
    ///             (
    ///                 "flag".to_owned(),
    ///                 Field {
    ///                     name: "flag".to_owned(),
    ///                     r#type: Type::Scalar(Scalar::String),
    ///                 },
    ///             ),
    ///             (
    ///                 "name".to_owned(),
    ///                 Field {
    ///                     name: "name".to_owned(),
    ///                     r#type: Type::Scalar(Scalar::Reference(
    ///                         "CountryName".to_owned(),
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
    ///     "DrivingSide".to_owned(),
    ///     Enum {
    ///         name: "DrivingSide".to_owned(),
    ///         variants: vec!["Left".to_owned(), "Right".to_owned()],
    ///     },
    /// );
    ///
    /// expected.enums.insert(
    ///     "CountryName".to_owned(),
    ///     Enum {
    ///         name: "CountryName".to_owned(),
    ///         variants: vec![
    ///             "Albania".to_owned(),
    ///             "Andorra".to_owned(),
    ///             "Austria".to_owned(),
    ///             "Yemen".to_owned(),
    ///             "Zambia".to_owned(),
    ///             "Zimbabwe".to_owned(),
    ///         ],
    ///     },
    /// );
    ///
    /// expected.enums.insert(
    ///     "Category".to_owned(),
    ///     Enum {
    ///         name: "Category".to_owned(),
    ///         variants: vec![
    ///             "Architecture".to_owned(),
    ///             "Bollard".to_owned(),
    ///             "Chevron".to_owned(),
    ///             "TrafficLight".to_owned(),
    ///             "TrafficSign".to_owned(),
    ///             "UtilityPole".to_owned(),
    ///         ],
    ///     },
    /// );
    ///
    /// expected.queries.insert(
    ///     "images".to_owned(),
    ///     Query {
    ///         name: "images".to_owned(),
    ///         r#type: QueryReturnType::Array("Image".to_owned()),
    ///         schema: QuerySchema {
    ///             name: "image".to_owned(),
    ///             nodes: vec![
    ///                 QuerySchemaNode::Field("title".to_owned()),
    ///                 QuerySchemaNode::Model {
    ///                     name: "country".to_owned(),
    ///                     nodes: vec![QuerySchemaNode::Field("name".to_owned())],
    ///                 },
    ///                 QuerySchemaNode::Field("category".to_owned()),
    ///             ],
    ///         },
    ///         r#where: None,
    ///         arguments: vec![],
    ///     },
    /// );
    ///
    /// expected.queries.insert(
    ///     "imagesByCountryName".to_owned(),
    ///     Query {
    ///         name: "imagesByCountryName".to_owned(),
    ///         r#type: QueryReturnType::Array("Image".to_owned()),
    ///         schema: QuerySchema {
    ///             name: "image".to_owned(),
    ///             nodes: vec![
    ///                 QuerySchemaNode::Field("title".to_owned()),
    ///                 QuerySchemaNode::Field("category".to_owned()),
    ///             ],
    ///         },
    ///         r#where: Some(QueryWhere {
    ///             name: "image".to_owned(),
    ///             conditions: vec![QueryCondition {
    ///                 path: FieldPath::new(&["country", "name"]),
    ///                 operator: QueryOperator::Equals,
    ///                 argument: "name".to_owned(),
    ///             }],
    ///         }),
    ///         arguments: vec![QueryArgument {
    ///             name: "name".to_owned(),
    ///             r#type: Type::Scalar(Scalar::Reference(
    ///                 "CountryName".to_owned(),
    ///             )),
    ///         }],
    ///     },
    /// );
    ///
    /// assert_eq!(Ast::parse(&input), Ok((expected, "".to_owned())));
    /// ```
    ///
    /// Component names must be unique:
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Ast,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// component Home {
    ///   path: Home
    /// }
    ///
    /// component Home {
    ///   path: Index
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Ast::parse(&input),
    ///     Err(ParseError::Custom {
    ///         message: "Component `Home` already defined.".to_owned(),
    ///     })
    /// );
    /// ```
    ///
    /// Model names must be unique:
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Ast,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// model Image {
    ///   title: String
    ///   country: Country
    ///   category: Category
    /// }
    ///
    /// model Image {
    ///   file: String
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Ast::parse(&input),
    ///     Err(ParseError::Custom {
    ///         message: "Model `Image` already defined.".to_owned(),
    ///     })
    /// );
    /// ```
    ///
    /// Component names must be unique:
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Ast,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// component Home {
    ///   path: Home
    /// }
    ///
    /// component Home {
    ///   path: Index
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Ast::parse(&input),
    ///     Err(ParseError::Custom {
    ///         message: "Component `Home` already defined.".to_owned(),
    ///     })
    /// );
    /// ```
    ///
    /// Enum names must be unique:
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Ast,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// enum Category {
    ///   Architecture
    ///   Bollard
    ///   Chevron
    /// }
    ///
    /// enum Category {
    ///   TrafficLight
    ///   TrafficSign
    ///   UtilityPole
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Ast::parse(&input),
    ///     Err(ParseError::Custom {
    ///         message: "Enum `Category` already defined.".to_owned(),
    ///     })
    /// );
    /// ```
    ///
    /// Query names must be unique:
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Ast,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// query images: [Image] {
    ///   image {
    ///     title
    ///   }
    /// }
    ///
    /// query images: [Image] {
    ///   image {
    ///     category
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Ast::parse(&input),
    ///     Err(ParseError::Custom {
    ///         message: "Query `images` already defined.".to_owned(),
    ///     })
    /// );
    /// ```
    ///
    /// Route paths must be unique:
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Ast,
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// route / {
    ///   root: Home
    ///   title: Home
    /// }
    ///
    /// route / {
    ///   root: Index
    ///   title: Index
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Ast::parse(&input),
    ///     Err(ParseError::Custom {
    ///         message: "Route with path `/` already defined.".to_owned(),
    ///     })
    /// );
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
                let name = declaration.name.clone();

                if ast.components.insert(name.clone(), declaration).is_some() {
                    return Err(ParseError::Custom {
                        message: format!("Component `{name}` already defined."),
                    });
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Model::parse(&new_input)
            {
                let name = declaration.name.clone();

                if ast.models.insert(name.clone(), declaration).is_some() {
                    return Err(ParseError::Custom {
                        message: format!("Model `{name}` already defined."),
                    });
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Query::parse(&new_input)
            {
                let name = declaration.name.clone();

                if ast.queries.insert(name.clone(), declaration).is_some() {
                    return Err(ParseError::Custom {
                        message: format!("Query `{name}` already defined."),
                    });
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) = Enum::parse(&new_input)
            {
                let name = declaration.name.clone();

                if ast.enums.insert(name.clone(), declaration).is_some() {
                    return Err(ParseError::Custom {
                        message: format!("Enum `{name}` already defined."),
                    });
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Route::parse(&new_input)
            {
                let path = declaration.path.clone();

                if ast.routes.insert(path.clone(), declaration).is_some() {
                    return Err(ParseError::Custom {
                        message: format!(
                            "Route with path `{path}` already defined."
                        ),
                    });
                }

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

    /// Check the AST for type errors.
    ///
    /// # Errors
    ///
    /// * Returns `TypeError::EmptyQuerySchema` if the schema of any query does
    ///   not contain any fields.
    /// * Returns `TypeError::UnusedQueryArgument` if any query argument is not
    ///   used in the query's `where` clause.
    /// * Returns `TypeError::IncompatibleQueryRootNodes` if the root nodes of
    ///   any query's schema and `where` clause do not match.
    /// * Returns `TypeError::UnknownQueryConditionReference` if any query
    ///   selector references an argument that does not exist.
    /// * Returns `TypeError::InvalidQueryArgumentType` if the type of any query
    ///   is not a primitive or a reference to an enum.
    /// * Returns `TypeError::InvalidQueryReturnType` if the return type of any
    ///   query is not a reference to a known model.
    /// * Returns `TypeError::UnknownRouteRoot` if the root of any route is not
    ///   a reference to a known component.
    /// * Returns `TypeError::UnknownFieldType` if the type of any model field
    ///   is not a primitive, a reference to a known enum or model, or an array
    ///   of any such a type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Ast,
    ///     Field,
    ///     Scalar,
    ///     Type,
    ///     TypeError,
    /// };
    ///
    /// let input = "
    ///
    /// model Post {
    ///   title: String
    ///   tags: [Tag]
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let ast = Ast::parse(&input).unwrap().0;
    ///
    /// assert_eq!(
    ///     ast.check(),
    ///     Err(TypeError::UnknownFieldType {
    ///         model_name: "Post".to_owned(),
    ///         field: Field {
    ///             name: "tags".to_owned(),
    ///             r#type: Type::Array(Scalar::Reference("Tag".to_owned()))
    ///         },
    ///     })
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Ast,
    ///     TypeError,
    /// };
    ///
    /// let input = "
    ///
    /// model Post {
    ///   title: String
    /// }
    ///
    /// query posts($title: String): [Post] {
    ///   post {
    ///     title
    ///   }
    ///   where {
    ///     posts {
    ///       title {
    ///         equals: $title
    ///       }
    ///     }
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Ast::parse(&input).unwrap().0.check(),
    ///     Err(TypeError::IncompatibleQueryRootNodes {
    ///         query_name: "posts".to_owned(),
    ///         where_root: "posts".to_owned(),
    ///         schema_root: "post".to_owned(),
    ///     })
    /// );
    /// ```
    pub fn check(&self) -> Result<(), TypeError> {
        self.check_entities()?;

        Ok(())
    }

    /// Check for errors in individual entities.
    ///
    /// # Errors
    ///
    /// * Returns `TypeError::EmptyQuerySchema` if the schema of any query does
    ///   not contain any fields.
    /// * Returns `TypeError::UnusedQueryArgument` if any query argument is not
    ///   used in the query's `where` clause.
    /// * Returns `TypeError::IncompatibleQueryRootNodes` if the root nodes of
    ///   any query's schema and `where` clause do not match.
    /// * Returns `TypeError::UnknownQueryConditionReference` if any query
    ///   selector references an argument that does not exist.
    pub fn check_entities(&self) -> Result<(), TypeError> {
        // Many of these checks could be combined into a single pass over the
        // AST, but doing them separately is easier to understand.

        for query in self.queries.values() {
            query.check_unused_arguments()?;
            query.check_empty_schema()?;
        }

        Ok(())
    }
}
