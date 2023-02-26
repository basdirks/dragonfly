use super::{
    alphabetics,
    choice,
    digit,
    hyphen,
    lowercase,
    many,
    many1,
    underscore,
    uppercase,
    ParseError,
    ParseResult,
};

/// Parse a capitalized alphabetic ASCII identifier.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// Returns `ParseError::UnexpectedChar` if the first character is not
/// uppercase.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     capitalized,
///     ParseError,
/// };
///
/// assert_eq!(capitalized("Foo"), Ok(("Foo".to_owned(), "".to_owned())));
/// assert_eq!(capitalized("F0o"), Ok(("F".to_owned(), "0o".to_owned())));
///
/// assert_eq!(
///     capitalized("Foo Bar"),
///     Ok(("Foo".to_owned(), " Bar".to_owned()))
/// );
///
/// assert_eq!(
///     capitalized("FooBar"),
///     Ok(("FooBar".to_owned(), "".to_owned()))
/// );
///
/// assert_eq!(
///     capitalized("foo"),
///     Err(ParseError::UnexpectedChar {
///         actual: 'f',
///         message: "Expected capitalized identifier to start with uppercase \
///                   character, found 'f'."
///             .to_string(),
///     })
/// );
/// ```
pub fn capitalized(input: &str) -> ParseResult<String> {
    let (head, input) = uppercase(input).map_err(|_| {
        let actual = input.chars().next().map_or('\0', |c| c);

        ParseError::UnexpectedChar {
            actual,
            message: format!(
                "Expected capitalized identifier to start with uppercase \
                 character, found '{actual}'."
            ),
        }
    })?;

    let (tail, input) = many(&input, alphabetics)?;

    Ok((
        format!("{head}{}", tail.into_iter().collect::<String>()),
        input,
    ))
}

/// Parses a pascal case ASCII identifier.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the identifier does not start with
///   an uppercase character.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     pascal_case,
///     ParseError,
/// };
///
/// assert_eq!(
///     pascal_case("FooBar"),
///     Ok(("FooBar".to_owned(), "".to_owned()))
/// );
///
/// assert_eq!(
///     pascal_case("foobar"),
///     Err(ParseError::UnexpectedChar {
///         actual: 'f',
///         message: "Expected segment of PascalCase identifier to start with \
///                   uppercase character, found 'f'."
///             .to_string(),
///     })
/// );
///
/// assert_eq!(
///     pascal_case("foo_bar"),
///     Err(ParseError::UnexpectedChar {
///         actual: 'f',
///         message: "Expected segment of PascalCase identifier to start with \
///                   uppercase character, found 'f'."
///             .to_string(),
///     })
/// );
///
/// assert_eq!(
///     pascal_case("foo-bar"),
///     Err(ParseError::UnexpectedChar {
///         actual: 'f',
///         message: "Expected segment of PascalCase identifier to start with \
///                   uppercase character, found 'f'."
///             .to_string(),
///     })
/// );
///
/// assert_eq!(pascal_case(""), Err(ParseError::UnexpectedEof));
/// ```
pub fn pascal(input: &str) -> ParseResult<String> {
    many1(input, capitalized)
        .map(|(parts, input)| (parts.join(""), input))
        .map_err(|e| {
            match e {
                ParseError::UnexpectedChar { .. } => {
                    input.chars().next().map_or(
                        ParseError::UnexpectedEof,
                        |actual| {
                            ParseError::UnexpectedChar {
                                actual,
                                message: format!(
                                    "Expected segment of PascalCase \
                                     identifier to start with uppercase \
                                     character, found '{actual}'."
                                ),
                            }
                        },
                    )
                }
                _ => unreachable!(),
            }
        })
}

