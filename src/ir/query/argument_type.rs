use crate::ir::Type;

/// The type of an argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ArgumentType {
    /// A reference to an enum.
    Enum(String),
    /// A scalar type.
    Type(Type),
}

impl ArgumentType {
    /// Create a new boolean argument type.
    #[must_use]
    pub const fn boolean() -> Self {
        Self::Type(Type::Boolean)
    }

    /// Create a new float argument type.
    #[must_use]
    pub const fn float() -> Self {
        Self::Type(Type::Float)
    }

    /// Create a new integer argument type.
    #[must_use]
    pub const fn int() -> Self {
        Self::Type(Type::Int)
    }

    /// Create a new string argument type.
    #[must_use]
    pub const fn string() -> Self {
        Self::Type(Type::String)
    }

    /// Create a new date time argument type.
    #[must_use]
    pub const fn date_time() -> Self {
        Self::Type(Type::DateTime)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boolean() {
        assert_eq!(ArgumentType::boolean(), ArgumentType::Type(Type::Boolean));
    }

    #[test]
    fn test_float() {
        assert_eq!(ArgumentType::float(), ArgumentType::Type(Type::Float));
    }

    #[test]
    fn test_int() {
        assert_eq!(ArgumentType::int(), ArgumentType::Type(Type::Int));
    }

    #[test]
    fn test_string() {
        assert_eq!(ArgumentType::string(), ArgumentType::Type(Type::String));
    }

    #[test]
    fn test_date_time() {
        assert_eq!(
            ArgumentType::date_time(),
            ArgumentType::Type(Type::DateTime)
        );
    }
}
