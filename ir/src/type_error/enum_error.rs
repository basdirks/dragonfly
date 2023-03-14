use std::{
    error::Error,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

/// Errors that can occur when type checking an enum.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum EnumError {
    /// The name of an enum must be unique within the application. This enum
    /// has the same name as another enum.
    Duplicate,
}

impl Display for EnumError {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(f, "enum already exists")
    }
}

impl Error for EnumError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display() {
        assert_eq!(EnumError::Duplicate.to_string(), "enum already exists");
    }

    #[test]
    fn test_source() {
        assert!(EnumError::Duplicate.source().is_none());
    }
}
