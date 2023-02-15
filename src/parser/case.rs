//! Parse ASCII identifiers with different case styles.
use super::{
    char::{
        hyphen,
        underscore,
    },
    char_range::{
        alphabetics,
        digit,
        lowercase,
        uppercase,
    },
    choice,
    many,
    many1,
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
/// * `ParseError::UnmetPredicate`
/// if the first character is not uppercase.
///
/// * `ParseError`
/// if the parser fails.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     case::capitalized,
///     ParseError,
/// };
///
/// assert_eq!(capitalized("Foo"), Ok(("Foo".to_string(), "".to_string())));
/// assert_eq!(capitalized("F0o"), Ok(("F".to_string(), "0o".to_string())));
///
/// assert_eq!(
///     capitalized("Foo Bar"),
///     Ok(("Foo".to_string(), " Bar".to_string()))
/// );
///
/// assert_eq!(
///     capitalized("FooBar"),
///     Ok(("FooBar".to_string(), "".to_string()))
/// );
///
/// assert_eq!(
///     capitalized("foo"),
///     Err(ParseError::UnmetPredicate {
///         actual: 'f',
///         message: "character is not uppercase".to_string(),
///     })
/// );
/// ```
pub fn capitalized(input: &str) -> ParseResult<String> {
    let (head, input) = uppercase(input)?;
    let (tail, input) = many(&input, alphabetics)?;

    Ok((
        format!("{head}{}", tail.into_iter().collect::<String>()),
        input,
    ))
}

/// Parses a pascal-case ASCII identifier.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// * `ParseError::UnmetPredicate`
/// if the identifier does not start with an uppercase character.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     case::pascal,
///     ParseError,
/// };
///
/// assert_eq!(pascal("FooBar"), Ok(("FooBar".to_string(), "".to_string())));
///
/// assert_eq!(
///     pascal("foobar"),
///     Err(ParseError::UnmetPredicate {
///         actual: 'f',
///         message: "character is not uppercase".to_string(),
///     })
/// );
///
/// assert_eq!(
///     pascal("foo_bar"),
///     Err(ParseError::UnmetPredicate {
///         actual: 'f',
///         message: "character is not uppercase".to_string(),
///     })
/// );
///
/// assert_eq!(
///     pascal("foo-bar"),
///     Err(ParseError::UnmetPredicate {
///         actual: 'f',
///         message: "character is not uppercase".to_string(),
///     })
/// );
/// ```
pub fn pascal(input: &str) -> ParseResult<String> {
    many1(input, capitalized).map(|(parts, input)| (parts.join(""), input))
}

/// Parses a kebab-case ASCII identifier.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// * `ParseError::UnmetPredicate`
/// if an identifier segment does not start with a lowercase character.
///
/// * `ParseError::UnmetPredicate`
/// if an identifier does not end with a lowercase character.
///
/// * `ParseError::UnmatchedChar`
/// if two identifier segments are not separated by a hyphen.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     case::kebab,
///     ParseError,
/// };
///
/// assert_eq!(kebab("foo"), Ok(("foo".to_string(), "".to_string())));
///
/// assert_eq!(
///     kebab("foo-bar"),
///     Ok(("foo-bar".to_string(), "".to_string()))
/// );
///
/// assert!(kebab("foo_bar").is_err());
/// assert!(kebab("fooBar").is_err());
/// assert!(kebab("Foo").is_err());
/// assert!(kebab("FooBar").is_err());
/// assert!(kebab("foo--").is_err());
/// ```
pub fn kebab(input: &str) -> ParseResult<String> {
    let (head, input) = many1(input, lowercase)?;

    let (tail, input) = many(&input, |input| {
        let (_, input) = hyphen(input)?;
        let (segment, input) = many1(&input, lowercase)?;

        Ok((segment.iter().collect::<String>(), input))
    })?;

    if choice(&input, vec![hyphen, uppercase, digit, underscore]).is_ok() {
        return Err(ParseError::UnmetPredicate {
            actual: input.chars().next().map_or('\0', |c| c),
            message: "unexpected uppercase character, digit, hyphen, or \
                      underscore"
                .to_string(),
        });
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
