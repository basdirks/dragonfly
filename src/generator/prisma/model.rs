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
        },
    },
    std::fmt::Display,
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
            AstScalar::Boolean => Self::Name("Boolean".to_string()),
            AstScalar::DateTime => Self::Name("DateTime".to_string()),
            AstScalar::Float => Self::Name("Float".to_string()),
            AstScalar::Int => Self::Name("Int".to_string()),
            AstScalar::Reference(name) => Self::Name(name),
            AstScalar::String => Self::Name("String".to_string()),
        }
    }
}

/// A field.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    /// The name of the field. Must adhere to the following regular expression:
    /// [A-Za-z][A-Za-z0-9_]*. Must start with a letter. Usually camelCase.
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

impl Display for Field {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            name,
            r#type,
            required,
            array,
            attributes,
        } = self;

        let indent = indent::psl(1);
        let array = if *array { "[]" } else { "" };
        let optional = if *required { "" } else { "?" };

        let attributes = if attributes.is_empty() {
            String::new()
        } else {
            format!(" {}", space_separated(&self.attributes))
        };

        write!(f, "{indent}{name} {type}{array}{optional}{attributes}")
    }
}

/// A Prisma model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    /// The name of the model. Must adhere to `[A-Za-z][A-Za-z0-9_]*`. Usually
    /// PascalCase. May not be a reserved Prisma keyword or a JavaScript
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

        let fields = newline_separated(fields);

        let attributes = if attributes.is_empty() {
            String::new()
        } else {
            format!("\n\n{}", space_separated(attributes))
        };

        write!(f, "model {name} {{\n{fields}{attributes}\n}}")
    }
}

impl From<AstModel> for Model {
    fn from(AstModel { name, fields }: AstModel) -> Self {
        let fields = fields
            .into_iter()
            .map(|(_, AstField { r#type, name })| {
                let (array, scalar) = match r#type {
                    AstType::Array(scalar) => (true, scalar),
                    AstType::Scalar(scalar) => (false, scalar),
                };

                Field {
                    name,
                    array,
                    required: true,
                    r#type: scalar.into(),
                    attributes: vec![],
                }
            })
            .collect();

        Self {
            name,
            fields,
            attributes: vec![],
        }
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
        assert_eq!(FieldType::Name("Int".to_string()).to_string(), "Int");

        assert_eq!(
            FieldType::Function(Function {
                name: "unique".to_string(),
                parameters: vec![Value::Array(vec![
                    Value::String("firstName".to_string()),
                    Value::String("lastName".to_string()),
                ]),],
            })
            .to_string(),
            "unique([firstName, lastName])"
        );
    }

    #[test]
    fn test_display_field() {
        assert_eq!(
            Field {
                name: "id".to_string(),
                r#type: FieldType::Name("Int".to_string()),
                required: true,
                array: false,
                attributes: vec![],
            }
            .to_string(),
            "  id Int"
        );

        assert_eq!(
            Field {
                name: "id".to_string(),
                r#type: FieldType::Name("Int".to_string()),
                required: true,
                array: false,
                attributes: vec![attribute::Field {
                    group: None,
                    name: "default".to_string(),
                    arguments: vec![Argument::Function(Function {
                        name: "autoincrement".to_string(),
                        parameters: vec![],
                    })],
                }],
            }
            .to_string(),
            "  id Int @default(autoincrement())"
        );
    }

    #[test]
    fn test_display_model() {
        let model = Model {
            name: "User".to_string(),
            fields: vec![
                Field {
                    name: "id".to_string(),
                    r#type: FieldType::Name("Int".to_string()),
                    required: true,
                    array: false,
                    attributes: vec![attribute::Field {
                        group: None,
                        name: "default".to_string(),
                        arguments: vec![Argument::Function(Function {
                            name: "autoincrement".to_string(),
                            parameters: vec![],
                        })],
                    }],
                },
                Field {
                    name: "firstName".to_string(),
                    r#type: FieldType::Name("String".to_string()),
                    required: true,
                    array: false,
                    attributes: vec![],
                },
                Field {
                    name: "lastName".to_string(),
                    r#type: FieldType::Name("String".to_string()),
                    required: true,
                    array: false,
                    attributes: vec![],
                },
                Field {
                    name: "isAdmin".to_string(),
                    r#type: FieldType::Name("Boolean".to_string()),
                    required: true,
                    array: false,
                    attributes: vec![attribute::Field {
                        group: None,
                        name: "default".to_string(),
                        arguments: vec![Argument::Value(Value::String(
                            "false".to_string(),
                        ))],
                    }],
                },
            ],
            attributes: vec![attribute::Block {
                group: None,
                name: "unique".to_string(),
                arguments: vec![Argument::Value(Value::Array(vec![
                    Value::String("firstName".to_string()),
                    Value::String("lastName".to_string()),
                ]))],
            }],
        };

        assert_eq!(
            model.to_string(),
            "\
model User {
  id Int @default(autoincrement())
  firstName String
  lastName String
  isAdmin Boolean @default(false)

  @@unique([firstName, lastName])
}"
        );
    }
}
