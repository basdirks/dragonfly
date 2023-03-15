use {
    super::ArgumentType,
    crate::{
        Cardinality,
        Type,
    },
    std::{
        borrow::Cow,
        collections::BTreeSet,
    },
};

/// An argument to a query.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Argument<'a> {
    /// The name of the argument.
    pub name: Cow<'a, str>,
    /// The type of the argument.
    pub r#type: ArgumentType<'a>,
    /// The cardinality of the argument.
    pub cardinality: Cardinality,
}

impl<'a> Argument<'a> {
    /// Create an argument from an AST type.
    ///
    /// # Arguments
    ///
    /// * `argument_name` - The name of the argument.
    /// * `ast_type` - The AST type.
    /// * `enum_names` - The names of the enums.
    #[must_use]
    pub fn from_ast_type(
        argument: &ast::query::Argument<'a>,
        enum_names: &BTreeSet<Cow<'a, str>>,
    ) -> Option<Self> {
        let argument_name = argument.name.clone();

        match &argument.r#type {
            ast::r#type::Type::Scalar(ast::r#type::Scalar::Boolean) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::Boolean),
                    cardinality: Cardinality::One,
                })
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::DateTime) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::DateTime),
                    cardinality: Cardinality::One,
                })
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::Float) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::Float),
                    cardinality: Cardinality::One,
                })
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::Int) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::Int),
                    cardinality: Cardinality::One,
                })
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::String) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::String),
                    cardinality: Cardinality::One,
                })
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::Reference(name)) => {
                enum_names.contains(name).then(|| {
                    Self {
                        name: argument_name,
                        r#type: ArgumentType::Enum(name.clone()),
                        cardinality: Cardinality::One,
                    }
                })
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::Boolean) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::Boolean),
                    cardinality: Cardinality::Many,
                })
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::DateTime) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::DateTime),
                    cardinality: Cardinality::Many,
                })
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::Float) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::Float),
                    cardinality: Cardinality::Many,
                })
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::Int) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::Int),
                    cardinality: Cardinality::Many,
                })
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::String) => {
                Some(Self {
                    name: argument_name,
                    r#type: ArgumentType::Type(Type::String),
                    cardinality: Cardinality::Many,
                })
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::Reference(name)) => {
                enum_names.contains(name).then(|| {
                    Self {
                        name: argument_name,
                        r#type: ArgumentType::Enum(name.clone()),
                        cardinality: Cardinality::Many,
                    }
                })
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        std::iter::once,
    };

    #[test]
    fn test_from_boolean() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "is_admin".into(),
                    r#type: ast::r#type::Type::Scalar(
                        ast::r#type::Scalar::Boolean
                    ),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "is_admin".into(),
                r#type: ArgumentType::Type(Type::Boolean),
                cardinality: Cardinality::One,
            })
        );
    }

    #[test]
    fn test_from_date_time() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "created_at".into(),
                    r#type: ast::r#type::Type::Scalar(
                        ast::r#type::Scalar::DateTime
                    ),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "created_at".into(),
                r#type: ArgumentType::Type(Type::DateTime),
                cardinality: Cardinality::One,
            })
        );
    }

    #[test]
    fn test_from_float() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "price".into(),
                    r#type: ast::r#type::Type::Scalar(
                        ast::r#type::Scalar::Float
                    ),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "price".into(),
                r#type: ArgumentType::Type(Type::Float),
                cardinality: Cardinality::One,
            })
        );
    }

    #[test]
    fn test_from_int() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "age".into(),
                    r#type: ast::r#type::Type::Scalar(ast::r#type::Scalar::Int),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "age".into(),
                r#type: ArgumentType::Type(Type::Int),
                cardinality: Cardinality::One,
            })
        );
    }

    #[test]
    fn test_from_string() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "name".into(),
                    r#type: ast::r#type::Type::Scalar(
                        ast::r#type::Scalar::String
                    ),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "name".into(),
                r#type: ArgumentType::Type(Type::String),
                cardinality: Cardinality::One,
            })
        );
    }

    #[test]
    fn test_from_enum() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "role".into(),
                    r#type: ast::r#type::Type::Scalar(
                        ast::r#type::Scalar::Reference("Role".into())
                    ),
                },
                &once("Role").map(Into::into).collect()
            ),
            Some(Argument {
                name: "role".into(),
                r#type: ArgumentType::Enum("Role".into()),
                cardinality: Cardinality::One,
            })
        );
    }

    #[test]
    fn test_from_boolean_array() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "is_admin".into(),
                    r#type: ast::r#type::Type::Array(
                        ast::r#type::Scalar::Boolean
                    ),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "is_admin".into(),
                r#type: ArgumentType::Type(Type::Boolean),
                cardinality: Cardinality::Many,
            })
        );
    }

    #[test]
    fn test_from_date_time_array() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "created_at".into(),
                    r#type: ast::r#type::Type::Array(
                        ast::r#type::Scalar::DateTime
                    ),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "created_at".into(),
                r#type: ArgumentType::Type(Type::DateTime),
                cardinality: Cardinality::Many,
            })
        );
    }

    #[test]
    fn test_from_float_array() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "price".into(),
                    r#type: ast::r#type::Type::Array(
                        ast::r#type::Scalar::Float
                    ),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "price".into(),
                r#type: ArgumentType::Type(Type::Float),
                cardinality: Cardinality::Many,
            })
        );
    }

    #[test]
    fn test_from_int_array() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "age".into(),
                    r#type: ast::r#type::Type::Array(ast::r#type::Scalar::Int),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "age".into(),
                r#type: ArgumentType::Type(Type::Int),
                cardinality: Cardinality::Many,
            })
        );
    }

    #[test]
    fn test_from_string_array() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "name".into(),
                    r#type: ast::r#type::Type::Array(
                        ast::r#type::Scalar::String
                    ),
                },
                &BTreeSet::new()
            ),
            Some(Argument {
                name: "name".into(),
                r#type: ArgumentType::Type(Type::String),
                cardinality: Cardinality::Many,
            })
        );
    }

    #[test]
    fn test_from_enum_array() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "roles".into(),
                    r#type: ast::r#type::Type::Array(
                        ast::r#type::Scalar::Reference("Role".into())
                    ),
                },
                &once("Role").map(Into::into).collect()
            ),
            Some(Argument {
                name: "roles".into(),
                r#type: ArgumentType::Enum("Role".into()),
                cardinality: Cardinality::Many,
            })
        );
    }

    #[test]
    fn test_from_owned() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "foo".into(),
                    r#type: ast::r#type::Type::Scalar(
                        ast::r#type::Scalar::Owned("Foo".into())
                    ),
                },
                &BTreeSet::new()
            ),
            None
        );
    }

    #[test]
    fn test_from_owned_array() {
        assert_eq!(
            Argument::from_ast_type(
                &ast::query::Argument {
                    name: "foos".into(),
                    r#type: ast::r#type::Type::Array(
                        ast::r#type::Scalar::Owned("Foo".into())
                    ),
                },
                &BTreeSet::new()
            ),
            None
        );
    }
}
