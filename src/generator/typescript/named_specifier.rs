use std::fmt::Display;

/// A named import specifier.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
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

impl NamedSpecifier {
    /// Create a new aliased name.
    ///
    /// # Arguments
    ///
    /// * `alias` - The local name of the import.
    /// * `identifier` - The exported name of the import.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::NamedSpecifier;
    ///
    /// let specifier = NamedSpecifier::aliased_name("foo", "bar");
    ///
    /// assert_eq!(
    ///     specifier,
    ///     NamedSpecifier::AliasedName {
    ///         alias: "foo".to_owned(),
    ///         identifier: "bar".to_owned(),
    ///     }
    /// );
    /// ```
    #[must_use]
    pub fn aliased_name(
        alias: &str,
        identifier: &str,
    ) -> Self {
        Self::AliasedName {
            alias: alias.to_owned(),
            identifier: identifier.to_owned(),
        }
    }

    /// Create a new name.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The exported name of the import.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::typescript::NamedSpecifier;
    ///
    /// let specifier = NamedSpecifier::name("foo");
    ///
    /// assert_eq!(specifier, NamedSpecifier::Name("foo".to_owned()));
    /// ```
    #[must_use]
    pub fn name(identifier: &str) -> Self {
        Self::Name(identifier.to_owned())
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_named_specifier() {
        let specifier = NamedSpecifier::AliasedName {
            alias: "foo".to_owned(),
            identifier: "bar".to_owned(),
        };

        assert_eq!(specifier.to_string(), "bar as foo");

        let specifier = NamedSpecifier::Name("foo".to_owned());

        assert_eq!(specifier.to_string(), "foo");
    }
}
