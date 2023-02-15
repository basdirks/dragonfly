///! Parser combinators used to parse the AST.
pub mod case;
pub mod char;
pub mod char_range;
pub mod r#macro;

/// Low-level parsing errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    /// A custom error.
    CustomError {
        /// A description of the error.
        message: String,
    },
    /// Unexpected end of file.
    UnexpectedEof,
    /// The predicate does not apply to first character of the input.
    ///
    /// aka `ParseError::DaBoiIsNotHappyWithThisOne` -- @Fecony
    UnmetPredicate {
        /// The character that was found.
        actual: char,
        /// A description of the predicate.
        message: String,
    },
    /// The input does not match the given character.
    UnmatchedChar {
        /// The character that was found.
        actual: char,
        /// The expected character.
        expected: char,
    },
    /// None of the given parsers matched the input.
    UnmatchedChoice {
        /// The errors generated by each parser.
        errors: Vec<ParseError>,
    },
    /// The input does not match the given string literal.
    UnmatchedLiteral {
        /// The expected string literal.
        expected: String,
    },
}

pub type ParseResult<T, E = ParseError> = Result<(T, String), E>;

pub type ParseFn<T> = fn(&str) -> ParseResult<T>;

/// Apply a parser and map the result.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `parser` - The parser to apply.
/// * `f` - The function to map the result with.
///
/// # Errors
///
/// * `ParseError`
/// if the parser fails.
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
/// * `ParseError`
/// if the parser fails.
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
/// * `ParseError::UnmatchedLiteral`
/// if the input does not start with the specified string.
///
/// * `ParseError::UnmatchedLiteral`
/// if the input does not end with the specified string.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// * `ParseError`
/// if the parser fails.
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
///     Ok(('o', "".to_string())),
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
/// * `ParseError::UnmatchedChar`
/// if the input does not start with the specified character.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char,
///     ParseError,
/// };
///
/// assert_eq!(char("a", 'a'), Ok(('a', "".to_string())));
/// assert_eq!(char("ab", 'a'), Ok(('a', "b".to_string())));
///
/// assert_eq!(
///     char("a", 'b'),
///     Err(ParseError::UnmatchedChar {
///         expected: 'b',
///         actual: 'a'
///     })
/// );
///
/// assert_eq!(char("", 'b'), Err(ParseError::UnexpectedEof));
/// ```
pub fn char(
    input: &str,
    char: char,
) -> ParseResult<char> {
    input
        .chars()
        .next()
        .map_or(Err(ParseError::UnexpectedEof), |c| {
            if c == char {
                Ok((char, input[1..].to_string()))
            } else {
                Err(ParseError::UnmatchedChar {
                    actual: c,
                    expected: char,
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
/// * `ParseError::UnmatchedLiteral`
/// if the input does not start with the specified literal.
///
/// * `ParseError::UnexpectedEof`
/// if the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     literal,
///     ParseError,
/// };
///
/// assert_eq!(
///     literal("foo", "foo"),
///     Ok(("foo".to_string(), "".to_string()))
/// );
/// assert_eq!(
///     literal("foobar", "foo"),
///     Ok(("foo".to_string(), "bar".to_string()))
/// );
///
/// assert_eq!(
///     literal("foo", "bar"),
///     Err(ParseError::UnmatchedLiteral {
///         expected: "bar".to_string()
///     })
/// );
///
/// assert_eq!(
///     literal("bbar", "bar"),
///     Err(ParseError::UnmatchedLiteral {
///         expected: "bar".to_string()
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
                expected: literal.to_string(),
            })
        },
        |input| Ok((literal.to_string(), input.to_string())),
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
/// None. This parser always succeeds.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::alphabetic,
///     many,
///     ParseError,
/// };
///
/// assert_eq!(
///     many("abc", alphabetic),
///     Ok((vec!['a', 'b', 'c'], "".to_string()))
/// );
///
/// assert_eq!(
///     many("ab3", alphabetic),
///     Ok((vec!['a', 'b'], "3".to_string()))
/// );
///
/// assert_eq!(many("a23", alphabetic), Ok((vec!['a'], "23".to_string())));
/// assert_eq!(many("123", alphabetic), Ok((vec![], "123".to_string())));
/// ```
pub fn many<T>(
    input: &str,
    parser: ParseFn<T>,
) -> ParseResult<Vec<T>> {
    let mut input = input.to_string();
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
/// * `ParseError`
/// if the parser fails to match at least once.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     char_range::alphabetic,
///     many1,
///     ParseError,
/// };
///
/// assert_eq!(
///     many1("abc", alphabetic),
///     Ok((vec!['a', 'b', 'c'], "".to_string()))
/// );
///
/// assert_eq!(
///     many1("ab3", alphabetic),
///     Ok((vec!['a', 'b'], "3".to_string()))
/// );
///
/// assert_eq!(many1("a23", alphabetic), Ok((vec!['a'], "23".to_string())));
///
/// assert_eq!(
///     many1("123", alphabetic),
///     Err(ParseError::UnmetPredicate {
///         actual: '1',
///         message: "character is not alphabetic".to_string(),
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
/// * `ParseError::UnmatchedChoice`
/// if none of the parsers match.
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
/// let input = "abc";
///
/// assert_eq!(
///     choice(
///         input,
///         vec![
///             tag!(literal!("abc"), Choice::A),
///             tag!(literal!("abc"), Choice::B),
///         ]
///     ),
///     Ok((Choice::A, "".to_string())),
/// );
///
/// assert_eq!(
///     choice(
///         input,
///         vec![
///             tag!(literal!("abc"), Choice::B),
///             tag!(literal!("abc"), Choice::A),
///         ]
///     ),
///     Ok((Choice::B, "".to_string())),
/// );
///
/// assert_eq!(
///     choice(input, vec![tag!(literal!("def"), Choice::A)],),
///     Err(ParseError::UnmatchedChoice {
///         errors: vec![ParseError::UnmatchedLiteral {
///             expected: "def".to_string(),
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

/// Optionally apply a parser.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// None. This parser always succeeds.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{
///     literal,
///     maybe,
///     ParseError,
/// };
///
/// assert_eq!(
///     maybe("abc", |input| literal(input, "abc"),),
///     Ok((Some("abc".to_string()), "".to_string())),
/// );
///
/// assert_eq!(
///     maybe("def", |input| literal(input, "abc"),),
///     Ok((None, "def".to_string())),
/// );
/// ```
pub fn maybe<T>(
    input: &str,
    parser: ParseFn<T>,
) -> ParseResult<Option<T>> {
    match parser(input) {
        Ok((value, input)) => Ok((Some(value), input)),
        Err(_) => Ok((None, input.to_string())),
    }
}
