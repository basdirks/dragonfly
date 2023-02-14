use crate::parser::{
    case::{
        kebab,
        pascal,
    },
    char::{
        brace_close,
        brace_open,
        colon,
        forward_slash,
    },
    char_range::spaces,
    literal,
    many,
    ParseResult,
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
        let (name, input) = pascal(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = literal(&input, "path")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;

        let (segments, input) = many(&input, |input| {
            let (segment, input) = kebab(input)?;
            let (_, input) = forward_slash(&input)?;

            Ok((segment, input))
        })?;

        let (file_name, input) = pascal(&input)?;

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
