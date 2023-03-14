use {
    super::NamedSpecifier,
    printer::{
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

impl<'a> Import<'a> {
    /// Create a new named import.
    ///
    /// # Arguments
    ///
    /// * `module` - The module to import from.
    /// * `specifiers` - The import specifiers.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use typescript::Import;
    ///
    /// let import = Import::named("foo", []);
    ///
    /// assert_eq!(
    ///     import,
    ///     Import::Named {
    ///         module: "foo".into(),
    ///         specifiers: Vec::new(),
    ///     }
    /// );
    /// ```
    #[must_use]
    pub fn named<S, T>(
        module: S,
        specifiers: T,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Vec<NamedSpecifier<'a>>>,
    {
        Self::Named {
            module: module.into(),
            specifiers: specifiers.into(),
        }
    }

    /// Create a new star import.
    ///
    /// # Arguments
    ///
    /// * `module` - The module to import from.
    /// * `alias` - The local name of the import.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use typescript::Import;
    ///
    /// let import = Import::star("foo", "bar");
    ///
    /// assert_eq!(
    ///     import,
    ///     Import::Star {
    ///         module: "foo".into(),
    ///         alias: "bar".into(),
    ///     }
    /// );
    /// ```
    #[must_use]
    pub fn star<S, T>(
        module: S,
        alias: T,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::Star {
            module: module.into(),
            alias: alias.into(),
        }
    }

    /// Create a new default import.
    ///
    /// # Arguments
    ///
    /// * `module` - The module to import from.
    /// * `alias` - The local name of the import.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use typescript::Import;
    ///
    /// let import = Import::default("foo", "bar");
    ///
    /// assert_eq!(
    ///     import,
    ///     Import::Default {
    ///         module: "foo".into(),
    ///         alias: "bar".into(),
    ///     }
    /// );
    /// ```
    #[must_use]
    pub fn default<S, T>(
        module: S,
        alias: T,
    ) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: Into<Cow<'a, str>>,
    {
        Self::Default {
            module: module.into(),
            alias: alias.into(),
        }
    }
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
        let import = Import::named(
            "foo",
            [
                NamedSpecifier::AliasedName {
                    alias: "foo".into(),
                    identifier: "bar".into(),
                },
                NamedSpecifier::Name {
                    identifier: "baz".into(),
                },
            ],
        );

        let mut f = Vec::new();

        import.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "import { bar as foo, baz } from \"foo\";"
        );
    }

    #[test]
    fn test_print_star() {
        let import = Import::star("foo", "bar");
        let mut f = Vec::new();

        import.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "import * as bar from \"foo\";"
        );
    }

    #[test]
    fn test_print_default() {
        let import = Import::default("foo", "bar");
        let mut f = Vec::new();

        import.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "import bar from \"foo\";");
    }
}
