use std::fmt::Display;

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
