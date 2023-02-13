use std::collections::HashMap;

use crate::parser::{
    brace_close, brace_open, capitalized, literal, spaces, ParseError, ParseResult,
};

use self::field::Field;

pub mod field;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    pub name: String,
    pub fields: HashMap<String, Field>,
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
    /// use std::collections::HashMap;
    ///
    /// let input = "model Foo {
    ///     bar: String
    ///     baz: Int
    ///     qux: [Bar]
    /// }";
    ///
    /// let mut fields = HashMap::new();
    ///
    /// fields.insert(
    ///     "bar".to_string(),
    ///     Field {
    ///         name: "bar".to_string(),
    ///         r#type: Type::One(Primitive::String),
    ///     }
    /// );
    ///
    /// fields.insert(
    ///     "baz".to_string(),
    ///     Field {
    ///         name: "baz".to_string(),
    ///         r#type: Type::One(Primitive::Int),
    ///     }
    /// );
    ///
    /// fields.insert(
    ///     "qux".to_string(),
    ///     Field {
    ///         name: "qux".to_string(),
    ///         r#type: Type::Array(Primitive::Identifier("Bar".to_string())),
    ///     }
    /// );
    ///
    /// let expected = Model {
    ///     name: "Foo".to_string(),
    ///     fields,
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
        let mut fields = HashMap::new();
        let (field, input) = Field::parse(&input)?;

        fields.insert(field.name.clone(), field);

        let (_, mut input) = spaces(&input)?;

        while let Ok((field, new_input)) = Field::parse(&input) {
            let (_, new_input) = spaces(&new_input)?;

            if fields.insert(field.name.clone(), field).is_some() {
                return Err(ParseError::CustomError {
                    message: "duplicate model field".to_string(),
                });
            }

            input = new_input;
        }

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, fields }, input))
    }
}
