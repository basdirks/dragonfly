use crate::parser::{
    brace_close,
    brace_open,
    colon,
    forward_slash,
    kebab_case,
    literal,
    many,
    pascal_case,
    spaces,
    ParseResult,
};

/// A JSX component.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Component {
    /// The name of the component.
    pub name: String,
    /// The path to the file that exports the component.
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
    /// Returns `ParseError` if the input does not start with a valid component.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Component;
    ///
    /// let input = "component Foo {
    ///    path: foo/bar/Foo
    /// }";
    ///
    /// let expected = Component {
    ///     name: "Foo".to_string(),
    ///     path: "foo/bar/Foo".to_string(),
    /// };
    ///
    /// assert_eq!(Component::parse(input), Ok((expected, "".to_string())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::component::Component;
    ///
    /// let input = "component Foo {
    ///    path: Foo
    /// }";
    ///
    /// let expected = Component {
    ///     name: "Foo".to_string(),
    ///     path: "Foo".to_string(),
    /// };
    ///
    /// assert_eq!(Component::parse(input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "component")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = pascal_case(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = literal(&input, "path")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;

        let (segments, input) = many(&input, |input| {
            let (segment, input) = kebab_case(input)?;
            let (_, input) = forward_slash(&input)?;

            Ok((segment, input))
        })?;

        let (file_name, input) = pascal_case(&input)?;

        let component = Self {
            name,
            path: if segments.is_empty() {
                file_name
            } else {
                format!(
                    "{}/{file_name}",
                    segments.join("/"),
                    file_name = file_name
                )
            },
        };

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((component, input))
    }
}
