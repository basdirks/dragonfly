use crate::parser::{
    alphabetics,
    brace_close,
    brace_open,
    chars_if,
    choice,
    colon,
    literal,
    pascal_case,
    spaces,
    ParseResult,
};

/// A route describes access to a component.
#[derive(Clone, Debug, Eq, PartialEq)]
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
    /// Returns a `ParseError` if the input does not start with a valid root.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Route;
    ///
    /// assert_eq!(
    ///     Route::parse_root("root: Foo"),
    ///     Ok(("Foo".to_string(), "".to_string()))
    /// );
    ///
    /// assert_eq!(
    ///     Route::parse_root("root Foo"),
    ///     Err(ParseError::UnmatchedChar {
    ///         expected: ':',
    ///         found: ' '
    ///     })
    /// );
    ///
    /// assert_eq!(
    ///     Route::parse_root("component: Foo"),
    ///     Err(ParseError::UnmatchedLiteral { expected: "root" })
    /// );
    /// ```
    pub fn parse_root(input: &str) -> ParseResult<String> {
        let (_, input) = literal(input, "root")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (root, input) = pascal_case(&input)?;

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
    /// Returns a `ParseError` if the input does not start with a valid title.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Route;
    ///
    /// assert_eq!(
    ///     Route::parse_title("title: Foo"),
    ///     Ok(("Foo".to_string(), "".to_string()))
    /// );
    ///
    /// assert_eq!(
    ///     Route::parse_title("title Foo"),
    ///     Err(ParseError::UnmatchedChar {
    ///         expected: ':',
    ///         found: ' '
    ///     })
    /// );
    ///
    /// assert_eq!(
    ///     Route::parse_title("name: Foo"),
    ///     Err(ParseError::UnmatchedLiteral { expected: "title" })
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
    /// Returns a `ParseError` if the input does not start with a valid route.
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
    ///             path: "/foo/bar".to_string(),
    ///             root: "Foo".to_string(),
    ///             title: "Foobar".to_string(),
    ///         },
    ///         "".to_string()
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

        // TODO: Replace with `path` parser.
        let (path, input) = chars_if(
            &input,
            |c| c.is_ascii_alphanumeric() || c == '/',
            "should be alphanumeric or '/'",
        )?;

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;

        // TODO: Replace with variant of `choice` that applies each parser
        // exactly once, regardless of order.
        let ((root, title), input) = choice(
            &input,
            vec![
                |input| {
                    let (root, input) = Self::parse_root(input)?;
                    let (_, input) = spaces(&input)?;
                    let (title, input) = Self::parse_title(&input)?;
                    Ok(((root, title), input))
                },
                |input| {
                    let (title, input) = Self::parse_title(input)?;
                    let (_, input) = spaces(&input)?;
                    let (root, input) = Self::parse_root(&input)?;
                    Ok(((root, title), input))
                },
            ],
        )?;

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { path, root, title }, input))
    }
}
