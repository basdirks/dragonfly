use crate::parser::{brace_close, brace_open, capitalized, literal, many1, spaces, ParseResult};

use self::field::Field;

pub mod field;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    pub name: String,
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
    /// * If the input is not a valid model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::model::field::Field;
    /// use dragonfly::ast::model::Model;
    /// use dragonfly::ast::r#type::{Primitive, Type};
    ///
    /// let input = "model Foo {
    ///     bar: String
    ///     baz: Int
    ///     qux: [Bar]
    /// }";
    ///
    /// let expected = Model {
    ///     name: "Foo".to_string(),
    ///     fields: vec![
    ///         Field {
    ///             name: "bar".to_string(),
    ///             r#type: Type::One(Primitive::String),
    ///         },
    ///         Field {
    ///             name: "baz".to_string(),
    ///             r#type: Type::One(Primitive::Int),
    ///         },
    ///         Field {
    ///             name: "qux".to_string(),
    ///             r#type: Type::Array(Primitive::Identifier("Bar".to_string())),
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(Model::parse(input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "model")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = capitalized(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (fields, input) = many1(&input, |input| {
            let (_, input) = spaces(input)?;
            let (field, input) = Field::parse(&input)?;
            let (_, input) = spaces(&input)?;

            Ok((field, input))
        })?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, fields }, input))
    }
}
