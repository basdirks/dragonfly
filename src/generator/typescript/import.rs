use {
    crate::generator::printer::common::comma_separated,
    std::fmt::Display,
};

/// A named import specifier.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NamedSpecifier {
    /// An import of a named export with an alias.
    AliasedName {
        /// The local name of the import.
        alias: String,
        /// The exported name of the import.
        identifier: String,
    },
    /// An import of a named export.
    Name(String),
}

impl Display for NamedSpecifier {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::AliasedName { alias, identifier } => {
                write!(f, "{identifier} as {alias}")
            }
            Self::Name(identifier) => write!(f, "{identifier}"),
        }
    }
}

/// An import declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    fn display_named_specifier() {
        let specifier = NamedSpecifier::AliasedName {
            alias: "foo".to_string(),
            identifier: "bar".to_string(),
        };

        assert_eq!(specifier.to_string(), "bar as foo");

        let specifier = NamedSpecifier::Name("foo".to_string());

        assert_eq!(specifier.to_string(), "foo");
    }

    #[test]
    fn named_import() {
        let import = Import::Named {
            module: "foo".to_string(),
            specifiers: vec![
                NamedSpecifier::AliasedName {
                    alias: "foo".to_string(),
                    identifier: "bar".to_string(),
                },
                NamedSpecifier::Name("baz".to_string()),
            ],
        };

        assert_eq!(
            import.to_string(),
            "import { bar as foo, baz } from \"foo\";"
        );
    }

    #[test]
    fn star_import() {
        let import = Import::Star {
            module: "foo".to_string(),
            alias: "bar".to_string(),
        };

        assert_eq!(import.to_string(), "import * as bar from \"foo\";");
    }

    #[test]
    fn default_import() {
        let import = Import::Default {
            module: "foo".to_string(),
            alias: "bar".to_string(),
        };

        assert_eq!(import.to_string(), "import bar from \"foo\";");
    }
}
