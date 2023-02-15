//! Parse common characters.
use super::{
    char,
    ParseResult,
};

/// Parse an opening brace.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not an opening brace.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::brace_open,
///     ParseError,
/// };
///
/// assert_eq!(brace_open("{"), Ok(('{', "".to_string())));
///
/// assert_eq!(
///     brace_open("}"),
///     Err(ParseError::UnmatchedChar {
///         expected: '{',
///         actual: '}'
///     })
/// );
/// ```
pub fn brace_open(input: &str) -> ParseResult<char> {
    char(input, '{')
}

/// Parse a closing brace.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not a closing brace.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::brace_close,
///     ParseError,
/// };
///
/// assert_eq!(brace_close("}"), Ok(('}', "".to_string())));
///
/// assert_eq!(
///     brace_close("{"),
///     Err(ParseError::UnmatchedChar {
///         expected: '}',
///         actual: '{'
///     })
/// );
/// ```
pub fn brace_close(input: &str) -> ParseResult<char> {
    char(input, '}')
}

/// Parse a hyphen.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not a hyphen.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::hyphen,
///     ParseError,
/// };
///
/// assert_eq!(hyphen("-"), Ok(('-', "".to_string())));
///
/// assert_eq!(
///     hyphen("a"),
///     Err(ParseError::UnmatchedChar {
///         expected: '-',
///         actual: 'a'
///     })
/// );
/// ```
pub fn hyphen(input: &str) -> ParseResult<char> {
    char(input, '-')
}

/// Parse a forward slash.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not a forward slash.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::forward_slash,
///     ParseError,
/// };
///
/// assert_eq!(forward_slash("/"), Ok(('/', "".to_string())));
///
/// assert_eq!(
///     forward_slash("a"),
///     Err(ParseError::UnmatchedChar {
///         expected: '/',
///         actual: 'a'
///     })
/// );
/// ```
pub fn forward_slash(input: &str) -> ParseResult<char> {
    char(input, '/')
}

/// Parse a colon.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not a colon.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::colon,
///     ParseError,
/// };
///
/// assert_eq!(colon(":"), Ok((':', "".to_string())));
///
/// assert_eq!(
///     colon("a"),
///     Err(ParseError::UnmatchedChar {
///         expected: ':',
///         actual: 'a'
///     })
/// );
/// ```
pub fn colon(input: &str) -> ParseResult<char> {
    char(input, ':')
}

/// Parse an opening parenthesis.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not an opening parenthesis.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::paren_open,
///     ParseError,
/// };
///
/// assert_eq!(paren_open("("), Ok(('(', "".to_string())));
///
/// assert_eq!(
///     paren_open(")"),
///     Err(ParseError::UnmatchedChar {
///         expected: '(',
///         actual: ')'
///     })
/// );
/// ```
pub fn paren_open(input: &str) -> ParseResult<char> {
    char(input, '(')
}

/// Parse a closing parenthesis.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not a closing parenthesis.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::paren_close,
///     ParseError,
/// };
///
/// assert_eq!(paren_close(")"), Ok((')', "".to_string())));
///
/// assert_eq!(
///     paren_close("("),
///     Err(ParseError::UnmatchedChar {
///         expected: ')',
///         actual: '('
///     })
/// );
/// ```
pub fn paren_close(input: &str) -> ParseResult<char> {
    char(input, ')')
}

/// Parse a dollar sign.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not a dollar sign.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::dollar,
///     ParseError,
/// };
///
/// assert_eq!(dollar("$"), Ok(('$', "".to_string())));
///
/// assert_eq!(
///     dollar("a"),
///     Err(ParseError::UnmatchedChar {
///         expected: '$',
///         actual: 'a'
///     })
/// );
/// ```
pub fn dollar(input: &str) -> ParseResult<char> {
    char(input, '$')
}

/// Parse an underscore.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not an underscore.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::underscore,
///     ParseError,
/// };
///
/// assert_eq!(underscore("_"), Ok(('_', "".to_string())));
///
/// assert_eq!(
///     underscore("a"),
///     Err(ParseError::UnmatchedChar {
///         expected: '_',
///         actual: 'a'
///     })
/// );
/// ```
pub fn underscore(input: &str) -> ParseResult<char> {
    char(input, '_')
}

/// Parse a comma.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar`
/// if the next character is not a comma.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char::comma,
///     ParseError,
/// };
///
/// assert_eq!(comma(","), Ok((',', "".to_string())));
///
/// assert_eq!(
///     comma("a"),
///     Err(ParseError::UnmatchedChar {
///         expected: ',',
///         actual: 'a'
///     })
/// );
/// ```
pub fn comma(input: &str) -> ParseResult<char> {
    char(input, ',')
}
