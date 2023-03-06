use std::{
    collections::VecDeque,
    fmt::Display,
};

/// A path to a field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Path(pub VecDeque<String>);

impl Path {
    /// Create a new path from the given segments.
    ///
    /// # Arguments
    ///
    /// * `segments` - The segments of the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryPath;
    ///
    /// let path = QueryPath::new(&["foo", "bar"]);
    ///
    /// assert_eq!(path.to_string(), "foo.bar");
    /// ```
    #[must_use]
    pub fn new(segments: &[&str]) -> Self {
        Self(segments.iter().map(ToString::to_string).collect())
    }

    /// Pop the first segment off the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryPath;
    ///
    /// let mut path = QueryPath::new(&["foo", "bar"]);
    ///
    /// assert_eq!(path.pop_front(), Some("foo".to_owned()));
    /// assert_eq!(path.pop_front(), Some("bar".to_owned()));
    /// assert_eq!(path.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<String> {
        self.0.pop_front()
    }

    /// Pop the last segment off the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryPath;
    ///
    /// let mut path = QueryPath::new(&["foo", "bar"]);
    ///
    /// assert_eq!(path.pop_back(), Some("bar".to_owned()));
    /// assert_eq!(path.pop_back(), Some("foo".to_owned()));
    /// assert_eq!(path.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<String> {
        self.0.pop_back()
    }

    /// Remove the last segment from the path without returning it.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryPath;
    ///
    /// let mut path = QueryPath::new(&["foo", "bar"]);
    ///
    /// path.drop_back();
    ///
    /// assert_eq!(path.to_string(), "foo");
    /// ```
    pub fn drop_back(&mut self) {
        let _: Option<String> = self.pop_back();
    }

    /// Push a segment onto the path.
    ///
    /// # Arguments
    ///
    /// * `segment` - The segment to push onto the path.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryPath;
    ///
    /// let mut path = QueryPath::new(&["foo"]);
    ///
    /// path.push("bar".to_owned());
    ///
    /// assert_eq!(path.to_string(), "foo.bar");
    /// ```
    pub fn push(
        &mut self,
        segment: String,
    ) {
        self.0.push_back(segment);
    }

    /// Check if the path is empty.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::QueryPath;
    ///
    /// assert!(!QueryPath::new(&["foo", "bar"]).is_empty());
    /// assert!(QueryPath::new(&[]).is_empty());
    /// ```
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Path {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(
            f,
            "{}",
            self.clone()
                .0
                .into_iter()
                .collect::<Vec<String>>()
                .join(".")
        )
    }
}
