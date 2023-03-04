use {
    crate::parser::{
        alphabetics,
        brace_close,
        brace_open,
        colon,
        forward_slash,
        literal,
        many,
        option,
        pascal_case,
        spaces,
        ParseResult,
    },
    std::path::PathBuf,
};

/// A JSX component.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Component {
    /// The name of the component.
    pub name: String,
    /// The path to the file that exports the component.
    pub path: PathBuf,
}

impl Component {
    /// Create a new component.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the component.
    /// * `path` - The path to the file that exports the component.
    #[must_use]
    pub fn new(
        name: &str,
        path: &str,
    ) -> Self {
        Self {
            name: name.to_owned(),
            path: PathBuf::from(path),
        }
    }

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
    /// let input = "
    ///
    /// component Foo {
    ///    path: /foo/bar/Foo
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Component {
    ///     name: "Foo".to_owned(),
    ///     path: "foo/bar/Foo".to_owned().into(),
    /// };
    ///
    /// assert_eq!(Component::parse(input), Ok((expected, String::new())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::Component;
    ///
    /// let input = "
    ///
    /// component Foo {
    ///    path: Foo
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Component {
    ///     name: "Foo".to_owned(),
    ///     path: "Foo".to_owned().into(),
    /// };
    ///
    /// assert_eq!(Component::parse(input), Ok((expected, String::new())));
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
            let (_, input) = option(input, forward_slash)?;
            let (segment, input) = alphabetics(&input)?;

            Ok((segment, input))
        })?;

        let component = Self {
            name,
            path: segments.join("/").into(),
        };

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((component, input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Component::new("Foo", "foo/bar/Foo"),
            Component {
                name: "Foo".to_owned(),
                path: "foo/bar/Foo".to_owned().into(),
            }
        );
    }

    #[test]
    fn test_parse() {
        let input = "
        
        component Foo {
           path: foo/bar/Foo
        }
        
        "
        .trim();

        let expected = Component {
            name: "Foo".to_owned(),
            path: "foo/bar/Foo".to_owned().into(),
        };

        assert_eq!(Component::parse(input), Ok((expected, String::new())));
    }

    #[test]
    fn test_parse_no_segments() {
        let input = "
        
        component Foo {
           path: Foo
        }
        
        "
        .trim();

        let expected = Component {
            name: "Foo".to_owned(),
            path: "Foo".to_owned().into(),
        };

        assert_eq!(Component::parse(input), Ok((expected, String::new())));
    }
}
