pub use self::field::Field;
use crate::parser::{
    brace_close,
    brace_open,
    capitalized,
    literal,
    spaces,
    ParseResult,
};

/// A field belonging to a model.
pub mod field;

/// A model describes an entity. It has a name and one or more fields.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    /// The name of the model. Used as query return type and inside other
    /// models to reference the model.
    pub name: String,
    /// The fields of the model.
    pub fields: Vec<Field>,
}

impl Model {
    /// Parse a model from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Field,
    ///     Model,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "
    ///
    /// model Foo {
    ///     bar: String
    ///     baz: Int
    ///     qux: [Bar]
    ///     quy: @Bar
    ///     quz: [@Bar]
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Model {
    ///     name: "Foo".to_owned(),
    ///     fields: vec![
    ///         Field::string("bar"),
    ///         Field::int("baz"),
    ///         Field::references("qux", "Bar"),
    ///         Field::owned_reference("quy", "Bar"),
    ///         Field::owned_references("quz", "Bar"),
    ///     ],
    /// };
    ///
    /// assert_eq!(Model::parse(input), Ok((expected, String::new())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "model")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = capitalized(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (field, input) = Field::parse(&input)?;
        let mut fields = vec![field];
        let (_, mut input) = spaces(&input)?;

        while let Ok((field, new_input)) = Field::parse(&input) {
            let (_, new_input) = spaces(&new_input)?;

            fields.push(field);

            input = new_input;
        }

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, fields }, input))
    }
}
