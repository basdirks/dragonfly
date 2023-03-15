//! A parser combinator library.
//!
//! This library provides a set of combinators for parsing strings.
#![feature(rustdoc_missing_doc_code_examples)]
#![deny(
    clippy::all,
    clippy::format_push_string,
    clippy::if_then_some_else_none,
    clippy::missing_docs_in_private_items,
    clippy::mixed_read_write_in_expression,
    clippy::nursery,
    clippy::pedantic,
    clippy::str_to_string,
    clippy::string_to_string,
    clippy::unnecessary_self_imports,
    clippy::unneeded_field_pattern,
    clippy::unwrap_in_result,
    missing_copy_implementations,
    missing_debug_implementations,
    missing_docs,
    rustdoc::missing_doc_code_examples,
    rustdoc::missing_crate_level_docs,
    trivial_casts,
    trivial_numeric_casts,
    unsafe_code,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results,
    variant_size_differences
)]

/// Parse ASCII identifiers with different case styles.
pub mod capitalization;
/// Parse common ASCII characters.
pub mod char;
/// Parse ASCII characters that match a predicate.
pub mod char_predicate;
/// Macro utilities.
pub mod r#macro;
/// Parsing errors.
pub mod parse_error;

pub use {
    self::char::{
        at,
        brace_close,
        brace_open,
        colon,
        comma,
        dollar,
        paren_close,
        paren_open,
    },
    capitalization::{
        camel_case,
        capitalized,
        pascal_case,
    },
    char_predicate::{
        alphabetics,
        char_if,
        chars_if,
        lowercase,
        space,
        spaces,
        uppercase,
    },
    parse_error::ParseError,
};

/// The result of applying a parser to an input string.
pub type ParseResult<T, E = ParseError> = Result<(T, String), E>;

/// A function that parses a string.
pub type ParseFn<T> = fn(&str) -> ParseResult<T>;

/// Apply a parser and map over the result.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `parser` - The parser to apply.
/// * `f` - The function to map over the result.
///
/// # Errors
///
/// Returns `ParseError` if the parser fails.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     char,
///     map,
/// };
///
/// assert_eq!(
///     map("abc", char!('a'), |c| c.to_ascii_uppercase()),
///     Ok(('A', "bc".to_owned()))
/// );
/// ```
pub fn map<T, U>(
    input: &str,
    parser: ParseFn<T>,
    f: fn(T) -> U,
) -> ParseResult<U> {
    parser(input).map(|(t, input)| (f(t), input))
}

/// Apply a parser and return the given value instead of the parsed result.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `parser` - The parser to apply.
/// * `value` - The value to return.
///
/// # Errors
///
/// Returns `ParseError` if the parser fails.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     char,
///     tag,
/// };
///
/// assert_eq!(tag("abc", char!('a'), 1), Ok((1, "bc".to_owned())));
/// ```
pub fn tag<T, U>(
    input: &str,
    parser: ParseFn<T>,
    value: U,
) -> ParseResult<U> {
    parser(input).map(|(_, input)| (value, input))
}

/// Parse an expression surrounded by two specified string literals.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `open` - The string to parse before the expression.
/// * `parser` - The parser to apply to the expression.
/// * `close` - The string to parse after the expression.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnmatchedLiteral` if the input does not start with
///   the specified string.
/// * Returns `ParseError` if the parser fails.
/// * Returns `ParseError::UnmatchedLiteral` if the input does not end with the
///   specified string.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     between,
///     char,
///     ParseError,
/// };
///
/// let input = "foo";
///
/// assert_eq!(
///     between(input, "f", |input| char(input, 'o'), "o"),
///     Ok(('o', String::new())),
/// );
/// ```
pub fn between<T>(
    input: &str,
    open: &str,
    parser: ParseFn<T>,
    close: &str,
) -> ParseResult<T> {
    let (_, input) = literal(input, open)?;
    let (t, input) = parser(&input)?;
    let (_, input) = literal(&input, close)?;

    Ok((t, input))
}

/// Parse the specified character.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `char` - The character to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnexpectedChar` if the input does not start with the
///   specified character.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     char,
///     ParseError,
/// };
///
/// assert_eq!(char("a", 'a'), Ok(('a', String::new())));
/// assert_eq!(char("a", 'a'), Ok(('a', "".to_owned())));
///
/// assert_eq!(
///     char("a", 'b'),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character 'b', found 'a'.".to_owned(),
///         actual: 'a'
///     })
/// );
///
/// assert_eq!(char("", 'b'), Err(ParseError::UnexpectedEof));
/// ```
pub fn char(
    input: &str,
    expected: char,
) -> ParseResult<char> {
    input
        .chars()
        .next()
        .map_or(Err(ParseError::UnexpectedEof), |actual| {
            if actual == expected {
                Ok((expected, input[1..].to_string()))
            } else {
                Err(ParseError::UnexpectedChar {
                    message: format!(
                        "Expected character '{expected}', found '{actual}'."
                    ),
                    actual,
                })
            }
        })
}

