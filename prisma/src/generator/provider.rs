use {
    print::PrintInline,
    std::{
        borrow::Cow,
        fmt::Display,
        io,
    },
};

/// A generator provider.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Provider<'a> {
    /// A file path.
    File(Cow<'a, str>),
    /// The standard Prisma client.
    #[default]
    PrismaClientJs,
}

impl Display for Provider<'_> {
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

impl PrintInline for Provider<'_> {
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
        assert_eq!(Provider::default(), Provider::PrismaClientJs);
    }

    #[test]
    fn test_print_file() {
        let provider = Provider::File("path/to/file".into());
        let mut f = Vec::new();

        provider.print(&mut f).unwrap();

        assert_eq!(f, b"\"path/to/file\"");
    }

    #[test]
    fn test_print_prisma_client_js() {
        let provider = Provider::PrismaClientJs;
        let mut f = Vec::new();

        provider.print(&mut f).unwrap();

        assert_eq!(f, b"\"prisma-client-js\"");
    }
}
