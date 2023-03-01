pub use self::field::Field;
use {
    crate::parser::{
        brace_close,
        brace_open,
        capitalized,
        literal,
        spaces,
        ParseError,
        ParseResult,
    },
    std::collections::BTreeMap,
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
    pub fields: BTreeMap<String, Field>,
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
    ///         (
    ///             "bar".to_owned(),
    ///             Field {
    ///                 name: "bar".to_owned(),
    ///                 r#type: Type::Scalar(Scalar::String),
    ///             },
    ///         ),
    ///         (
    ///             "baz".to_owned(),
    ///             Field {
    ///                 name: "baz".to_owned(),
    ///                 r#type: Type::Scalar(Scalar::Int),
    ///             },
    ///         ),
    ///         (
    ///             "qux".to_owned(),
    ///             Field {
    ///                 name: "qux".to_owned(),
    ///                 r#type: Type::Array(Scalar::Reference("Bar".to_owned())),
    ///             },
    ///         ),
    ///         (
    ///             "quy".to_owned(),
    ///             Field {
    ///                 name: "quy".to_owned(),
    ///                 r#type: Type::Scalar(Scalar::Owned("Bar".to_owned())),
    ///             },
    ///         ),
    ///         (
    ///             "quz".to_owned(),
    ///             Field {
    ///                 name: "quz".to_owned(),
    ///                 r#type: Type::Array(Scalar::Owned("Bar".to_owned())),
    ///             },
    ///         ),
    ///     ]
    ///     .into_iter()
    ///     .collect(),
    /// };
    ///
    /// assert_eq!(Model::parse(input), Ok((expected, "".to_owned())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::{
    ///         Field,
    ///         Model,
    ///         Scalar,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "
    ///
    /// model Foo {
    ///   bar: String
    ///   bar: Int
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Model::parse(input),
    ///     Err(ParseError::Custom {
    ///         message: "Duplicate model field with name `bar`.".to_owned()
    ///     }),
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "model")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = capitalized(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let mut fields = BTreeMap::new();
        let (field, input) = Field::parse(&input)?;
        let _ = fields.insert(field.name.clone(), field);
        let (_, mut input) = spaces(&input)?;

        while let Ok((field, new_input)) = Field::parse(&input) {
            let (_, new_input) = spaces(&new_input)?;
            let name = field.name.clone();

            if fields.insert(name.clone(), field.clone()).is_some() {
                return Err(ParseError::Custom {
                    message: format!(
                        "Duplicate model field with name `{name}`.",
                    ),
                });
            }

            input = new_input;
        }

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, fields }, input))
    }
}
