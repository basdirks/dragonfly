//! Parse ASCII characters in different ranges.
use super::{
    many,
    ParseError,
    ParseResult,
};

/// Parse a character that fulfills the specified predicate.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `predicate` - The predicate to apply.
/// * `description` - A description of the predicate, used in error messages.
///
/// # Errors
///
/// * `ParseError::UnmetPredicate`
/// if the character does not fulfill the predicate.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::char_if,
///     ParseError,
/// };
///
/// assert_eq!(
///     char_if(
///         "a",
///         |c| c.is_ascii_lowercase(),
///         "character is not lowercase"
///     ),
///     Ok(('a', "".to_string())),
/// );
///
/// assert_eq!(
///     char_if(
///         "A",
///         |c| c.is_ascii_lowercase(),
///         "character is not lowercase"
///     ),
///     Err(ParseError::UnmetPredicate {
///         actual: 'A',
///         message: "character is not lowercase".to_string(),
///     }),
/// );
///
/// assert_eq!(
///     char_if("", |c| c.is_ascii_lowercase(), "character is not lowercase"),
///     Err(ParseError::UnexpectedEof),
/// );
/// ```
pub fn char_if(
    input: &str,
    predicate: fn(char) -> bool,
    description: &str,
) -> ParseResult<char> {
    if let Some(char) = input.chars().next() {
        if predicate(char) {
            return Ok((char, input[1..].to_string()));
        }

        return Err(ParseError::UnmetPredicate {
            actual: char,
            message: description.to_string(),
        });
    }

    Err(ParseError::UnexpectedEof)
}

/// Parse one or more characters that fulfill the specified predicate into a
/// string.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `predicate` - The predicate to apply.
/// * `description` - The description of the predicate, used in error messages.
///
/// # Errors
///
/// * `ParseError::UnmetPredicate`
/// if the first character does not fulfill the predicate.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::chars_if,
///     ParseError,
/// };
///
/// assert_eq!(
///     chars_if(
///         "abc",
///         |c| c.is_ascii_alphabetic(),
///         "character is not alphabetic"
///     ),
///     Ok(("abc".to_string(), "".to_string())),
/// );
///
/// assert_eq!(
///     chars_if(
///         "123",
///         |c| c.is_ascii_alphabetic(),
///         "character is not alphabetic"
///     ),
///     Err(ParseError::UnmetPredicate {
///         actual: '1',
///         message: "character is not alphabetic".to_string(),
///     }),
/// );
/// ```
pub fn chars_if(
    input: &str,
    predicate: fn(char) -> bool,
    description: &str,
) -> ParseResult<String> {
    let (head, mut input) = char_if(input, predicate, description)?;
    let mut result = head.to_string();

    while let Ok((char, new_input)) = char_if(&input, predicate, description) {
        result.push(char);
        input = new_input;
    }

    Ok((result, input))
}

/// Parse an alphabetic ASCII character.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmetPredicate`
/// if the character is not alphabetic.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::alphabetic,
///     ParseError,
/// };
///
/// assert!(alphabetic("a").is_ok());
/// assert!(alphabetic("A").is_ok());
///
/// assert_eq!(
///     alphabetic("1"),
///     Err(ParseError::UnmetPredicate {
///         actual: '1',
///         message: "character is not alphabetic".to_string(),
///     })
/// );
/// ```
pub fn alphabetic(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_alphabetic(),
        "character is not alphabetic",
    )
}

/// Parse one or more alphabetic ASCII characters into a string.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmetPredicate`
/// if the first character is not alphabetic.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::alphabetics,
///     ParseError,
/// };
///
/// assert_eq!(alphabetics("abc"), Ok(("abc".to_string(), "".to_string())),);
///
/// assert_eq!(
///     alphabetics("123"),
///     Err(ParseError::UnmetPredicate {
///         actual: '1',
///         message: "character is not alphabetic".to_string(),
///     }),
/// );
/// ```
pub fn alphabetics(input: &str) -> ParseResult<String> {
    chars_if(
        input,
        |char| char.is_ascii_alphabetic(),
        "character is not alphabetic",
    )
}

/// Parse an alphanumeric ASCII character.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmetPredicate`
/// if the character is not alphanumeric.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::alphanumeric,
///     ParseError,
/// };
///
/// assert!(alphanumeric("a").is_ok());
/// assert!(alphanumeric("A").is_ok());
/// assert!(alphanumeric("1").is_ok());
///
/// assert_eq!(
///     alphanumeric(" "),
///     Err(ParseError::UnmetPredicate {
///         actual: ' ',
///         message: "character is not alphanumeric".to_string(),
///     }),
/// );
/// ```
pub fn alphanumeric(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_alphanumeric(),
        "character is not alphanumeric",
    )
}

/// Parse an ASCII decimal digit.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmetPredicate`
/// if the character is not a digit.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::digit,
///     ParseError,
/// };
///
/// assert!(digit("1").is_ok());
///
/// assert_eq!(
///     digit("a"),
///     Err(ParseError::UnmetPredicate {
///         actual: 'a',
///         message: "character is not a decimal digit".to_string(),
///     }),
/// );
/// ```
pub fn digit(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_digit(),
        "character is not a decimal digit",
    )
}

/// Parse an ASCII lowercase letter.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmetPredicate`
/// if the character is not lowercase.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::lowercase,
///     ParseError,
/// };
///
/// assert!(lowercase("a").is_ok());
///
/// assert_eq!(
///     lowercase("A"),
///     Err(ParseError::UnmetPredicate {
///         actual: 'A',
///         message: "character is not lowercase".to_string(),
///     })
/// );
/// ```
pub fn lowercase(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_lowercase(),
        "character is not lowercase",
    )
}

/// Parse an ASCII uppercase letter.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmetPredicate`
/// if the character is not uppercase.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::uppercase,
///     ParseError,
/// };
///
/// assert!(uppercase("A").is_ok());
///
/// assert_eq!(
///     uppercase("a"),
///     Err(ParseError::UnmetPredicate {
///         actual: 'a',
///         message: "character is not uppercase".to_string(),
///     })
/// );
/// ```
pub fn uppercase(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_uppercase(),
        "character is not uppercase",
    )
}

/// Parse an ASCII whitespace character.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmetPredicate`
/// if the character is not whitespace.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::whitespace,
///     ParseError,
/// };
///
/// assert!(whitespace(" ").is_ok());
/// assert!(whitespace("\t").is_ok());
/// assert!(whitespace("\r").is_ok());
/// assert!(whitespace("\n").is_ok());
///
/// assert_eq!(
///     whitespace("a"),
///     Err(ParseError::UnmetPredicate {
///         actual: 'a',
///         message: "character is not whitespace".to_string(),
///     })
/// );
/// ```
pub fn whitespace(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_whitespace(),
        "character is not whitespace",
    )
}

/// Consume zero or more whitespace characters.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// This parser always succeeds.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::spaces,
///     ParseError,
/// };
///
/// assert_eq!(
///     spaces(" \t\r\n"),
///     Ok((vec![' ', '\t', '\r', '\n'], "".to_string()))
/// );
/// assert_eq!(spaces("abc"), Ok((vec![], "abc".to_string())));
/// ```
pub fn spaces(input: &str) -> ParseResult<Vec<char>> {
    many(input, whitespace)
}
