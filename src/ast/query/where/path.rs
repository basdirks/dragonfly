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
    /// use dragonfly::ast::FieldPath;
    ///
    /// let path = FieldPath::new(&["foo", "bar"]);
    ///
    /// assert_eq!(path.to_string(), "foo { bar }");
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
    /// use dragonfly::ast::FieldPath;
    ///
    /// let mut path = FieldPath::new(&["foo", "bar"]);
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
    /// use dragonfly::ast::FieldPath;
    ///
    /// let mut path = FieldPath::new(&["foo", "bar"]);
    ///
    /// assert_eq!(path.pop_back(), Some("bar".to_owned()));
    /// assert_eq!(path.pop_back(), Some("foo".to_owned()));
    /// assert_eq!(path.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<String> {
        self.0.pop_back()
    }

    /// Push a segment onto the path.
    ///
    /// # Arguments
    ///
    /// * `segment` - The segment to push onto the path.
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
    /// use dragonfly::ast::FieldPath;
    ///
    /// assert!(!FieldPath::new(&["foo", "bar"]).is_empty());
    /// assert!(FieldPath::new(&[]).is_empty());
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
        let Self(fields) = self;
        let path_length = fields.len().saturating_sub(1);

        for (index, field) in fields.iter().enumerate() {
            if index > 0 {
                write!(f, " {{ {field}")?;
            } else {
                write!(f, "{field}")?;
            }
        }

        for _ in 0..path_length {
            write!(f, " }}")?;
        }

        Ok(())
    }
}
