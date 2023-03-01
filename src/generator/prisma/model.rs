use {
    super::{
        value::Function,
        BlockAttribute,
        FieldAttribute,
        Value,
    },
    crate::generator::printer::{
        indent,
        newline_separated,
        space_separated,
        Print,
    },
    std::fmt::{
        Display,
        Write,
    },
};

/// A field type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum FieldType {
    /// A name.
    Name(String),
    /// A function.
    Function(Function),
}

impl FieldType {
    /// Create a new `Name` field type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::FieldType;
    ///
    /// let field_type = FieldType::new("foo");
    ///
    /// assert_eq!(field_type, FieldType::Name("foo".to_owned()));
    /// ```
    #[must_use]
    pub fn name(name: &str) -> Self {
        Self::Name(name.to_owned())
    }

    /// Create a new `Function` field type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the function.
    /// * `parameters` - The parameters of the function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::FieldType;
    ///
    /// let field_type = FieldType::function("foo", &[]);
    ///
    /// assert_eq!(field_type, FieldType::Function(Function::new("foo", &[])));
    ///
    /// let field_type = FieldType::function(
    ///     "foo",
    ///     &[Value::Boolean(true), Value::String("bar".to_owned())],
    /// );
    ///
    /// assert_eq!(
    ///     field_type,
    ///     FieldType::Function(Function::new(
    ///         "foo",
    ///         &[Value::Boolean(true), Value::String("bar".to_owned())]
    ///     ))
    /// );
    /// ```
    #[must_use]
    pub fn function(
        name: &str,
        parameters: &[Value],
    ) -> Self {
        Self::Function(Function::new(name, parameters))
    }
}

impl Display for FieldType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Name(name) => write!(f, "{name}"),
            Self::Function(function) => write!(f, "{function}"),
        }
    }
}

/// A field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Field {
    /// The name of the field. Must adhere to the following regular expression:
    /// [A-Za-z][A-Za-z0-9_]*. Must start with an alphabetic character. Usually
    /// camel case.
    pub name: String,
    /// The type of the field.
    pub r#type: FieldType,
    /// Is the field required?
    pub required: bool,
    /// Is the field an array?
    pub array: bool,
    /// Field attributes.
    pub attributes: Vec<FieldAttribute>,
}

