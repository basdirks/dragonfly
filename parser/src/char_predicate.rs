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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the character does not fulfill the
///   predicate.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     char_if,
///     ParseError,
/// };
///
/// assert_eq!(
///     char_if(
///         "a",
///         |c| c.is_ascii_lowercase(),
///         "Expected lowercase character."
///     ),
///     Ok(('a', String::new())),
/// );
///
/// assert_eq!(
///     char_if(
///         "A",
///         |c| c.is_ascii_lowercase(),
///         "Expected lowercase character."
///     ),
///     Err(ParseError::UnexpectedChar {
///         actual: 'A',
///         message: "Expected lowercase character.".to_owned(),
///     }),
/// );
///
/// assert_eq!(
///     char_if(
///         "",
///         |c| c.is_ascii_lowercase(),
///         "Expected lowercase character."
///     ),
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

        return Err(ParseError::UnexpectedChar {
            actual: char,
            message: description.to_owned(),
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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the first character does not
///   fulfill the predicate.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     chars_if,
///     ParseError,
/// };
///
/// assert_eq!(
///     chars_if(
///         "abc",
///         |c| c.is_ascii_alphabetic(),
///         "Expected alphabetic character."
///     ),
///     Ok(("abc".to_owned(), String::new())),
/// );
///
/// assert_eq!(
///     chars_if(
///         "123",
///         |c| c.is_ascii_alphabetic(),
///         "Expected alphabetic character."
///     ),
///     Err(ParseError::UnexpectedChar {
///         actual: '1',
///         message: "Expected alphabetic character.".to_owned(),
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

/// Parse one or more alphabetic ASCII characters into a string.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the first character is not
///   alphabetic.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     alphabetics,
///     ParseError,
/// };
///
/// assert_eq!(alphabetics("abc"), Ok(("abc".to_owned(), String::new())));
///
/// assert_eq!(
///     alphabetics("123"),
///     Err(ParseError::UnexpectedChar {
///         actual: '1',
///         message: "Expected alphabetic character.".to_owned(),
///     }),
/// );
/// ```
pub fn alphabetics(input: &str) -> ParseResult<String> {
    chars_if(
        input,
        |char| char.is_ascii_alphabetic(),
        "Expected alphabetic character.",
    )
}

/// Parse an ASCII lowercase character.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not
///   lowercase.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     lowercase,
///     ParseError,
/// };
///
/// assert!(lowercase("a").is_ok());
///
/// assert_eq!(
///     lowercase("A"),
///     Err(ParseError::UnexpectedChar {
///         actual: 'A',
///         message: "Expected lowercase character.".to_owned(),
///     })
/// );
/// ```
pub fn lowercase(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_lowercase(),
        "Expected lowercase character.",
    )
}

/// Parse an ASCII uppercase character.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not
///   uppercase.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     uppercase,
///     ParseError,
/// };
///
/// assert!(uppercase("A").is_ok());
///
/// assert_eq!(
///     uppercase("a"),
///     Err(ParseError::UnexpectedChar {
///         actual: 'a',
///         message: "Expected uppercase character.".to_owned(),
///     })
/// );
/// ```
pub fn uppercase(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_uppercase(),
        "Expected uppercase character.",
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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not
///   whitespace.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     space,
///     ParseError,
/// };
///
/// assert!(space(" ").is_ok());
/// assert!(space("\t").is_ok());
/// assert!(space("\r").is_ok());
/// assert!(space("\n").is_ok());
///
/// assert_eq!(
///     space("a"),
///     Err(ParseError::UnexpectedChar {
///         actual: 'a',
///         message: "Expected whitespace character.".to_owned(),
///     })
/// );
/// ```
pub fn space(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_whitespace(),
        "Expected whitespace character.",
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
/// use parser::{
///     spaces,
///     ParseError,
/// };
///
/// assert_eq!(
///     spaces(" \t\r\n"),
///     Ok((vec![' ', '\t', '\r', '\n'], String::new()))
/// );
/// assert_eq!(spaces("abc"), Ok((Vec::new(), "abc".to_owned())));
/// ```
pub fn spaces(input: &str) -> ParseResult<Vec<char>> {
    many(input, space)
}
