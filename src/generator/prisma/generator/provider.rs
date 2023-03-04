use std::fmt::Display;

/// A generator provider.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub enum Provider {
    /// A file path.
    File(String),
    /// The standard Prisma client.
    #[default]
    PrismaClientJs,
}

impl Display for Provider {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "\"{}\"",
            match self {
                Self::File(path) => path,
                Self::PrismaClientJs => "prisma-client-js",
            }
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default() {
        assert_eq!(Provider::default(), Provider::PrismaClientJs);
    }

    #[test]
    fn test_display_file() {
        assert_eq!(
            Provider::File("path/to/file".to_owned()).to_string(),
            "\"path/to/file\""
        );
    }

    #[test]
    fn test_display_prisma_client_js() {
        assert_eq!(
            Provider::PrismaClientJs.to_string(),
            "\"prisma-client-js\""
        );
    }
}