/// Parses a kebab case ASCII identifier.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the first character of an
///   identifier segment is not lowercase.
/// * Returns `ParseError::UnexpectedChar` if the last character of an
///   identifier segment is not lowercase.
/// * Returns `ParseError::UnexpectedChar` if two identifier segments are not
///   separated by a hyphen.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     kebab_case,
///     ParseError,
/// };
///
/// assert_eq!(kebab_case("foo"), Ok(("foo".to_owned(), "".to_owned())));
///
/// assert_eq!(
///     kebab_case("foo-bar"),
///     Ok(("foo-bar".to_owned(), "".to_owned()))
/// );
///
/// assert_eq!(
///     kebab_case("foo_bar"),
///     Err(ParseError::UnexpectedChar {
///         message: "Unexpected character at end of kebab-case identifier, \
///                   found '_'."
///             .to_string(),
///         actual: '_',
///     })
/// );
///
/// assert_eq!(
///     kebab_case("foo-Bar"),
///     Err(ParseError::UnexpectedChar {
///         actual: '-',
///         message: "Unexpected character at end of kebab-case identifier, \
///                   found '-'."
///             .to_string(),
///     })
/// );
///
/// assert_eq!(
///     kebab_case("fooBar"),
///     Err(ParseError::UnexpectedChar {
///         actual: 'B',
///         message: "Unexpected character at end of kebab-case identifier, \
///                   found 'B'."
///             .to_string(),
///     })
/// );
///
/// assert_eq!(
///     kebab_case("Foo"),
///     Err(ParseError::UnexpectedChar {
///         actual: 'F',
///         message: "Expected kebab-case identifier to start with a \
///                   lowercase character, found 'F'."
///             .to_string(),
///     })
/// );
///
/// assert_eq!(
///     kebab_case("foo--"),
///     Err(ParseError::UnexpectedChar {
///         actual: '-',
///         message: "Unexpected character at end of kebab-case identifier, \
///                   found '-'."
///             .to_string(),
///     })
/// );
///
/// assert_eq!(kebab_case(""), Err(ParseError::UnexpectedEof));
/// ```
pub fn kebab(input: &str) -> ParseResult<String> {
    let (head, input) = many1(input, lowercase).map_err(|e| {
        match e {
            ParseError::UnexpectedChar { .. } => {
                input.chars().next().map_or(
                    ParseError::UnexpectedEof,
                    |actual| {
                        ParseError::UnexpectedChar {
                            actual,
                            message: format!(
                                "Expected kebab-case identifier to start with \
                                 a lowercase character, found '{actual}'."
                            ),
                        }
                    },
                )
            }
            _ => e,
        }
    })?;

    let (tail, input) = many(&input, |input| {
        let (_, input) = hyphen(input)?;

        let (segment, input) = many1(&input, lowercase).map_err(|e| {
            match e {
                ParseError::UnexpectedChar { .. } => {
                    input.chars().next().map_or(
                        ParseError::UnexpectedEof,
                        |actual| {
                            ParseError::UnexpectedChar {
                                actual,
                                message: format!(
                                    "Expected kebab-case identifier segment \
                                     to start with lowercase character, found \
                                     '{actual}'."
                                ),
                            }
                        },
                    )
                }
                _ => e,
            }
        })?;

        Ok((segment.iter().collect::<String>(), input))
    })?;

    if choice(&input, vec![hyphen, uppercase, digit, underscore]).is_ok() {
        return Err(input.chars().next().map_or(
            ParseError::UnexpectedEof,
            |actual| {
                ParseError::UnexpectedChar {
                    actual,
                    message: format!(
                        "Unexpected character at end of kebab-case \
                         identifier, found '{actual}'."
                    ),
                }
            },
        ));
    }

    Ok((
        if tail.is_empty() {
            head.iter().collect::<String>()
        } else {
            format!("{}-{}", head.iter().collect::<String>(), tail.join("-"))
        },
        input,
    ))
}

/// Parse a camel case ASCII identifier.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if an identifier segment does not
///   start with a lowercase character.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     camel_case,
///     ParseError,
/// };
///
/// assert_eq!(camel_case("foo"), Ok(("foo".to_owned(), "".to_owned())));
///
/// assert_eq!(
///     camel_case("fooBar"),
///     Ok(("fooBar".to_owned(), "".to_owned()))
/// );
///
/// assert_eq!(
///     camel_case("FooBar"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected camelCase identifier to start with lowercase \
///                   character, found 'F'."
///             .to_string(),
///         actual: 'F',
///     })
/// );
///
/// assert_eq!(camel_case(""), Err(ParseError::UnexpectedEof));
/// ```
pub fn camel(input: &str) -> ParseResult<String> {
    let (head, input) = many1(input, lowercase).map_err(|e| {
        match e {
            ParseError::UnexpectedChar { .. } => {
                input.chars().next().map_or(
                    ParseError::UnexpectedEof,
                    |actual| {
                        ParseError::UnexpectedChar {
                            actual,
                            message: format!(
                                "Expected camelCase identifier to start with \
                                 lowercase character, found '{actual}'."
                            ),
                        }
                    },
                )
            }
            _ => e,
        }
    })?;

    let (tail, input) = many(&input, capitalized)?;

    Ok((
        if tail.is_empty() {
            head.iter().collect::<String>()
        } else {
            format!("{}{}", head.iter().collect::<String>(), tail.join(""))
        },
        input,
    ))
}
