use {
    super::NamedSpecifier,
    print::{
        Print,
        PrintInline,
    },
    std::{
        borrow::Cow,
        io,
    },
};

/// An import declaration.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Import<'a> {
    /// A named import.
    Named {
        /// The module to import from.
        module: Cow<'a, str>,
        /// The import specifiers.
        specifiers: Vec<NamedSpecifier<'a>>,
    },
    /// An import of all named exports as a single object.
    Star {
        /// The module to import from.
        module: Cow<'a, str>,
        /// The local name of the import.
        alias: Cow<'a, str>,
    },
    /// A default import.
    Default {
        /// The module to import from.
        module: Cow<'a, str>,
        /// The local name of the import.
        alias: Cow<'a, str>,
    },
}

impl Print for Import<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let indent = Self::indent(level);

        match self {
            Self::Named { module, specifiers } => {
                write!(f, "{indent}import {{ ")?;
                PrintInline::intercalate(specifiers.clone(), f, ", ")?;
                write!(f, " }} from \"{module}\";")
            }
            Self::Star { module, alias } => {
                write!(f, "{indent}import * as {alias} from \"{module}\";")
            }
            Self::Default { module, alias } => {
                write!(f, "{indent}import {alias} from \"{module}\";")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_named() {
        let import = Import::Named {
            module: "foo".into(),
            specifiers: vec![
                NamedSpecifier::AliasedName {
                    alias: "foo".into(),
                    identifier: "bar".into(),
                },
                NamedSpecifier::Name {
                    identifier: "baz".into(),
                },
            ],
        };

        let mut f = Vec::new();

        import.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "import { bar as foo, baz } from \"foo\";"
        );
    }

    #[test]
    fn test_print_star() {
        let import = Import::Star {
            module: "foo".into(),
            alias: "bar".into(),
        };

        let mut f = Vec::new();

        import.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "import * as bar from \"foo\";"
        );
    }

    #[test]
    fn test_print_default() {
        let import = Import::Default {
            module: "foo".into(),
            alias: "bar".into(),
        };

        let mut f = Vec::new();

        import.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "import bar from \"foo\";");
    }
}