/// Parse the specified string literal.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `literal` - The literal to parse.
///
/// # Errors
///
/// * Returns `ParseError::UnexpectedEof` if the input is empty.
/// * Returns `ParseError::UnmatchedLiteral` if the input does not start with
///   the specified literal.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     literal,
///     ParseError,
/// };
///
/// assert_eq!(literal("foo", "foo"), Ok(("foo".to_owned(), String::new())));
/// assert_eq!(
///     literal("foobar", "foo"),
///     Ok(("foo".to_owned(), "bar".to_owned()))
/// );
///
/// assert_eq!(
///     literal("foo", "bar"),
///     Err(ParseError::UnmatchedLiteral {
///         expected: "bar".to_owned()
///     })
/// );
///
/// assert_eq!(
///     literal("bbar", "bar"),
///     Err(ParseError::UnmatchedLiteral {
///         expected: "bar".to_owned()
///     })
/// );
///
/// assert_eq!(literal("", "bar"), Err(ParseError::UnexpectedEof));
/// ```
pub fn literal(
    input: &str,
    literal: &str,
) -> ParseResult<String> {
    if input.is_empty() {
        return Err(ParseError::UnexpectedEof);
    }

    input.strip_prefix(literal).map_or_else(
        || {
            Err(ParseError::UnmatchedLiteral {
                expected: literal.to_owned(),
            })
        },
        |input| Ok((literal.to_owned(), input.to_owned())),
    )
}

/// Apply a parser zero or more times until it fails, returning a vector of the
/// results.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `parser` - The parser to apply.
///
/// # Errors
///
/// This parser always succeeds.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     lowercase,
///     many,
/// };
///
/// assert_eq!(
///     many("abc", lowercase),
///     Ok((vec!['a', 'b', 'c'], String::new()))
/// );
///
/// assert_eq!(many("ab3", lowercase), Ok((vec!['a', 'b'], "3".to_owned())));
/// assert_eq!(many("a23", lowercase), Ok((vec!['a'], "23".to_owned())));
/// assert_eq!(many("123", lowercase), Ok((Vec::new(), "123".to_owned())));
/// ```
pub fn many<T>(
    input: &str,
    parser: ParseFn<T>,
) -> ParseResult<Vec<T>> {
    let mut input = input.to_owned();
    let mut result = Vec::new();

    while let Ok((value, new_input)) = parser(&input) {
        result.push(value);
        input = new_input;
    }

    Ok((result, input))
}

/// Apply a parser one or more times until it fails, returning a vector of the
/// results.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `parser` - The parser to apply.
///
/// # Errors
///
/// Returns `ParseError` if the parser fails to match at least once.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     lowercase,
///     many1,
///     ParseError,
/// };
///
/// assert_eq!(
///     many1("abc", lowercase),
///     Ok((vec!['a', 'b', 'c'], String::new()))
/// );
///
/// assert_eq!(
///     many1("ab3", lowercase),
///     Ok((vec!['a', 'b'], "3".to_owned()))
/// );
///
/// assert_eq!(many1("a23", lowercase), Ok((vec!['a'], "23".to_owned())));
/// ```
///
/// ```rust
/// use parser::{
///     lowercase,
///     many1,
///     ParseError,
/// };
///
/// assert_eq!(
///     many1("123", lowercase),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected lowercase character.".to_owned(),
///         actual: '1',
///     })
/// );
/// ```
pub fn many1<T>(
    input: &str,
    parser: ParseFn<T>,
) -> ParseResult<Vec<T>> {
    let (head, input) = parser(input)?;
    let (mut tail, input) = many(&input, parser)?;

    tail.insert(0, head);

    Ok((tail, input))
}

/// Try applying each parser in turn, returning the first successful result.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `parsers` - The parsers to apply.
///
/// # Errors
///
/// Returns `ParseError::UnmatchedChoice` if none of the parsers match.
///
/// # Examples
///
/// ```rust
/// use parser::{
///     choice,
///     literal,
///     tag,
///     ParseError,
/// };
///
/// #[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
/// enum Choice {
///     A,
///     B,
/// }
///
/// assert_eq!(
///     choice(
///         "abc",
///         vec![
///             tag!(literal!("abc"), Choice::A),
///             tag!(literal!("abc"), Choice::B),
///         ]
///     ),
///     Ok((Choice::A, String::new())),
/// );
///
/// assert_eq!(
///     choice(
///         "abc",
///         vec![
///             tag!(literal!("abc"), Choice::B),
///             tag!(literal!("abc"), Choice::A),
///         ]
///     ),
///     Ok((Choice::B, String::new())),
/// );
///
/// assert_eq!(
///     choice("abc", vec![tag!(literal!("def"), Choice::A)]),
///     Err(ParseError::UnmatchedChoice {
///         errors: vec![ParseError::UnmatchedLiteral {
///             expected: "def".to_owned(),
///         }]
///     })
/// );
/// ```
pub fn choice<T>(
    input: &str,
    parsers: Vec<ParseFn<T>>,
) -> ParseResult<T> {
    let mut errors = Vec::new();

    for parser in parsers {
        match parser(input) {
            Ok((value, input)) => return Ok((value, input)),
            Err(error) => errors.push(error),
        }
    }

    Err(ParseError::UnmatchedChoice { errors })
}

/// Optionally apply a parser.
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
///     literal,
///     option,
///     ParseError,
/// };
///
/// assert_eq!(
///     option("abc", |input| literal(input, "abc")),
///     Ok((Some("abc".to_owned()), String::new())),
/// );
///
/// assert_eq!(
///     option("def", |input| literal(input, "abc")),
///     Ok((None, "def".to_owned())),
/// );
/// ```
pub fn option<T>(
    input: &str,
    parser: ParseFn<T>,
) -> ParseResult<Option<T>> {
    match parser(input) {
        Ok((value, input)) => Ok((Some(value), input)),
        Err(_) => Ok((None, input.to_owned())),
    }
}
