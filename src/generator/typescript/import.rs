use {
    super::NamedSpecifier,
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// An import declaration.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Import {
    /// A named import.
    Named {
        /// The module to import from.
        module: String,
        /// The import specifiers.
        specifiers: Vec<NamedSpecifier>,
    },
    /// An import of all named exports as a single object.
    Star {
        /// The module to import from.
        module: String,
        /// The local name of the import.
        alias: String,
    },
    /// A default import.
    Default {
        /// The module to import from.
        module: String,
        /// The local name of the import.
        alias: String,
    },
}

impl Import {
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
    /// use dragonfly::generator::typescript::Import;
    ///
    /// let import = Import::named("foo", &[]);
    ///
    /// assert_eq!(import.module, "foo".to_owned());
    /// assert!(import.specifiers.is_empty());
    /// ```
    #[must_use]
    pub fn named(
        module: &str,
        specifiers: &[NamedSpecifier],
    ) -> Self {
        Self::Named {
            module: module.to_owned(),
            specifiers: specifiers.iter().map(ToOwned::to_owned).collect(),
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
    /// use dragonfly::generator::typescript::Import;
    ///
    /// let import = Import::star("foo", "bar");
    ///
    /// assert_eq!(import.module, "foo".to_owned());
    /// assert_eq!(import.alias, "bar".to_owned());
    /// ```
    #[must_use]
    pub fn star(
        module: &str,
        alias: &str,
    ) -> Self {
        Self::Star {
            module: module.to_owned(),
            alias: alias.to_owned(),
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
    /// use dragonfly::generator::typescript::Import;
    ///
    /// let import = Import::default("foo", "bar");
    ///
    /// assert_eq!(import.module, "foo".to_owned());
    /// assert_eq!(import.alias, "bar".to_owned());
    /// ```
    #[must_use]
    pub fn default(
        module: &str,
        alias: &str,
    ) -> Self {
        Self::Default {
            module: module.to_owned(),
            alias: alias.to_owned(),
        }
    }
}

impl Display for Import {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Named { module, specifiers } => {
                let specifiers = comma_separated(specifiers);

                write!(f, "import {{ {specifiers} }} from \"{module}\";")
            }
            Self::Star { module, alias } => {
                write!(f, "import * as {alias} from \"{module}\";")
            }
            Self::Default { module, alias } => {
                write!(f, "import {alias} from \"{module}\";")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_named_import() {
        let import = Import::named(
            "foo",
            &[
                NamedSpecifier::aliased_name("foo", "bar"),
                NamedSpecifier::name("baz"),
            ],
        );

        assert_eq!(
            import.to_string(),
            "import { bar as foo, baz } from \"foo\";"
        );
    }

    #[test]
    fn test_display_star_import() {
        let import = Import::star("foo", "bar");

        assert_eq!(import.to_string(), "import * as bar from \"foo\";");
    }

    #[test]
    fn test_display_default_import() {
        let import = Import::default("foo", "bar");

        assert_eq!(import.to_string(), "import bar from \"foo\";");
    }
}
