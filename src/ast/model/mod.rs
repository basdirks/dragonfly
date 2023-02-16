use {
    self::field::Field,
    crate::parser::{
        case::capitalized,
        char::{
            brace_close,
            brace_open,
        },
        char_range::spaces,
        literal,
        ParseError,
        ParseResult,
    },
    std::collections::HashMap,
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
    /// Returns a `ParseError` if the input does not start with a valid model.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     dragonfly::ast::{
    ///         model::{
    ///             field::Field,
    ///             Model,
    ///         },
    ///         r#type::{
    ///             Scalar,
    ///             Type,
    ///         },
    ///     },
    ///     std::collections::HashMap,
    /// };
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
    ///         r#type: Type::Scalar(Scalar::String),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "baz".to_string(),
    ///     Field {
    ///         name: "baz".to_string(),
    ///         r#type: Type::Scalar(Scalar::Int),
    ///     },
    /// );
    ///
    /// fields.insert(
    ///     "qux".to_string(),
    ///     Field {
    ///         name: "qux".to_string(),
    ///         r#type: Type::Array(Scalar::Reference("Bar".to_string())),
    ///     },
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
