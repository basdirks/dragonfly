use {
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A named import specifier.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum NamedSpecifier<'a> {
    /// An import of a named export with an alias.
    AliasedName {
        /// The local name of the import.
        alias: Cow<'a, str>,
        /// The exported name of the import.
        identifier: Cow<'a, str>,
    },
    /// An import of a named export.
    Name {
        /// The exported name of the import.
        identifier: Cow<'a, str>,
    },
}

impl PrintInline for NamedSpecifier<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        match self {
            Self::AliasedName { alias, identifier } => {
                write!(f, "{identifier} as {alias}")
            }
            Self::Name { identifier } => write!(f, "{identifier}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_aliased_name() {
        let specifier = NamedSpecifier::AliasedName {
            alias: "foo".into(),
            identifier: "bar".into(),
        };

        let mut f = Vec::new();

        specifier.print(&mut f).unwrap();

        assert_eq!(f, b"bar as foo");
    }

    #[test]
    fn test_print_name() {
        let specifier = NamedSpecifier::Name {
            identifier: "foo".into(),
        };

        let mut f = Vec::new();

        specifier.print(&mut f).unwrap();

        assert_eq!(f, b"foo");
    }
}
