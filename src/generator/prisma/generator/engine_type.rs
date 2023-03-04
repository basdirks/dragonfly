use std::fmt::Display;

/// An engine type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
pub enum EngineType {
    /// A library.
    #[default]
    Library,
    /// A binary.
    Binary,
}

impl Display for EngineType {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::Library => "library",
                Self::Binary => "binary",
            }
        )
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(EngineType::default(), EngineType::Library);
    }

    #[test]
    fn test_display() {
        assert_eq!(EngineType::Binary.to_string(), "\"binary\"");
        assert_eq!(EngineType::Library.to_string(), "\"library\"");
    }
}
