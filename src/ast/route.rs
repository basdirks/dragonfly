use crate::parser::{
    alphabetics,
    brace_close,
    brace_open,
    chars_if,
    colon,
    literal,
    many_once,
    pascal_case,
    spaces,
    ParseResult,
};

/// A route describes access to a component.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Route {
    /// The path of the route.
    pub path: String,
    /// The root component of the route.
    pub root: String,
    /// The title of the page of the route.
    pub title: String,
}

impl Route {
    /// Create a new route.
    ///
    /// # Arguments
    ///
    /// * `path` - The path of the route.
    /// * `root` - The root component of the route.
    /// * `title` - The title of the page of the route.
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

    /// Parse the root from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid root.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Route,
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(
    ///     Route::parse_root("root: Foo"),
    ///     Ok(("Foo".to_owned(), String::new()))
    /// );
    ///
    /// assert_eq!(
    ///     Route::parse_root("root Foo"),
    ///     Err(ParseError::UnexpectedChar {
    ///         message: "Expected character ':', found ' '.".to_owned(),
    ///         actual: ' '
    ///     })
    /// );
    ///
    /// assert_eq!(
    ///     Route::parse_root("component: Foo"),
    ///     Err(ParseError::UnmatchedLiteral {
    ///         expected: "root".to_owned()
    ///     })
    /// );
    /// ```
    pub fn parse_root(input: &str) -> ParseResult<String> {
        let (_, input) = spaces(input)?;
        let (_, input) = literal(&input, "root")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (root, input) = pascal_case(&input)?;
        let (_, input) = spaces(&input)?;

        Ok((root, input))
    }

    /// Parse the title from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::Route,
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(
    ///     Route::parse_title("title: Foo"),
    ///     Ok(("Foo".to_owned(), String::new()))
    /// );
    ///
    /// assert_eq!(
    ///     Route::parse_title("title Foo"),
    ///     Err(ParseError::UnexpectedChar {
    ///         message: "Expected character ':', found ' '.".to_owned(),
    ///         actual: ' '
    ///     })
    /// );
    ///
    /// assert_eq!(
    ///     Route::parse_title("name: Foo"),
    ///     Err(ParseError::UnmatchedLiteral {
    ///         expected: "title".to_owned()
    ///     })
    /// );
    /// ```
    pub fn parse_title(input: &str) -> ParseResult<String> {
        let (_, input) = literal(input, "title")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (title, input) = alphabetics(&input)?;

        Ok((title, input))
    }

    /// Parse a route from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Route;
    ///
    /// let input = "
    ///
    /// route /foo/bar {
    ///   root: Foo
    ///   title: Foobar
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(
    ///     Route::parse(input),
    ///     Ok((
    ///         Route {
    ///             path: "/foo/bar".to_owned(),
    ///             root: "Foo".to_owned(),
    ///             title: "Foobar".to_owned(),
    ///         },
    ///         String::new()
    ///     ))
    /// );
    /// ```
    ///
    /// Order of `root` and `title` does not matter.
    ///
    /// ```rust
    /// use dragonfly::ast::Route;
    ///
    /// let input1 = "
    ///
    /// route / {
    ///   root: Index
    ///   title: Home
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let input2 = "
    ///
    /// route / {
    ///   title: Home
    ///   root: Index
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// assert_eq!(Route::parse(input1), Route::parse(input2));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "route")?;
        let (_, input) = spaces(&input)?;

        let (path, input) = chars_if(
            &input,
            |c| c.is_ascii_alphanumeric() || c == '/',
            "Should be alphanumeric or '/'.",
        )?;

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;

        let (results, input) =
            many_once(&input, &[Self::parse_root, Self::parse_title])?;

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((
            Self {
                path,
                root: results[0].clone(),
                title: results[1].clone(),
            },
            input,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let route = Route::new("/foo/bar", "Foo", "Foobar");

        assert_eq!(route.path, "/foo/bar");
        assert_eq!(route.root, "Foo");
        assert_eq!(route.title, "Foobar");
    }
}
