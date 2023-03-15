use {
    print::PrintInline,
    std::{
        fmt::Display,
        io,
    },
};

/// An engine type.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl PrintInline for EngineType {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{self}")
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
    fn test_print_binary() {
        let engine_type = EngineType::Binary;
        let mut f = Vec::new();

        engine_type.print(&mut f).unwrap();

        assert_eq!(f, b"\"binary\"");
    }

    #[test]
    fn test_print_library() {
        let engine_type = EngineType::Library;
        let mut f = Vec::new();

        engine_type.print(&mut f).unwrap();

        assert_eq!(f, b"\"library\"");
    }
}
