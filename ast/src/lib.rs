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
    variant_size_differences
)]

//! The abstract syntax tree of a Dragonfly program.
//!
//! This form is not intended to be used directly to generate code. Instead, it
//! is used to generate a more efficient intermediate representation. This
//! intermediate representation is then used to generate code.

pub use self::{
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
};
use {
    ord_str_map::OrdStrMap,
    parser::{
        spaces,
        ParseError,
        ParseResult,
    },
};

/// An enumerated type.
pub mod r#enum;
/// A data model.
pub mod model;
/// A data query.
pub mod query;
/// Types used inside models and queries.
pub mod r#type;

/// The root of an AST.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Ast<'a> {
    /// Enum declarations.
    pub enums: OrdStrMap<Enum<'a>>,
    /// Model declarations.
    pub models: OrdStrMap<Model<'a>>,
    /// Query declarations.
    pub queries: OrdStrMap<Query<'a>>,
}

impl<'a> Ast<'a> {
    /// Create a new AST.
    #[must_use]
    pub const fn new() -> Self {
        Self {
            enums: OrdStrMap::new(),
            models: OrdStrMap::new(),
            queries: OrdStrMap::new(),
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
    /// use {
    ///     ast::{
    ///         Ast,
    ///         Enum,
    ///         Field,
    ///         Model,
    ///         Query,
    ///         QueryArgument,
    ///         QueryCondition,
    ///         QueryOperator,
    ///         QueryPath,
    ///         QueryReturnType,
    ///         QuerySchema,
    ///         QuerySchemaNode,
    ///         QueryWhere,
    ///         Scalar,
    ///         Type,
    ///     },
    ///     ord_str_map::OrdStrMap,
    ///     token_set::TokenSet,
    /// };
    ///
    /// let input = "
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
    /// expected.models = OrdStrMap::from_iter([
    ///     (
    ///         "Image",
    ///         Model {
    ///             name: "Image".into(),
    ///             fields: OrdStrMap::from_iter([
    ///                 (
    ///                     "title",
    ///                     Field {
    ///                         name: "title".into(),
    ///                         r#type: Type::Scalar(Scalar::String),
    ///                     },
    ///                 ),
    ///                 (
    ///                     "country",
    ///                     Field {
    ///                         name: "country".into(),
    ///                         r#type: Type::Scalar(Scalar::Reference(
    ///                             "Country".into(),
    ///                         )),
    ///                     },
    ///                 ),
    ///                 (
    ///                     "category",
    ///                     Field {
    ///                         name: "category".into(),
    ///                         r#type: Type::Array(Scalar::Reference(
    ///                             "Category".into(),
    ///                         )),
    ///                     },
    ///                 ),
    ///                 (
    ///                     "dimensions",
    ///                     Field {
    ///                         name: "dimensions".into(),
    ///                         r#type: Type::Scalar(Scalar::Owned(
    ///                             "Dimensions".into(),
    ///                         )),
    ///                     },
    ///                 ),
    ///             ]),
    ///         },
    ///     ),
    ///     (
    ///         "Dimensions",
    ///         Model {
    ///             name: "Dimensions".into(),
    ///             fields: OrdStrMap::from_iter([
    ///                 (
    ///                     "width",
    ///                     Field {
    ///                         name: "width".into(),
    ///                         r#type: Type::Scalar(Scalar::Int),
    ///                     },
    ///                 ),
    ///                 (
    ///                     "height",
    ///                     Field {
    ///                         name: "height".into(),
    ///                         r#type: Type::Scalar(Scalar::Int),
    ///                     },
    ///                 ),
    ///             ]),
    ///         },
    ///     ),
    ///     (
    ///         "Country",
    ///         Model {
    ///             name: "Country".into(),
    ///             fields: OrdStrMap::from_iter([
    ///                 (
    ///                     "domain",
    ///                     Field {
    ///                         name: "domain".into(),
    ///                         r#type: Type::Scalar(Scalar::String),
    ///                     },
    ///                 ),
    ///                 (
    ///                     "drivingSide",
    ///                     Field {
    ///                         name: "drivingSide".into(),
    ///                         r#type: Type::Scalar(Scalar::Reference(
    ///                             "DrivingSide".into(),
    ///                         )),
    ///                     },
    ///                 ),
    ///                 (
    ///                     "flag",
    ///                     Field {
    ///                         name: "flag".into(),
    ///                         r#type: Type::Scalar(Scalar::String),
    ///                     },
    ///                 ),
    ///                 (
    ///                     "name",
    ///                     Field {
    ///                         name: "name".into(),
    ///                         r#type: Type::Scalar(Scalar::Reference(
    ///                             "CountryName".into(),
    ///                         )),
    ///                     },
    ///                 ),
    ///             ]),
    ///         },
    ///     ),
    /// ]);
    ///
    /// expected.enums = OrdStrMap::from_iter([
    ///     (
    ///         "DrivingSide",
    ///         Enum {
    ///             name: "DrivingSide".into(),
    ///             values: TokenSet::from_iter(["Left", "Right"]),
    ///         },
    ///     ),
    ///     (
    ///         "CountryName",
    ///         Enum {
    ///             name: "CountryName".into(),
    ///             values: TokenSet::from_iter([
    ///                 "Albania", "Andorra", "Austria", "Yemen", "Zambia",
    ///                 "Zimbabwe",
    ///             ]),
    ///         },
    ///     ),
    ///     (
    ///         "Category",
    ///         Enum {
    ///             name: "Category".into(),
    ///             values: TokenSet::from_iter([
    ///                 "Architecture",
    ///                 "Bollard",
    ///                 "Chevron",
    ///                 "TrafficLight",
    ///                 "TrafficSign",
    ///                 "UtilityPole",
    ///             ]),
    ///         },
    ///     ),
    /// ]);
    ///
    /// expected.queries = OrdStrMap::from_iter([
    ///     (
    ///         "images",
    ///         Query {
    ///             name: "images".into(),
    ///             r#type: QueryReturnType::Array("Image".into()),
    ///             schema: QuerySchema {
    ///                 name: "image".into(),
    ///                 nodes: vec![
    ///                     QuerySchemaNode::Field {
    ///                         name: "title".into(),
    ///                     },
    ///                     QuerySchemaNode::Relation {
    ///                         name: "country".into(),
    ///                         nodes: vec![QuerySchemaNode::Field {
    ///                             name: "name".into(),
    ///                         }],
    ///                     },
    ///                     QuerySchemaNode::Field {
    ///                         name: "category".into(),
    ///                     },
    ///                 ],
    ///             },
    ///             r#where: None,
    ///             arguments: OrdStrMap::new(),
    ///         },
    ///     ),
    ///     (
    ///         "imagesByCountryName",
    ///         Query {
    ///             name: "imagesByCountryName".into(),
    ///             r#type: QueryReturnType::Array("Image".into()),
    ///             schema: QuerySchema {
    ///                 name: "image".into(),
    ///                 nodes: vec![
    ///                     QuerySchemaNode::Field {
    ///                         name: "title".into(),
    ///                     },
    ///                     QuerySchemaNode::Field {
    ///                         name: "category".into(),
    ///                     },
    ///                 ],
    ///             },
    ///             r#where: Some(QueryWhere {
    ///                 name: "image".into(),
    ///                 conditions: vec![QueryCondition {
    ///                     path: QueryPath::from_iter(["country", "name"]),
    ///                     operator: QueryOperator::Equals,
    ///                     argument_name: "name".into(),
    ///                 }],
    ///             }),
    ///             arguments: OrdStrMap::from_iter([(
    ///                 "name",
    ///                 QueryArgument {
    ///                     name: "name".into(),
    ///                     r#type: Type::Scalar(Scalar::Reference(
    ///                         "CountryName".into(),
    ///                     )),
    ///                 },
    ///             )]),
    ///         },
    ///     ),
    /// ]);
    ///
    /// assert_eq!(Ast::parse(&input), Ok((expected, String::new())));
    /// ```
    ///
    /// ```rust
    /// use {
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
    ///         message: "Expected an enum, model, or query.".into(),
    ///     })
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let mut input = input.to_owned();
        let mut ast = Self::new();

        while !input.is_empty() {
            let (_, new_input) = spaces(&input)?;

            if let Ok((declaration, new_input)) = Model::parse(&new_input) {
                if ast
                    .models
                    .insert(declaration.name.clone(), declaration.clone())
                    .is_some()
                {
                    return Err(ParseError::Custom {
                        message: format!(
                            "Duplicate model name `{}`",
                            declaration.name
                        ),
                    });
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) =
                Query::parse(&new_input)
            {
                if ast
                    .queries
                    .insert(declaration.name.clone(), declaration.clone())
                    .is_some()
                {
                    return Err(ParseError::custom(format!(
                        "Duplicate query name `{}`",
                        declaration.name
                    )));
                }

                input = new_input;
            } else if let Ok((declaration, new_input)) = Enum::parse(&new_input)
            {
                if ast
                    .enums
                    .insert(declaration.name.clone(), declaration.clone())
                    .is_some()
                {
                    return Err(ParseError::Custom {
                        message: format!(
                            "Duplicate enum name `{}`",
                            declaration.name
                        ),
                    });
                }

                input = new_input;
            } else {
                return Err(ParseError::Custom {
                    message: "Expected an enum, model, or query.".to_owned(),
                });
            }

            let (_, new_input) = spaces(&input)?;

            input = new_input;
        }

        let (_, input) = spaces(&input)?;

        Ok((ast, input))
    }
}

impl Default for Ast<'_> {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_duplicate_model_name() {
        let input = "

model Image {
  title: String
}

model Image {
  title: String
}

        "
        .trim();

        assert_eq!(
            Ast::parse(input),
            Err(ParseError::Custom {
                message: "Duplicate model name `Image`".into(),
            })
        );
    }

    #[test]
    fn test_parse_duplicate_query_name() {
        let input = "

query images: [Image] {
  image {
    title
  }
}

query images: [Image] {
  image {
    title
  }
}

        "
        .trim();

        assert_eq!(
            Ast::parse(input),
            Err(ParseError::Custom {
                message: "Duplicate query name `images`".into(),
            })
        );
    }

    #[test]
    fn test_parse_duplicate_enum_name() {
        let input = "

enum DrivingSide {
  Left
  Right
}

enum DrivingSide {
  Left
  Right
}

        "
        .trim();

        assert_eq!(
            Ast::parse(input),
            Err(ParseError::Custom {
                message: "Duplicate enum name `DrivingSide`".into(),
            })
        );
    }

    #[test]
    fn test_parse_empty_input() {
        let input = "";

        assert_eq!(Ast::parse(input), Ok((Ast::new(), String::new())));
    }

    #[test]
    fn test_default() {
        assert_eq!(Ast::default(), Ast::new());
    }
}
