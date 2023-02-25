use {
    super::TypeError,
    crate::parser::{
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
    },
    std::collections::HashSet,
};

/// A route describes access to a component.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Route {
    /// The path of the route.
    pub path: String,
    /// The root component of the route.
    pub root: String,
    /// The title of the page that is rendered.
    pub title: String,
}

impl Route {
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
    ///     Ok(("Foo".to_owned(), "".to_owned()))
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
    ///     Ok(("Foo".to_owned(), "".to_owned()))
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
    /// let input = "route /foo/bar {
    ///   root: Foo
    ///   title: Foobar
    /// }";
    ///
    /// assert_eq!(
    ///     Route::parse(input),
    ///     Ok((
    ///         Route {
    ///             path: "/foo/bar".to_owned(),
    ///             root: "Foo".to_owned(),
    ///             title: "Foobar".to_owned(),
    ///         },
    ///         "".to_owned()
    ///     ))
    /// );
    /// ```
    ///
    /// Order of `root` and `title` does not matter.
    ///
    /// ```rust
    /// use dragonfly::ast::Route;
    ///
    /// let input1 = "route / {
    ///   root: Index
    ///   title: Home
    /// }";
    ///
    /// let input2 = "route / {
    ///   title: Home
    ///   root: Index
    /// }";
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

    /// Check whether the root references a known component.
    ///
    /// # Arguments
    ///
    /// * `components` - The components to check against.
    ///
    /// # Errors
    ///
    /// Returns `TypeError::UnknownComponent` if the root does not reference a
    /// known component.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Route;
    ///
    /// let route = Route {
    ///     path: "/".to_owned(),
    ///     root: "Index".to_owned(),
    ///     title: "Home".to_owned(),
    /// };
    ///
    /// let components = vec!["Index".to_owned()].into_iter().collect();
    ///
    /// assert!(route.check_root(&components).is_ok());
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Route,
    ///     TypeError,
    /// };
    ///
    /// let route = Route {
    ///     path: "/".to_owned(),
    ///     root: "Index".to_owned(),
    ///     title: "Home".to_owned(),
    /// };
    ///
    /// let components = vec!["Home".to_owned()].into_iter().collect();
    ///
    /// assert_eq!(
    ///     route.check_root(&components),
    ///     Err(TypeError::UnknownRouteRoot {
    ///         root: "Index".to_owned(),
    ///         route_name: "/".to_owned(),
    ///     })
    /// );
    /// ```
    pub fn check_root(
        &self,
        components: &HashSet<String>,
    ) -> Result<(), TypeError> {
        if !components.contains(&self.root) {
            return Err(TypeError::UnknownRouteRoot {
                root: self.root.clone(),
                route_name: self.path.clone(),
            });
        }

        Ok(())
    }
}
