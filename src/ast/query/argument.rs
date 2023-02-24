use {
    super::Query,
    crate::{
        ast::{
            Scalar,
            Type,
        },
        parser::{
            colon,
            spaces,
            ParseResult,
        },
    },
    std::fmt::Display,
};

/// A query argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argument {
    /// The name of the argument. Used inside the conditions of
    /// the where clause.
    pub name: String,
    /// The type of the argument.
    pub r#type: Type,
}

impl Display for Argument {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self { name, r#type } = self;

        write!(f, "${name}: {type}")
    }
}

impl Argument {
    /// Parse an argument from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid
    /// argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     QueryArgument::parse("$name: String"),
    ///     Ok((
    ///         QueryArgument {
    ///             name: "name".to_owned(),
    ///             r#type: Type::Scalar(Scalar::String),
    ///         },
    ///         "".to_owned()
    ///     ))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = Query::parse_reference(input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((Self { name, r#type }, input))
    }

    /// Return the scalar type of the argument.
    #[must_use]
    pub const fn scalar(&self) -> &Scalar {
        self.r#type.scalar()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        let argument = Argument {
            name: "name".to_owned(),
            r#type: Type::Scalar(Scalar::String),
        };

        assert_eq!(argument.to_string(), "$name: String");

        let argument = Argument {
            name: "name".to_owned(),
            r#type: Type::Array(Scalar::String),
        };

        assert_eq!(argument.to_string(), "$name: [String]");
    }
}
