use crate::ast;

/// A route.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Route {
    /// The path of the route.
    pub path: String,
    /// The root component of the route.
    pub root: String,
    /// The title of the page at the route.
    pub title: String,
}

impl Route {
    /// Create a new route.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the route.
    /// * `root` - The root component of the route.
    /// * `title` - The title of the page at the route.
    #[must_use]
    pub fn new(
        path: &str,
        root: &str,
        title: &str,
    ) -> Self {
        Self {
            path: path.to_owned(),
            root: root.to_owned(),
            title: title.to_owned(),
        }
    }
}

impl From<ast::Route> for Route {
    fn from(value: ast::Route) -> Self {
        Self {
            root: value.root,
            path: value.path,
            title: value.title,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Route::new("foo", "bar", "baz"),
            Route {
                path: "foo".to_owned(),
                root: "bar".to_owned(),
                title: "baz".to_owned(),
            }
        );
    }

    #[test]
    fn test_from() {
        assert_eq!(
            Route::from(ast::Route::new("foo", "bar", "baz")),
            Route::new("foo", "bar", "baz"),
        );
    }
}
