use std::fmt::Display;

/// GraphQL types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    /// A list type.
    List(Box<Type>),
    /// A non-null type.
    NonNull(Box<Type>),
    /// A name.
    Name(String),
}

impl Type {
    /// Create a new list type.
    ///
    /// # Arguments
    ///
    /// * `inner` - The inner type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Type;
    ///
    /// let list = Type::list(Type::name("String"));
    ///
    /// assert_eq!(list, Type::List(Box::new(Type::Name("String".to_owned()))));
    /// ```
    #[must_use]
    pub fn list(inner: Self) -> Self {
        Self::List(Box::new(inner))
    }

    /// Create a new non-null type.
    ///
    /// # Arguments
    ///
    /// * `inner` - The inner type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Type;
    ///
    /// let non_null = Type::non_null(Type::name("String"));
    ///
    /// assert_eq!(
    ///     non_null,
    ///     Type::NonNull(Box::new(Type::Name("String".to_owned())))
    /// );
    /// ```
    #[must_use]
    pub fn non_null(inner: Self) -> Self {
        Self::NonNull(Box::new(inner))
    }

    /// Create a new name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::graphql::Type;
    ///
    /// let name = Type::name("String");
    ///
    /// assert_eq!(name, Type::Name("String".to_owned()));
    /// ```
    #[must_use]
    pub fn name(name: &str) -> Self {
        Self::Name(name.to_owned())
    }
}

impl Display for Type {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::List(inner) => write!(f, "[{inner}]"),
            Self::NonNull(inner) => write!(f, "{inner}!"),
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}
