use {
    super::ArgumentType,
    crate::{
        ast,
        ir::{
            Cardinality,
            Type,
        },
    },
    std::collections::BTreeSet,
};

/// An argument to a query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argument {
    /// The name of the argument.
    pub name: String,
    /// The type of the argument.
    pub r#type: ArgumentType,
    /// The cardinality of the argument.
    pub cardinality: Cardinality,
}

impl Argument {
    /// Create a new argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `r#type` - The type of the argument.
    /// * `cardinality` - The cardinality of the argument.
    #[must_use]
    pub fn new(
        name: &str,
        r#type: ArgumentType,
        cardinality: Cardinality,
    ) -> Self {
        Self {
            name: name.to_owned(),
            r#type,
            cardinality,
        }
    }

    /// Create an argument with a single enum.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `enum_name` - The name of the enum.
    #[must_use]
    pub fn r#enum(
        name: &str,
        enum_name: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Enum(enum_name.to_owned()),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with a single boolean.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn boolean(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::Boolean),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with a single date time.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn date_time(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::DateTime),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with a single float.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn float(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::Float),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with a single integer.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn int(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::Int),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with a single string.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::String),
            cardinality: Cardinality::One,
        }
    }

    /// Create an argument with an array of enums.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `enum_name` - The name of the enum.
    #[must_use]
    pub fn enums(
        name: &str,
        enum_name: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Enum(enum_name.to_owned()),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument with an array of booleans.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn booleans(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::Boolean),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument with an array of date times.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn date_times(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::DateTime),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument with an array of floats.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn floats(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::Float),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument with an array of integers.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn ints(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::Int),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument with an array of strings.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn strings(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: ArgumentType::Type(Type::String),
            cardinality: Cardinality::Many,
        }
    }

    /// Create an argument from an AST type.
    ///
    /// # Arguments
    ///
    /// * `argument_name` - The name of the argument.
    /// * `ast_type` - The AST type.
    /// * `enum_names` - The names of the enums.
    #[must_use]
    pub fn from_ast_type(
        argument: &ast::QueryArgument,
        enum_names: &BTreeSet<String>,
    ) -> Option<Self> {
        match &argument.r#type {
            ast::r#type::Type::Scalar(ast::r#type::Scalar::Boolean) => {
                Some(Self::boolean(&argument.name))
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::DateTime) => {
                Some(Self::date_time(&argument.name))
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::Float) => {
                Some(Self::float(&argument.name))
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::Int) => {
                Some(Self::int(&argument.name))
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::String) => {
                Some(Self::string(&argument.name))
            }
            ast::r#type::Type::Scalar(ast::r#type::Scalar::Reference(name)) => {
                enum_names
                    .contains(name)
                    .then(|| Self::r#enum(&argument.name, name))
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::Boolean) => {
                Some(Self::booleans(&argument.name))
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::DateTime) => {
                Some(Self::date_times(&argument.name))
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::Float) => {
                Some(Self::floats(&argument.name))
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::Int) => {
                Some(Self::ints(&argument.name))
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::String) => {
                Some(Self::strings(&argument.name))
            }
            ast::r#type::Type::Array(ast::r#type::Scalar::Reference(name)) => {
                enum_names
                    .contains(name)
                    .then(|| Self::enums(&argument.name, name))
            }
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Argument::new(
                "name",
                ArgumentType::Type(Type::String),
                Cardinality::One
            ),
            Argument {
                name: "name".to_owned(),
                r#type: ArgumentType::Type(Type::String),
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_enum() {
        assert_eq!(
            Argument::r#enum("role", "Role"),
            Argument {
                name: "role".to_owned(),
                r#type: ArgumentType::Enum("Role".to_owned()),
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_boolean() {
        assert_eq!(
            Argument::boolean("is_admin"),
            Argument {
                name: "is_admin".to_owned(),
                r#type: ArgumentType::boolean(),
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_date_time() {
        assert_eq!(
            Argument::date_time("created_at"),
            Argument {
                name: "created_at".to_owned(),
                r#type: ArgumentType::date_time(),
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_float() {
        assert_eq!(
            Argument::float("price"),
            Argument {
                name: "price".to_owned(),
                r#type: ArgumentType::float(),
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_int() {
        assert_eq!(
            Argument::int("age"),
            Argument {
                name: "age".to_owned(),
                r#type: ArgumentType::int(),
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(
            Argument::string("name"),
            Argument {
                name: "name".to_owned(),
                r#type: ArgumentType::string(),
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_enums() {
        assert_eq!(
            Argument::enums("role", "Role"),
            Argument {
                name: "role".to_owned(),
                r#type: ArgumentType::Enum("Role".to_owned()),
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_booleans() {
        assert_eq!(
            Argument::booleans("is_admin"),
            Argument {
                name: "is_admin".to_owned(),
                r#type: ArgumentType::Type(Type::Boolean),
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_date_times() {
        assert_eq!(
            Argument::date_times("created_at"),
            Argument {
                name: "created_at".to_owned(),
                r#type: ArgumentType::Type(Type::DateTime),
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_floats() {
        assert_eq!(
            Argument::floats("price"),
            Argument {
                name: "price".to_owned(),
                r#type: ArgumentType::Type(Type::Float),
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_ints() {
        assert_eq!(
            Argument::ints("age"),
            Argument {
                name: "age".to_owned(),
                r#type: ArgumentType::Type(Type::Int),
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_strings() {
        assert_eq!(
            Argument::strings("name"),
            Argument {
                name: "name".to_owned(),
                r#type: ArgumentType::Type(Type::String),
                cardinality: Cardinality::Many,
            }
        );
    }
}
