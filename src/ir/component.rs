use {
    crate::ast,
    std::path::PathBuf,
};

/// A component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Component {
    /// The name of the component.
    pub name: String,
    /// The path to the component source file.
    pub path: PathBuf,
}

impl From<ast::Component> for Component {
    fn from(value: ast::Component) -> Self {
        Self {
            name: value.name,
            path: value.path,
        }
    }
}

impl Component {
    /// Create a new component.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the component.
    /// * `path` - The path to the component source file.
    #[must_use]
    pub fn new(
        name: &str,
        path: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            path: path.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from() {
        let ast = ast::Component::new("foo", "foo/bar/baz");
        let expected = Component::new("foo", "foo/bar/baz");
        let actual = Component::from(ast);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_new() {
        assert_eq!(
            Component::new("foo", "foo/bar/baz"),
            Component {
                name: "foo".to_owned(),
                path: "foo/bar/baz".into()
            }
        );
    }
}
