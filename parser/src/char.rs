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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not an
///   opening brace.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     brace_open,
///     ParseError,
/// };
///
/// assert_eq!(brace_open("{"), Ok(('{', String::new())));
///
/// assert_eq!(
///     brace_open("}"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character '{', found '}'.".to_owned(),
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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not a
///   closing brace.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     brace_close,
///     ParseError,
/// };
///
/// assert_eq!(brace_close("}"), Ok(('}', String::new())));
///
/// assert_eq!(
///     brace_close("{"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character '}', found '{'.".to_owned(),
///         actual: '{'
///     })
/// );
/// ```
pub fn brace_close(input: &str) -> ParseResult<char> {
    char(input, '}')
}

/// Parse a colon.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not a colon.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     colon,
///     ParseError,
/// };
///
/// assert_eq!(colon(":"), Ok((':', String::new())));
///
/// assert_eq!(
///     colon("a"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character ':', found 'a'.".to_owned(),
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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not an
///   opening parenthesis.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     paren_open,
///     ParseError,
/// };
///
/// assert_eq!(paren_open("("), Ok(('(', String::new())));
///
/// assert_eq!(
///     paren_open(")"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character '(', found ')'.".to_owned(),
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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not a
///   closing parenthesis.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     paren_close,
///     ParseError,
/// };
///
/// assert_eq!(paren_close(")"), Ok((')', String::new())));
///
/// assert_eq!(
///     paren_close("("),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character ')', found '('.".to_owned(),
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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not a dollar
///   sign.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     dollar,
///     ParseError,
/// };
///
/// assert_eq!(dollar("$"), Ok(('$', String::new())));
///
/// assert_eq!(
///     dollar("a"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character '$', found 'a'.".to_owned(),
///         actual: 'a'
///     })
/// );
/// ```
pub fn dollar(input: &str) -> ParseResult<char> {
    char(input, '$')
}

/// Parse a comma.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not a comma.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     comma,
///     ParseError,
/// };
///
/// assert_eq!(comma(","), Ok((',', String::new())));
///
/// assert_eq!(
///     comma("a"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character ',', found 'a'.".to_owned(),
///         actual: 'a'
///     })
/// );
/// ```
pub fn comma(input: &str) -> ParseResult<char> {
    char(input, ',')
}

/// Parse an at sign.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not an at
///  sign.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     at,
///     ParseError,
/// };
///
/// assert_eq!(at("@"), Ok(('@', String::new())));
/// assert_eq!(
///     at("a"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character '@', found 'a'.".to_owned(),
///         actual: 'a'
///     })
/// );
/// ```
pub fn at(input: &str) -> ParseResult<char> {
    char(input, '@')
}
