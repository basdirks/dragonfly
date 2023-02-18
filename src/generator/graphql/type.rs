use {
    crate::ast::{
        Scalar as AstScalar,
        Type as AstType,
    },
    std::fmt::Display,
};

/// GraphQL types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Type {
    /// A list type.
    List(Box<Type>),
    /// A non-null type.
    NonNull(Box<Type>),
    /// A name.
    Name(String),
}

impl Display for Type {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::List(inner) => write!(f, "[{inner}]"),
            Self::NonNull(inner) => write!(f, "{inner}!"),
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}

impl From<AstType> for Type {
    fn from(ast_type: AstType) -> Self {
        let print_scalar = |scalar| {
            match scalar {
                AstScalar::Boolean => "Boolean".to_string(),
                AstScalar::DateTime => "DateTime".to_string(),
                AstScalar::Float => "Float".to_string(),
                AstScalar::Int => "Int".to_string(),
                AstScalar::String => "String".to_string(),
                AstScalar::Reference(name) => name,
            }
        };

        let inner = match ast_type {
            AstType::Array(scalar) => {
                Self::List(Box::new(Self::Name(print_scalar(scalar))))
            }
            AstType::Scalar(scalar) => Self::Name(print_scalar(scalar)),
        };

        Self::NonNull(Box::new(inner))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_ast_type() {
        assert_eq!(
            Type::from(AstType::Scalar(AstScalar::Boolean)),
            Type::NonNull(Box::new(Type::Name("Boolean".to_string())))
        );

        assert_eq!(
            Type::from(AstType::Scalar(AstScalar::DateTime)),
            Type::NonNull(Box::new(Type::Name("DateTime".to_string())))
        );

        assert_eq!(
            Type::from(AstType::Scalar(AstScalar::Float)),
            Type::NonNull(Box::new(Type::Name("Float".to_string())))
        );

        assert_eq!(
            Type::from(AstType::Scalar(AstScalar::Int)),
            Type::NonNull(Box::new(Type::Name("Int".to_string())))
        );

        assert_eq!(
            Type::from(AstType::Scalar(AstScalar::String)),
            Type::NonNull(Box::new(Type::Name("String".to_string())))
        );

        assert_eq!(
            Type::from(AstType::Scalar(AstScalar::Reference(
                "Foo".to_string()
            ))),
            Type::NonNull(Box::new(Type::Name("Foo".to_string())))
        );

        assert_eq!(
            Type::from(AstType::Array(AstScalar::Boolean)),
            Type::NonNull(Box::new(Type::List(Box::new(Type::Name(
                "Boolean".to_string()
            )))))
        );

        assert_eq!(
            Type::from(AstType::Array(AstScalar::DateTime)),
            Type::NonNull(Box::new(Type::List(Box::new(Type::Name(
                "DateTime".to_string()
            )))))
        );

        assert_eq!(
            Type::from(AstType::Array(AstScalar::Float)),
            Type::NonNull(Box::new(Type::List(Box::new(Type::Name(
                "Float".to_string()
            )))))
        );

        assert_eq!(
            Type::from(AstType::Array(AstScalar::Int)),
            Type::NonNull(Box::new(Type::List(Box::new(Type::Name(
                "Int".to_string()
            )))))
        );

        assert_eq!(
            Type::from(AstType::Array(AstScalar::String)),
            Type::NonNull(Box::new(Type::List(Box::new(Type::Name(
                "String".to_string()
            )))))
        );

        assert_eq!(
            Type::from(AstType::Array(AstScalar::Reference("Foo".to_string()))),
            Type::NonNull(Box::new(Type::List(Box::new(Type::Name(
                "Foo".to_string()
            )))))
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(
            format!("{}", Type::from(AstType::Scalar(AstScalar::Boolean))),
            "Boolean!"
        );

        assert_eq!(
            format!("{}", Type::from(AstType::Scalar(AstScalar::DateTime))),
            "DateTime!"
        );

        assert_eq!(
            format!("{}", Type::from(AstType::Scalar(AstScalar::Float))),
            "Float!"
        );

        assert_eq!(
            format!("{}", Type::from(AstType::Scalar(AstScalar::Int))),
            "Int!"
        );

        assert_eq!(
            format!("{}", Type::from(AstType::Scalar(AstScalar::String))),
            "String!"
        );

        assert_eq!(
            format!(
                "{}",
                Type::from(AstType::Scalar(AstScalar::Reference(
                    "Foo".to_string()
                )))
            ),
            "Foo!"
        );

        assert_eq!(
            format!("{}", Type::from(AstType::Array(AstScalar::Boolean))),
            "[Boolean]!"
        );

        assert_eq!(
            format!("{}", Type::from(AstType::Array(AstScalar::DateTime))),
            "[DateTime]!"
        );

        assert_eq!(
            format!("{}", Type::from(AstType::Array(AstScalar::Float))),
            "[Float]!"
        );

        assert_eq!(
            format!("{}", Type::from(AstType::Array(AstScalar::Int))),
            "[Int]!"
        );

        assert_eq!(
            format!("{}", Type::from(AstType::Array(AstScalar::String))),
            "[String]!"
        );

        assert_eq!(
            format!(
                "{}",
                Type::from(AstType::Array(AstScalar::Reference(
                    "Foo".to_string()
                )))
            ),
            "[Foo]!"
        );
    }
}
