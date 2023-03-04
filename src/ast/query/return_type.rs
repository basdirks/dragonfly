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
    /// Create a new model return type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    #[must_use]
    pub fn model(name: &str) -> Self {
        Self::Model(name.to_owned())
    }

    /// Create a new array return type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    #[must_use]
    pub fn array(name: &str) -> Self {
        Self::Array(name.to_owned())
    }

    /// Return the name of the model.
    #[must_use]
    pub fn name(&self) -> &str {
        match self {
            Self::Array(name) | Self::Model(name) => name,
        }
    }

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
    ///     Ok((QueryReturnType::model("Foo"), String::new()))
    /// );
    ///
    /// assert_eq!(
    ///     QueryReturnType::parse("[Foo]"),
    ///     Ok((QueryReturnType::array("Foo"), String::new()))
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_model() {
        assert_eq!(
            ReturnType::model("Foo"),
            ReturnType::Model("Foo".to_owned())
        );
    }

    #[test]
    fn test_array() {
        assert_eq!(
            ReturnType::array("Foo"),
            ReturnType::Array("Foo".to_owned())
        );
    }
}
