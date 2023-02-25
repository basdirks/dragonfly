use {
    super::{
        attribute,
        value::Function,
    },
    crate::{
        ast::{
            Field as AstField,
            Model as AstModel,
            Scalar as AstScalar,
            Type as AstType,
        },
        generator::printer::{
            indent,
            newline_separated,
            space_separated,
            Print,
        },
    },
    std::fmt::{
        Display,
        Write,
    },
};

/// A field type.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum FieldType {
    /// A name.
    Name(String),
    /// A function.
    Function(Function),
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

impl From<AstScalar> for FieldType {
    fn from(scalar: AstScalar) -> Self {
        match scalar {
            AstScalar::Boolean => Self::Name("Boolean".to_owned()),
            AstScalar::DateTime => Self::Name("DateTime".to_owned()),
            AstScalar::Float => Self::Name("Float".to_owned()),
            AstScalar::Int => Self::Name("Int".to_owned()),
            AstScalar::Reference(name) => Self::Name(name),
            AstScalar::String => Self::Name("String".to_owned()),
        }
    }
}

/// A field.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    pub attributes: Vec<attribute::Field>,
}

impl Field {
    /// Standard `id` field.
    #[must_use]
    pub fn id() -> Self {
        Self {
            name: "id".to_owned(),
            r#type: FieldType::Name("Int".to_owned()),
            required: true,
            array: false,
            attributes: vec![
                attribute::Field::id(),
                attribute::Field::default_auto_increment(),
            ],
        }
    }

    /// Standard `createdAt` field.
    #[must_use]
    pub fn created_at() -> Self {
        Self {
            name: "createdAt".to_owned(),
            r#type: FieldType::Name("DateTime".to_owned()),
            required: true,
            array: false,
            attributes: vec![],
        }
    }

    /// Standard `updatedAt` field.
    #[must_use]
    pub fn updated_at() -> Self {
        Self {
            name: "updatedAt".to_owned(),
            r#type: FieldType::Name("DateTime".to_owned()),
            required: true,
            array: false,
            attributes: vec![],
        }
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    /// The name of the model. Must adhere to `[A-Za-z][A-Za-z0-9_]*`. Usually
    /// pascal case. May not be a reserved Prisma keyword or a JavaScript
    /// reserved keyword. Should be singular.
    pub name: String,
    /// The fields of the model.
    pub fields: Vec<Field>,
    /// Block attributes.
    pub attributes: Vec<attribute::Block>,
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

        let longest_field_name = fields
            .iter()
            .map(|field| field.name.len())
            .max()
            .unwrap_or(0)
            + 1;

        let longest_field_type = fields
            .iter()
            .map(|field| field.print_type().len())
            .max()
            .unwrap_or(0)
            + 1;

        let indent = indent::psl(1);

        let fields = fields
            .iter()
            .map(|field| {
                let Field {
                    name, attributes, ..
                } = field;

                let mut string = format!("{indent}{name:<longest_field_name$}");
                let r#type = field.print_type();

                if attributes.is_empty() {
                    let _ = write!(string, "{type}");
                } else {
                    let attributes = space_separated(attributes);

                    let _ = write!(
                        string,
                        "{type:<longest_field_type$}{attributes}"
                    );
                }

                string
            })
            .collect::<Vec<_>>();

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

impl From<AstModel> for Model {
    fn from(
        AstModel {
            name,
            fields: ast_fields,
        }: AstModel
    ) -> Self {
        let mut fields =
            vec![Field::id(), Field::created_at(), Field::updated_at()];

        for AstField { r#type, name } in ast_fields.values() {
            let (array, scalar) = match r#type.clone() {
                AstType::Array(scalar) => (true, scalar),
                AstType::Scalar(scalar) => (false, scalar),
            };

            let field = Field {
                name: name.clone(),
                array,
                required: true,
                r#type: scalar.into(),
                attributes: vec![],
            };

            fields.push(field);
        }

        Self {
            name,
            fields,
            attributes: vec![],
        }
    }
}

impl From<&AstModel> for Model {
    fn from(model: &AstModel) -> Self {
        model.clone().into()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::prisma::{
            attribute::Argument,
            value::Value,
        },
    };

    #[test]
    fn test_display_field_type() {
        assert_eq!(FieldType::Name("Int".to_owned()).to_string(), "Int");

        assert_eq!(
            FieldType::Function(Function {
                name: "unique".to_owned(),
                parameters: vec![Value::Array(vec![
                    Value::String("firstName".to_owned()),
                    Value::String("lastName".to_owned()),
                ])],
            })
            .to_string(),
            "unique([firstName, lastName])"
        );
    }

    #[test]
    fn test_display_model() {
        let model = Model {
            name: "User".to_owned(),
            fields: vec![
                Field {
                    name: "id".to_owned(),
                    r#type: FieldType::Name("Int".to_owned()),
                    required: true,
                    array: false,
                    attributes: vec![attribute::Field {
                        group: None,
                        name: "default".to_owned(),
                        arguments: vec![Argument::Function(Function {
                            name: "autoincrement".to_owned(),
                            parameters: vec![],
                        })],
                    }],
                },
                Field {
                    name: "firstName".to_owned(),
                    r#type: FieldType::Name("String".to_owned()),
                    required: true,
                    array: false,
                    attributes: vec![],
                },
                Field {
                    name: "lastName".to_owned(),
                    r#type: FieldType::Name("String".to_owned()),
                    required: true,
                    array: false,
                    attributes: vec![],
                },
                Field {
                    name: "isAdmin".to_owned(),
                    r#type: FieldType::Name("Boolean".to_owned()),
                    required: true,
                    array: false,
                    attributes: vec![attribute::Field {
                        group: None,
                        name: "default".to_owned(),
                        arguments: vec![Argument::Value(Value::String(
                            "false".to_owned(),
                        ))],
                    }],
                },
            ],
            attributes: vec![attribute::Block {
                group: None,
                name: "unique".to_owned(),
                arguments: vec![Argument::Value(Value::Array(vec![
                    Value::String("firstName".to_owned()),
                    Value::String("lastName".to_owned()),
                ]))],
            }],
        };

        assert_eq!(
            model.to_string(),
            "\
model User {
  id        Int     @default(autoincrement())
  firstName String
  lastName  String
  isAdmin   Boolean @default(false)

  @@unique([firstName, lastName])
}"
        );
    }

    #[test]
    fn test_field_type_from_string() {
        assert_eq!(
            FieldType::from(AstScalar::String),
            FieldType::Name("String".to_owned())
        );
    }

    #[test]
    fn test_field_type_from_int() {
        assert_eq!(
            FieldType::from(AstScalar::Int),
            FieldType::Name("Int".to_owned())
        );
    }

    #[test]
    fn test_field_type_from_float() {
        assert_eq!(
            FieldType::from(AstScalar::Float),
            FieldType::Name("Float".to_owned())
        );
    }

    #[test]
    fn test_field_type_from_boolean() {
        assert_eq!(
            FieldType::from(AstScalar::Boolean),
            FieldType::Name("Boolean".to_owned())
        );
    }

    #[test]
    fn test_field_type_from_datetime() {
        assert_eq!(
            FieldType::from(AstScalar::DateTime),
            FieldType::Name("DateTime".to_owned())
        );
    }

    #[test]
    fn test_field_type_from_reference() {
        assert_eq!(
            FieldType::from(AstScalar::Reference("User".to_owned())),
            FieldType::Name("User".to_owned())
        );
    }
}