impl Field {
    /// Create a new field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `r#type` - The type of the field.
    /// * `required` - Is the field required?
    /// * `array` - Is the field an array?
    /// * `attributes` - Field attributes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::model::{
    ///     Field,
    ///     FieldType,
    /// };
    ///
    /// let field = Field::new("foo", FieldType::name("Int"), true, false, &[]);
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.r#type, FieldType::name("Int"));
    /// assert_eq!(field.required, true);
    /// assert_eq!(field.array, false);
    /// assert_eq!(field.attributes, vec![]);
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        r#type: FieldType,
        required: bool,
        array: bool,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self {
            name: name.to_owned(),
            r#type,
            required,
            array,
            attributes: attributes.to_owned(),
        }
    }

    /// Create a new boolean field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `attributes` - Field attributes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::model::{
    ///     Field,
    ///     FieldType,
    /// };
    ///
    /// let field = Field::boolean("foo", &[]);
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.r#type, FieldType::name("Boolean"));
    /// assert_eq!(field.required, true);
    /// assert_eq!(field.array, false);
    /// assert_eq!(field.attributes, vec![]);
    /// ```
    #[must_use]
    pub fn boolean(
        name: &str,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self::new(name, FieldType::name("Boolean"), true, false, attributes)
    }

    /// Create a new integer field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `attributes` - Field attributes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::model::{
    ///     Field,
    ///     FieldType,
    /// };
    ///
    /// let field = Field::int("foo", &[]);
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.r#type, FieldType::name("Int"));
    /// assert_eq!(field.required, true);
    /// assert_eq!(field.array, false);
    /// assert_eq!(field.attributes, vec![]);
    /// ```
    #[must_use]
    pub fn int(
        name: &str,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self::new(name, FieldType::name("Int"), true, false, attributes)
    }

    /// Create a new date time field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `attributes` - Field attributes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::model::{
    ///     Field,
    ///     FieldType,
    /// };
    ///
    /// let field = Field::datetime("foo", &[]);
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.r#type, FieldType::name("DateTime"));
    /// assert_eq!(field.required, true);
    /// assert_eq!(field.array, false);
    /// assert_eq!(field.attributes, vec![]);
    /// ```
    #[must_use]
    pub fn datetime(
        name: &str,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self::new(name, FieldType::name("DateTime"), true, false, attributes)
    }

    /// Create a new string field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `attributes` - Field attributes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::model::{
    ///     Field,
    ///     FieldType,
    /// };
    ///
    /// let field = Field::string("foo", &[]);
    ///
    /// assert_eq!(field.name, "foo");
    /// assert_eq!(field.r#type, FieldType::name("String"));
    /// assert_eq!(field.required, true);
    /// assert_eq!(field.array, false);
    /// assert_eq!(field.attributes, vec![]);
    /// ```
    #[must_use]
    pub fn string(
        name: &str,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self::new(name, FieldType::name("String"), true, false, attributes)
    }

    /// Standard `id` field.
    #[must_use]
    pub fn id() -> Self {
        Self::int(
            "id",
            &[
                FieldAttribute::id(),
                FieldAttribute::default_auto_increment(),
            ],
        )
    }

    /// Standard `createdAt` field.
    #[must_use]
    pub fn created_at() -> Self {
        Self::datetime("createdAt", &[FieldAttribute::default_now()])
    }

    /// Standard `updatedAt` field.
    #[must_use]
    pub fn updated_at() -> Self {
        Self::datetime("updatedAt", &[])
    }

    /// Print the type of the field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::model::{
    ///     Field,
    ///     FieldType,
    /// };
    ///
    /// assert_eq!(
    ///     Field {
    ///         name: "id".to_owned(),
    ///         r#type: FieldType::Name("Int".to_owned()),
    ///         required: true,
    ///         array: false,
    ///         attributes: vec![],
    ///     }
    ///     .print_type(),
    ///     "Int"
    /// );
    /// ```
    #[must_use]
    pub fn print_type(&self) -> String {
        let Self {
            r#type,
            required,
            array,
            ..
        } = self;

        let array = if *array { "[]" } else { "" };
        let optional = if *required { "" } else { "?" };

        format!("{type}{array}{optional}")
    }
}

/// A Prisma model.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Model {
    /// The name of the model. Must adhere to `[A-Za-z][A-Za-z0-9_]*`. Usually
    /// pascal case. May not be a reserved Prisma keyword or a JavaScript
    /// reserved keyword. Should be singular.
    pub name: String,
    /// The fields of the model.
    pub fields: Vec<Field>,
    /// Block attributes.
    pub attributes: Vec<BlockAttribute>,
}

