use {
    super::field_type::FieldType,
    crate::FieldAttribute,
    ir::{
        self,
        Cardinality,
    },
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// Is the field required?
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Modifier {
    /// The field is a list.
    List,
    /// The field is required.
    None,
    /// The field is optional.
    Optional,
}

/// A field.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Field<'a> {
    /// The name of the field. Must adhere to the following regular expression:
    /// [A-Za-z][A-Za-z0-9_]*. Must start with an alphabetic character. Usually
    /// camel case.
    pub name: Cow<'a, str>,
    /// The type of the field.
    pub r#type: FieldType<'a>,
    /// Is the field optional, required, or a list?
    pub modifier: Modifier,
    /// Field attributes.
    pub attributes: Vec<FieldAttribute<'a>>,
}

impl<'a> Field<'a> {
    /// Standard `id` field.
    #[must_use]
    pub fn id() -> Self {
        Self {
            name: "id".into(),
            r#type: FieldType::Name("Int".into()),
            modifier: Modifier::None,
            attributes: vec![
                FieldAttribute::id(),
                FieldAttribute::default_auto_increment(),
            ],
        }
    }

    /// Standard `createdAt` field.
    #[must_use]
    pub fn created_at() -> Self {
        Self {
            name: "createdAt".into(),
            r#type: FieldType::Name("DateTime".into()),
            modifier: Modifier::None,
            attributes: vec![FieldAttribute::default_now()],
        }
    }

    /// Print the type of the field.
    ///
    /// # Arguments
    ///
    /// * `f` - The writer to print to.
    ///
    /// # Errors
    ///
    /// If an error occurs while writing to the stream.
    pub fn print_type(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            r#type, modifier, ..
        } = self;

        r#type.print(f)?;

        match modifier {
            Modifier::Optional => write!(f, "?"),
            Modifier::List => write!(f, "[]"),
            Modifier::None => Ok(()),
        }
    }
}

impl<'a> From<ir::Field<'a>> for Field<'a> {
    fn from(value: ir::Field<'a>) -> Self {
        let ir::Field {
            name,
            r#type,
            cardinality,
        } = value;

        match (r#type, cardinality) {
            (ir::Type::Boolean, Cardinality::One) => {
                Self {
                    name,
                    r#type: FieldType::Name("Boolean".into()),
                    modifier: Modifier::None,
                    attributes: Vec::new(),
                }
            }
            (ir::Type::DateTime, Cardinality::One) => {
                Self {
                    name,
                    r#type: FieldType::Name("DateTime".into()),
                    modifier: Modifier::None,
                    attributes: Vec::new(),
                }
            }
            (ir::Type::Float, Cardinality::One) => {
                Self {
                    name,
                    r#type: FieldType::Name("Float".into()),
                    modifier: Modifier::None,
                    attributes: Vec::new(),
                }
            }
            (ir::Type::Int, Cardinality::One) => {
                Self {
                    name,
                    r#type: FieldType::Name("Int".into()),
                    modifier: Modifier::None,
                    attributes: Vec::new(),
                }
            }
            (ir::Type::String, Cardinality::One) => {
                Self {
                    name,
                    r#type: FieldType::Name("String".into()),
                    modifier: Modifier::None,
                    attributes: Vec::new(),
                }
            }
            (ir::Type::Boolean, Cardinality::Many) => {
                Self {
                    name,
                    r#type: FieldType::Name("Boolean".into()),
                    modifier: Modifier::List,
                    attributes: Vec::new(),
                }
            }
            (ir::Type::DateTime, Cardinality::Many) => {
                Self {
                    name,
                    r#type: FieldType::Name("DateTime".into()),
                    modifier: Modifier::List,
                    attributes: Vec::new(),
                }
            }
            (ir::Type::Float, Cardinality::Many) => {
                Self {
                    name,
                    r#type: FieldType::Name("Float".into()),
                    modifier: Modifier::List,
                    attributes: Vec::new(),
                }
            }
            (ir::Type::Int, Cardinality::Many) => {
                Self {
                    name,
                    r#type: FieldType::Name("Int".into()),
                    modifier: Modifier::List,
                    attributes: Vec::new(),
                }
            }
            (ir::Type::String, Cardinality::Many) => {
                Self {
                    name,
                    r#type: FieldType::Name("String".into()),
                    modifier: Modifier::List,
                    attributes: Vec::new(),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let field = Field {
            name: "id".into(),
            r#type: FieldType::Name("Int".into()),
            modifier: Modifier::List,
            attributes: Vec::new(),
        };

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Int[]");
    }

    #[test]
    fn test_print_optional() {
        let field = Field {
            name: "aId".into(),
            r#type: FieldType::Name("Int".into()),
            modifier: Modifier::Optional,
            attributes: Vec::new(),
        };

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Int?");
    }

    #[test]
    fn test_from_one_boolean() {
        let field = Field::from(ir::Field {
            name: "predicate".into(),
            r#type: ir::Type::Boolean,
            cardinality: Cardinality::One,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Boolean");
    }

    #[test]
    fn test_from_one_datetime() {
        let field = Field::from(ir::Field {
            name: "createdAt".into(),
            r#type: ir::Type::DateTime,
            cardinality: Cardinality::One,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "DateTime");
    }

    #[test]
    fn test_from_one_float() {
        let field = Field::from(ir::Field {
            name: "price".into(),
            r#type: ir::Type::Float,
            cardinality: Cardinality::One,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Float");
    }

    #[test]
    fn test_from_one_int() {
        let field = Field::from(ir::Field {
            name: "id".into(),
            r#type: ir::Type::Int,
            cardinality: Cardinality::One,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Int");
    }

    #[test]
    fn test_from_one_string() {
        let field = Field::from(ir::Field {
            name: "name".into(),
            r#type: ir::Type::String,
            cardinality: Cardinality::One,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "String");
    }

    #[test]
    fn test_from_many_boolean() {
        let field = Field::from(ir::Field {
            name: "predicates".into(),
            r#type: ir::Type::Boolean,
            cardinality: Cardinality::Many,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Boolean[]");
    }

    #[test]
    fn test_from_many_datetime() {
        let field = Field::from(ir::Field {
            name: "createdAt".into(),
            r#type: ir::Type::DateTime,
            cardinality: Cardinality::Many,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "DateTime[]");
    }

    #[test]
    fn test_from_many_float() {
        let field = Field::from(ir::Field {
            name: "prices".into(),
            r#type: ir::Type::Float,
            cardinality: Cardinality::Many,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Float[]");
    }

    #[test]
    fn test_from_many_int() {
        let field = Field::from(ir::Field {
            name: "ids".into(),
            r#type: ir::Type::Int,
            cardinality: Cardinality::Many,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Int[]");
    }

    #[test]
    fn test_from_many_string() {
        let field = Field::from(ir::Field {
            name: "names".into(),
            r#type: ir::Type::String,
            cardinality: Cardinality::Many,
        });

        let mut f = Vec::new();

        field.print_type(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "String[]");
    }
}
