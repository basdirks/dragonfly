use crate::parser::{
    brace_close, brace_open, capitalized, char, chars_if, literal, spaces, ParseResult,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Component {
    pub name: String,
    pub path: String,
}

impl Component {
    /// Parse a component from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid component.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::component::Component;
    ///
    /// let input = "component Foo {
    ///    path: /foo
    /// }";
    ///
    /// let expected = Component {
    ///     name: "Foo".to_string(),
    ///     path: "/foo".to_string(),
    /// };
    ///
    /// assert_eq!(Component::parse(input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "component")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = capitalized(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = literal(&input, "path")?;
        let (_, input) = char(&input, ':')?;
        let (_, input) = spaces(&input)?;

        // TODO: Replace with `path` parser.
        let (path, input) = chars_if(
            &input,
            |c| c.is_ascii_alphabetic() || c == '/',
            "should be alphabetic or '/'",
        )?;

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self { name, path }, input))
    }
}
