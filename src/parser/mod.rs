/// Parse ASCII identifiers with different case styles.
pub mod case;
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
        forward_slash,
        hyphen,
        paren_close,
        paren_open,
        underscore,
    },
    case::{
        camel as camel_case,
        capitalized,
        kebab as kebab_case,
        pascal as pascal_case,
    },
    char_predicate::{
        alphabetic,
        alphabetics,
        alphanumeric,
        alphanumerics,
        char_if,
        chars_if,
        digit,
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
/// use dragonfly::parser::{
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
/// use dragonfly::parser::{
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
/// use dragonfly::parser::{
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
/// use dragonfly::parser::{
///     alphabetic,
///     many,
/// };
///
/// assert_eq!(
///     many("abc", alphabetic),
///     Ok((vec!['a', 'b', 'c'], String::new()))
/// );
///
/// assert_eq!(
///     many("ab3", alphabetic),
///     Ok((vec!['a', 'b'], "3".to_owned()))
/// );
///
/// assert_eq!(many("a23", alphabetic), Ok((vec!['a'], "23".to_owned())));
/// assert_eq!(many("123", alphabetic), Ok((vec![], "123".to_owned())));
/// ```
pub fn many<T>(
    input: &str,
    parser: ParseFn<T>,
) -> ParseResult<Vec<T>> {
    let mut input = input.to_owned();
    let mut result = vec![];

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
/// use dragonfly::parser::{
///     alphabetic,
///     many1,
///     ParseError,
/// };
///
/// assert_eq!(
///     many1("abc", alphabetic),
///     Ok((vec!['a', 'b', 'c'], String::new()))
/// );
///
/// assert_eq!(
///     many1("ab3", alphabetic),
///     Ok((vec!['a', 'b'], "3".to_owned()))
/// );
///
/// assert_eq!(many1("a23", alphabetic), Ok((vec!['a'], "23".to_owned())));
/// ```
///
/// ```rust
/// use dragonfly::parser::{
///     alphabetic,
///     many1,
///     ParseError,
/// };
///
/// assert_eq!(
///     many1("123", alphabetic),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected alphabetic character.".to_owned(),
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
/// use dragonfly::{
///     literal,
///     parser::{
///         choice,
///         literal,
///         tag,
///         ParseError,
///     },
///     tag,
/// };
///
/// #[derive(Debug, Eq, PartialEq)]
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
    let mut errors = vec![];

    for parser in parsers {
        match parser(input) {
            Ok((value, input)) => return Ok((value, input)),
            Err(error) => errors.push(error),
        }
    }

    Err(ParseError::UnmatchedChoice { errors })
}

/// Apply a parser a specified number of times, returning a vector of the
/// the results.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `parser` - The parser to apply.
/// * `count` - The number of times to apply the parser.
///
/// # Errors
///
/// Returns `ParseError` if the parser fails to match the specified number of
/// times.
///
/// # Examples
///
/// ```rust
/// use dragonfly::{
///     char,
///     parser::{
///         alphabetic,
///         char,
///         count,
///         literal,
///         ParseError,
///     },
/// };
///
/// assert_eq!(
///     count("abc", char!('a'), 1),
///     Ok((vec!['a'], "bc".to_owned()))
/// );
///
/// assert!(count("abc", char!('a'), 2).is_err());
///
/// assert_eq!(
///     count("abc", alphabetic, 3),
///     Ok((vec!['a', 'b', 'c'], String::new()))
/// );
///
/// assert!(count("abc", alphabetic, 4).is_err());
///
/// assert_eq!(count("abc", alphabetic, 0), Ok((vec![], "abc".to_owned())));
/// ```
pub fn count<T>(
    input: &str,
    parser: ParseFn<T>,
    count: usize,
) -> ParseResult<Vec<T>> {
    let mut input = input.to_owned();
    let mut result = vec![];

    for _ in 0..count {
        let (value, new_input) = parser(&input)?;

        result.push(value);

        input = new_input;
    }

    Ok((result, input))
}

/// Apply each parser exactly once, regardless of order. The results must be in
/// the same order as the parsers, but the parsers may be applied in any order.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `parsers` - The parsers to apply.
///
/// # Errors
///
/// Returns `ParseError` if any parser fails to match exactly once.
///
/// # Examples
///
/// ```rust
/// use dragonfly::{
///     char,
///     parser::{
///         alphabetic,
///         char,
///         many_once,
///         ParseError,
///     },
/// };
///
/// assert_eq!(
///     many_once("abc", &[char!('a'), char!('b'), char!('c')]),
///     Ok((vec!['a', 'b', 'c'], String::new()))
/// );
///
/// assert_eq!(
///     many_once("abc", &[char!('c'), char!('b'), char!('a')]),
///     Ok((vec!['c', 'b', 'a'], String::new()))
/// );
///
/// assert_eq!(
///     many_once("abc", &[char!('a'), char!('b'), char!('d')]),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character 'd', found 'c'.".to_owned(),
///         actual: 'c',
///     })
/// );
///
/// assert_eq!(
///     many_once("", &[char!('a'), char!('b'), char!('d')]),
///     Err(ParseError::UnexpectedEof)
/// );
///
/// assert_eq!(
///     many_once::<String>("abc", &[]),
///     Ok((vec![], "abc".to_owned()))
/// );
///
/// assert_eq!(
///     many_once("aaa", &[char!('a'), alphabetic]),
///     Ok((vec!['a', 'a'], "a".to_owned()))
/// );
///
/// assert_eq!(
///     many_once("abc", &[char!('a'), char!('a')]),
///     Err(ParseError::UnexpectedChar {
///         message: "Expected character 'a', found 'b'.".to_owned(),
///         actual: 'b',
///     })
/// );
/// ```
pub fn many_once<T: Clone>(
    input: &str,
    parsers: &[ParseFn<T>],
) -> ParseResult<Vec<T>> {
    let length = parsers.len();
    let mut input = input.to_owned();
    let mut results = vec![None; length];
    let mut last_error = None;

    'outer: for _ in 0..length {
        for (index, parser) in parsers.iter().enumerate() {
            if results[index].is_some() {
                continue;
            }

            match parser(&input) {
                Ok((value, new_input)) => {
                    results[index] = Some(value);
                    input = new_input;
                    continue 'outer;
                }
                err => {
                    last_error = err.err();
                    continue;
                }
            }
        }

        if let Some(error) = last_error {
            return Err(error);
        }
    }

    Ok((results.into_iter().map(Option::unwrap).collect(), input))
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
/// use dragonfly::parser::{
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
