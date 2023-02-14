use crate::parser::{
    case::pascal,
    char::{
        brace_close,
        brace_open,
        colon,
    },
    char_range::{
        alphabetics,
        chars_if,
        spaces,
    },
    choice,
    literal,
    ParseResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Route {
    pub path: String,
    pub root: String,
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
    /// * If the input is not a valid root.
    pub fn parse_root(input: &str) -> ParseResult<String> {
        let (_, input) = literal(input, "root")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (root, input) = pascal(&input)?;

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
    /// * If the input is not a valid title.
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
    /// * If the input is not a valid route.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::route::Route;
    ///
    /// let input = "route /foo {
    ///    root: Foo
    ///    title: Foobar
    /// }";
    ///
    /// assert_eq!(
    ///     Route::parse(input),
    ///     Ok((
    ///         Route {
    ///             path: "/foo".to_string(),
    ///             root: "Foo".to_string(),
    ///             title: "Foobar".to_string(),
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::route::Route;
    ///
    /// let input = "route / {
    ///   root: Index
    ///   title: Home
    /// }";
    ///
    /// assert_eq!(
    ///     Route::parse(input),
    ///     Ok((
    ///         Route {
    ///             path: "/".to_string(),
    ///             root: "Index".to_string(),
    ///             title: "Home".to_string(),
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::route::Route;
    ///
    /// let input = "route / {
    ///   title: Home
    ///   root: Index
    /// }";
    ///
    /// assert_eq!(
    ///     Route::parse(input),
    ///     Ok((
    ///         Route {
    ///             path: "/".to_string(),
    ///             root: "Index".to_string(),
    ///             title: "Home".to_string(),
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
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
