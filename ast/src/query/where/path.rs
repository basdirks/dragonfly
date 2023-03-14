use std::{
    borrow::Cow,
    collections::VecDeque,
    fmt::{
        self,
        Display,
        Formatter,
    },
};

/// A path to a field.
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Path<'a>(pub VecDeque<Cow<'a, str>>);

impl<'a> Path<'a> {
    /// Remove the last segment from the path without returning it.
    pub fn drop_back(&mut self) {
        let _: Option<Cow<'a, str>> = self.0.pop_back();
    }

    /// Push a segment onto the path.
    ///
    /// # Arguments
    ///
    /// * `segment` - The segment to push onto the path.
    pub fn push<S>(
        &mut self,
        segment: S,
    ) where
        S: Into<Cow<'a, str>>,
    {
        self.0.push_back(segment.into());
    }

    /// Check if the path is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Display for Path<'_> {
    fn fmt(
        &self,
        f: &mut Formatter<'_>,
    ) -> fmt::Result {
        write!(
            f,
            "{}",
            self.clone().0.into_iter().collect::<Vec<_>>().join(".")
        )
    }
}

impl<'a, S> FromIterator<S> for Path<'a>
where
    S: Into<Cow<'a, str>>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        S: Into<Cow<'a, str>>,
        T: IntoIterator<Item = S>,
    {
        Self(iter.into_iter().map(Into::into).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_drop_back() {
        let mut path = Path::from_iter(["foo", "bar"]);

        path.drop_back();

        assert_eq!(path.to_string(), "foo");
    }

    #[test]
    fn test_push() {
        let mut path = Path::from_iter(["foo"]);

        path.push("bar".to_owned());

        assert_eq!(path.to_string(), "foo.bar");
    }

    #[test]
    fn test_is_empty() {
        assert!(!Path::from_iter(["foo", "bar"]).is_empty());
        assert!(Path::default().is_empty());
    }

    #[test]
    fn test_display() {
        assert_eq!(Path::from_iter(["foo", "bar"]).to_string(), "foo.bar");
    }
}
