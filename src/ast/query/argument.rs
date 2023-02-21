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
    ///             name: "name".to_string(),
    ///             r#type: Type::Scalar(Scalar::String),
    ///         },
    ///         "".to_string()
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
