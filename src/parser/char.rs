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
/// use dragonfly::parser::{
///     brace_open,
///     ParseError,
/// };
///
/// assert_eq!(brace_open("{"), Ok(('{', "".to_owned())));
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
/// use dragonfly::parser::{
///     brace_close,
///     ParseError,
/// };
///
/// assert_eq!(brace_close("}"), Ok(('}', "".to_owned())));
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

/// Parse a hyphen.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not a
///   hyphen.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     hyphen,
///     ParseError,
/// };
///
/// assert_eq!(hyphen("-"), Ok(('-', "".to_owned())));
///
/// assert_eq!(
///     hyphen("a"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character '-', found 'a'.".to_owned(),
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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not a
///   forward slash.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     forward_slash,
///     ParseError,
/// };
///
/// assert_eq!(forward_slash("/"), Ok(('/', "".to_owned())));
///
/// assert_eq!(
///     forward_slash("a"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character '/', found 'a'.".to_owned(),
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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not a colon.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     colon,
///     ParseError,
/// };
///
/// assert_eq!(colon(":"), Ok((':', "".to_owned())));
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
/// use dragonfly::parser::{
///     paren_open,
///     ParseError,
/// };
///
/// assert_eq!(paren_open("("), Ok(('(', "".to_owned())));
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
/// use dragonfly::parser::{
///     paren_close,
///     ParseError,
/// };
///
/// assert_eq!(paren_close(")"), Ok((')', "".to_owned())));
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
/// use dragonfly::parser::{
///     dollar,
///     ParseError,
/// };
///
/// assert_eq!(dollar("$"), Ok(('$', "".to_owned())));
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

/// Parse an underscore.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not an
///   underscore.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     underscore,
///     ParseError,
/// };
///
/// assert_eq!(underscore("_"), Ok(('_', "".to_owned())));
///
/// assert_eq!(
///     underscore("a"),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character '_', found 'a'.".to_owned(),
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
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the next character is not a comma.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     comma,
///     ParseError,
/// };
///
/// assert_eq!(comma(","), Ok((',', "".to_owned())));
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
/// use dragonfly::parser::{
///     at,
///     ParseError,
/// };
///
/// assert_eq!(at("@"), Ok(('@', "".to_owned())));
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
