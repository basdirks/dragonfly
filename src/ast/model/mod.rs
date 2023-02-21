pub use self::field::Field;
use {
    super::{
        Scalar,
        TypeError,
    },
    crate::parser::{
        brace_close,
        brace_open,
        capitalized,
        literal,
        spaces,
        ParseError,
        ParseResult,
    },
    std::collections::{
        HashMap,
        HashSet,
    },
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
    /// let input = "model Foo {
    ///     bar: String
    ///     baz: Int
    ///     qux: [Bar]
    /// }";
    ///
    /// let expected = Model {
    ///     name: "Foo".to_string(),
    ///     fields: vec![
    ///         (
    ///             "bar".to_string(),
    ///             Field {
    ///                 name: "bar".to_string(),
    ///                 r#type: Type::Scalar(Scalar::String),
    ///             },
    ///         ),
    ///         (
    ///             "baz".to_string(),
    ///             Field {
    ///                 name: "baz".to_string(),
    ///                 r#type: Type::Scalar(Scalar::Int),
    ///             },
    ///         ),
    ///         (
    ///             "qux".to_string(),
    ///             Field {
    ///                 name: "qux".to_string(),
    ///                 r#type: Type::Array(Scalar::Reference("Bar".to_string())),
    ///             },
    ///         ),
    ///     ]
    ///     .into_iter()
    ///     .collect(),
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
        let _ = fields.insert(field.name.clone(), field);
        let (_, mut input) = spaces(&input)?;

        while let Ok((field, new_input)) = Field::parse(&input) {
            let (_, new_input) = spaces(&new_input)?;

            if fields.insert(field.name.clone(), field).is_some() {
                return Err(ParseError::CustomError {
                    message: "duplicate model field".to_string(),
                    input: input.to_string(),
                });
            }

            input = new_input;
        }

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, fields }, input))
    }

    /// Check whether the type of each field is a primitive, a reference to
    /// an existing enum or model, or an array of such a type.
    ///
    /// # Arguments
    ///
    /// * `model_names` - The names of all models.
    /// * `enum_names` - The names of all enums.
    ///
    /// # Errors
    ///
    /// Returns `TypeError::UnknownModelFieldType` if the type of a field is
    /// not a primitive, a reference to an existing enum or model, or an
    /// array of such a type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     dragonfly::ast::{
    ///         Field,
    ///         Model,
    ///         Scalar,
    ///         Type,
    ///         TypeError,
    ///     },
    ///     std::collections::HashSet,
    /// };
    ///
    /// let model = Model {
    ///     name: "Foo".to_string(),
    ///     fields: vec![
    ///         (
    ///             "bar".to_string(),
    ///             Field {
    ///                 name: "bar".to_string(),
    ///                 r#type: Type::Scalar(Scalar::String),
    ///             },
    ///         ),
    ///         (
    ///             "baz".to_string(),
    ///             Field {
    ///                 name: "baz".to_string(),
    ///                 r#type: Type::Scalar(Scalar::Int),
    ///             },
    ///         ),
    ///         (
    ///             "qux".to_string(),
    ///             Field {
    ///                 name: "qux".to_string(),
    ///                 r#type: Type::Array(Scalar::Reference("Bar".to_string())),
    ///             },
    ///         ),
    ///     ]
    ///     .into_iter()
    ///     .collect(),
    /// };
    ///
    /// assert!(model
    ///     .check_field_types(
    ///         &["Foo".to_string(), "Bar".to_string()]
    ///             .iter()
    ///             .cloned()
    ///             .collect(),
    ///         &HashSet::new()
    ///     )
    ///     .is_ok());
    ///
    /// assert_eq!(
    ///     model.check_field_types(
    ///         &["Foo".to_string()].iter().cloned().collect(),
    ///         &HashSet::new()
    ///     ),
    ///     Err(TypeError::UnknownModelFieldType {
    ///         model_name: "Foo".to_string(),
    ///         field: Field {
    ///             name: "qux".to_string(),
    ///             r#type: Type::Array(Scalar::Reference("Bar".to_string())),
    ///         },
    ///     })
    /// );
    /// ```
    pub fn check_field_types(
        &self,
        model_names: &HashSet<String>,
        enum_names: &HashSet<String>,
    ) -> Result<(), TypeError> {
        for field @ Field { r#type, .. } in self.fields.values() {
            if let Scalar::Reference(reference) = r#type.scalar() {
                if !model_names.contains(reference)
                    && !enum_names.contains(reference)
                {
                    return Err(TypeError::UnknownModelFieldType {
                        model_name: self.name.clone(),
                        field: field.clone(),
                    });
                }
            }
        }

        Ok(())
    }
}