impl Model {
    /// Create a new model.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    /// * `fields` - The fields of the model.
    /// * `attributes` - Block attributes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::model::{
    ///     BlockAttribute,
    ///     Field,
    ///     FieldType,
    ///     Model,
    /// };
    ///
    /// let model = Model::new(
    ///     "Foo",
    ///     &[Field::int("bar", &[]), Field::int("baz", &[])],
    ///     &[BlockAttribute::map("foo")],
    /// );
    ///
    /// assert_eq!(model.name, "Foo");
    /// assert_eq!(model.fields.len(), 2);
    /// assert_eq!(model.attributes.len(), 1);
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        fields: &[Field],
        attributes: &[BlockAttribute],
    ) -> Self {
        Self {
            name: name.to_owned(),
            fields: fields.to_owned(),
            attributes: attributes.to_owned(),
        }
    }

    /// Create a new model with the standard `id`, `createdAt`, and `updatedAt`
    /// fields.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    /// * `fields` - The fields of the model.
    /// * `attributes` - Block attributes.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::model::{
    ///     BlockAttribute,
    ///     Field,
    ///     FieldType,
    ///     Model,
    /// };
    ///
    /// let model = Model::standard(
    ///     "Foo",
    ///     &[Field::int("bar", &[]), Field::int("baz", &[])],
    ///     &[BlockAttribute::map("foo")],
    /// );
    ///
    /// assert_eq!(model.name, "Foo");
    /// assert_eq!(model.fields.len(), 4);
    /// assert_eq!(model.attributes.len(), 1);
    /// ```
    #[must_use]
    pub fn standard(
        name: &str,
        fields: &mut Vec<Field>,
        attributes: &[BlockAttribute],
    ) -> Self {
        fields.push(Field::id());
        fields.push(Field::created_at());

        Self::new(name, fields, attributes)
    }
}

impl Display for Model {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            name,
            fields,
            attributes,
        } = self;

        let max_field_name_length = fields
            .iter()
            .map(|field| field.name.len())
            .max()
            .unwrap_or(0)
            + 1;

        let max_field_type_length = fields
            .iter()
            .map(|field| field.print_type().len())
            .max()
            .unwrap_or(0)
            + 1;

        let indent = indent::psl(1);

        let mut fields = fields
            .iter()
            .map(|field| {
                let Field {
                    name, attributes, ..
                } = field;

                let mut string =
                    format!("{indent}{name:<max_field_name_length$}");
                let r#type = field.print_type();

                if attributes.is_empty() {
                    let _ = write!(string, "{type}");
                } else {
                    let attributes = space_separated(attributes);

                    let _ = write!(
                        string,
                        "{type:<max_field_type_length$}{attributes}"
                    );
                }

                string
            })
            .collect::<Vec<_>>();

        fields.sort();

        let fields = newline_separated(&fields);

        let attributes = if attributes.is_empty() {
            String::new()
        } else {
            format!("\n\n{}", space_separated(attributes))
        };

        write!(f, "model {name} {{\n{fields}{attributes}\n}}")
    }
}

impl Print for Model {
    fn print(
        &self,
        _: usize,
    ) -> String {
        self.to_string()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::prisma::{
            Argument,
            Value,
        },
    };

    #[test]
    fn test_display_field_type() {
        assert_eq!(FieldType::name("Int").to_string(), "Int");

        assert_eq!(
            FieldType::function(
                "unique",
                &[Value::array(&[
                    Value::keyword("firstName"),
                    Value::keyword("lastName"),
                ])],
            )
            .to_string(),
            "unique([firstName, lastName])"
        );
    }

    #[test]
    fn test_display_model() {
        let model = Model {
            name: "User".to_owned(),
            fields: vec![
                Field::int(
                    "id",
                    &[FieldAttribute {
                        group: None,
                        name: "default".to_owned(),
                        arguments: vec![Argument::unnamed(&Value::function(
                            "autoincrement",
                            &[],
                        ))],
                    }],
                ),
                Field::boolean(
                    "isAdmin",
                    &[FieldAttribute::new(
                        "default",
                        &[Argument::unnamed(&Value::Boolean(false))],
                        None,
                    )],
                ),
                Field::string("firstName", &[]),
                Field::string("lastName", &[]),
            ],
            attributes: vec![BlockAttribute::new(
                "unique",
                &[Argument::unnamed(&Value::Array(vec![
                    Value::keyword("firstName"),
                    Value::keyword("lastName"),
                ]))],
                None,
            )],
        };

        assert_eq!(
            model.to_string(),
            "

model User {
  firstName String
  id        Int     @default(autoincrement())
  isAdmin   Boolean @default(false)
  lastName  String

  @@unique([firstName, lastName])
}

"
            .trim()
        );
    }
}
