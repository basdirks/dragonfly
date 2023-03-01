use crate::{
    ast::{
        Scalar,
        Type,
    },
    parser::{
        spaces,
        ParseError,
        ParseResult,
    },
};

/// The return type of a query. Must be a model or an array of such a type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ReturnType {
    /// The name of a model.
    Model(String),
    /// An array of a model.
    Array(String),
}

impl ReturnType {
    /// Parse a return type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid return
    /// type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::QueryReturnType,
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(
    ///     QueryReturnType::parse("Foo"),
    ///     Ok((QueryReturnType::Model("Foo".to_owned()), "".to_owned()))
    /// );
    ///
    /// assert_eq!(
    ///     QueryReturnType::parse("[Foo]"),
    ///     Ok((QueryReturnType::Array("Foo".to_owned()), "".to_owned()))
    /// );
    ///
    /// assert_eq!(
    ///     QueryReturnType::parse("String"),
    ///     Err(ParseError::Custom {
    ///         message: "Expected return type, found `String`.".to_owned(),
    ///     })
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (r#type, input) = Type::parse(input)?;
        let (_, input) = spaces(&input)?;

        match r#type {
            Type::Scalar(Scalar::Reference(name)) => {
                Ok((Self::Model(name), input))
            }
            Type::Array(Scalar::Reference(name)) => {
                Ok((Self::Array(name), input))
            }
            _ => {
                Err(ParseError::Custom {
                    message: format!("Expected return type, found `{type}`."),
                })
            }
        }
    }

    /// Return the name of the model that the return type references.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryReturnType;
    ///
    /// assert_eq!(QueryReturnType::Model("Foo".to_owned()).model(), "Foo");
    /// assert_eq!(QueryReturnType::Array("Foo".to_owned()).model(), "Foo");
    /// ```
    #[must_use]
    pub fn model(&self) -> &str {
        match self {
            Self::Model(name) | Self::Array(name) => name,
        }
    }
}
