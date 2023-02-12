#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ParseError {
    CustomError { message: String },
    UnexpectedEof,
    UnmatchedChar { expected: char, actual: char },
    UnmatchedCharPredicate { description: String, actual: char },
    UnmatchedChoice,
    UnmatchedLiteral { expected: String },
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
/// * `ParseError` - If the parser fails.
pub fn map_ok<T, U>(input: &str, parser: ParseFn<T>, f: fn(T) -> U) -> ParseResult<U> {
    parser(input).map(|(t, input)| (f(t), input))
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
/// * `ParseError::UnmatchedLiteral` - If the input does not start with the specified string.
/// * `ParseError` - If the parser fails.
/// * `ParseError::UnmatchedLiteral` - If the input does not end with the specified string.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{between, char, ParseError};
///
/// let input = "foo";
///
/// assert_eq!(
///     between(input, "f", |input| char(input, 'o'), "o"),
///     Ok(('o', "".to_string())),
/// );
/// ```
pub fn between<T>(input: &str, open: &str, parser: ParseFn<T>, close: &str) -> ParseResult<T> {
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
/// * `ParseError::UnmatchedChar` - If the input does not start with the specified character.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{char, ParseError};
///
/// assert_eq!(char("a", 'a'), Ok(('a', "".to_string())));
/// assert_eq!(char("ab", 'a'), Ok(('a', "b".to_string())));
/// assert_eq!(char("a", 'b'), Err(ParseError::UnmatchedChar { expected: 'b', actual: 'a' }));
/// assert_eq!(char("", 'b'), Err(ParseError::UnexpectedEof));
/// ```
pub fn char(input: &str, char: char) -> ParseResult<char> {
    input
        .chars()
        .next()
        .map_or(Err(ParseError::UnexpectedEof), |c| {
            if c == char {
                Ok((char, input[1..].to_string()))
            } else {
                Err(ParseError::UnmatchedChar {
                    expected: char,
                    actual: c,
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
/// * `ParseError::UnmatchedLiteral` - If the input does not start with the specified literal.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{literal, ParseError};
///
/// assert_eq!(literal("foo", "foo"), Ok(("foo".to_string(), "".to_string())));
/// assert_eq!(literal("foobar", "foo"), Ok(("foo".to_string(), "bar".to_string())));
///
/// assert_eq!(
///     literal("foo", "bar"),
///     Err(ParseError::UnmatchedLiteral { expected: "bar".to_string() })
/// );
///
/// assert_eq!(
///     literal("bbar", "bar"),
///     Err(ParseError::UnmatchedLiteral { expected: "bar".to_string() })
/// );
///
/// assert_eq!(literal("", "bar"), Err(ParseError::UnexpectedEof));
/// ```
pub fn literal(input: &str, literal: &str) -> ParseResult<String> {
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

/// Parse a capitalized alphabetic ASCII identifier.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedCharPredicate` - If the first character is not uppercase.
/// * `ParseError` - If the parser fails.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{capitalized, ParseError};
///
/// assert_eq!(capitalized("Foo"), Ok(("Foo".to_string(), "".to_string())));
/// assert_eq!(capitalized("F0o"), Ok(("F".to_string(), "0o".to_string())));
/// assert_eq!(capitalized("Foo Bar"), Ok(("Foo".to_string(), " Bar".to_string())));
/// assert_eq!(capitalized("FooBar"), Ok(("FooBar".to_string(), "".to_string())));
///
/// assert_eq!(capitalized("foo"), Err(ParseError::UnmatchedCharPredicate {
///     actual: 'f',
///     description: "should be uppercase".to_string(),
/// }));
/// ```
pub fn capitalized(input: &str) -> ParseResult<String> {
    let (head, input) = uppercase(input)?;
    let (tail, input) = many(&input, alphabetic)?;

    Ok((format!("{head}{}", tail.iter().collect::<String>()), input))
}

/// Apply a parser zero or more times until it fails, returning a vector of the results.
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
/// use dragonfly::parser::{alphabetic, many, ParseError};
///
/// assert_eq!(many("abc", alphabetic), Ok((vec!['a', 'b', 'c'], "".to_string())));
/// assert_eq!(many("ab3", alphabetic), Ok((vec!['a', 'b'], "3".to_string())));
/// assert_eq!(many("a23", alphabetic), Ok((vec!['a'], "23".to_string())));
/// assert_eq!(many("123", alphabetic), Ok((vec![], "123".to_string())));
/// ```
pub fn many<T>(input: &str, parser: ParseFn<T>) -> ParseResult<Vec<T>> {
    let mut input = input.to_string();
    let mut result = vec![];

    while let Ok((value, new_input)) = parser(&input) {
        result.push(value);
        input = new_input;
    }

    Ok((result, input))
}

/// Apply a parser one or more times until it fails, returning a vector of the results.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `parser` - The parser to apply.
///
/// # Errors
///
/// * `ParseError` - If the parser fails to match at least once.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{alphabetic, many1, ParseError};
///
/// assert_eq!(many1("abc", alphabetic), Ok((vec!['a', 'b', 'c'], "".to_string())));
/// assert_eq!(many1("ab3", alphabetic), Ok((vec!['a', 'b'], "3".to_string())));
/// assert_eq!(many1("a23", alphabetic), Ok((vec!['a'], "23".to_string())));
///
/// assert_eq!(many1("123", alphabetic), Err(ParseError::UnmatchedCharPredicate {
///     actual: '1',
///     description: "should be alphabetic".to_string(),
/// }));
/// ```
pub fn many1<T>(input: &str, parser: ParseFn<T>) -> ParseResult<Vec<T>> {
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
/// * `ParseError::UnmatchedChoice` - If none of the parsers match.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{choice, literal, ParseError};
///
/// #[derive(Debug, Eq, PartialEq)]
/// enum Choice {
///    A,
///    B,
/// }
///
/// let input = "abc";
///
/// assert_eq!(
///     choice(
///         input,
///         vec![
///             |input| literal(input, "abc").map(|(_, rem)| (Choice::A, rem)),
///             |input| literal(input, "abc").map(|(_, rem)| (Choice::B, rem)),
///         ]
///     ),
///     Ok((Choice::A, "".to_string()))
/// );
///
/// assert_eq!(
///     choice(
///         input,
///         vec![
///             |input| literal(input, "abc").map(|(_, rem)| (Choice::B, rem)),
///             |input| literal(input, "abc").map(|(_, rem)| (Choice::A, rem)),
///         ]
///     ),
///     Ok((Choice::B, "".to_string()))
/// );
///
/// assert_eq!(
///     choice(
///         input,
///         vec![|input| literal(input, "def").map(|(_, rem)| (Choice::A, rem))]
///     ),
///     Err(ParseError::UnmatchedChoice)
/// );
///
/// assert_eq!(choice::<bool>(input, vec![]), Err(ParseError::UnmatchedChoice));
/// ```
pub fn choice<T>(input: &str, parsers: Vec<ParseFn<T>>) -> ParseResult<T> {
    for parser in parsers {
        if let Ok((value, input)) = parser(input) {
            return Ok((value, input));
        }
    }

    Err(ParseError::UnmatchedChoice)
}

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
/// * `ParseError::UnmatchedCharPredicate` - If the character does not fulfill the predicate.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{char_if, ParseError};
///
/// assert_eq!(
///     char_if("a", |c| c.is_ascii_lowercase(), "should be lowercase"),
///     Ok(('a', "".to_string()))
/// );
///
/// assert_eq!(
///     char_if("A", |c| c.is_ascii_lowercase(), "should be lowercase"),
///     Err(ParseError::UnmatchedCharPredicate {
///         actual: 'A',
///         description: "should be lowercase".to_string()
///     })
/// );
///
/// assert_eq!(
///     char_if("", |c| c.is_ascii_lowercase(), "should be lowercase"),
///     Err(ParseError::UnexpectedEof)
/// );
/// ```
pub fn char_if(input: &str, predicate: fn(char) -> bool, description: &str) -> ParseResult<char> {
    if let Some(char) = input.chars().next() {
        if predicate(char) {
            return Ok((char, input[1..].to_string()));
        }

        return Err(ParseError::UnmatchedCharPredicate {
            actual: char,
            description: description.to_string(),
        });
    }

    Err(ParseError::UnexpectedEof)
}

/// Parse one or more characters that fulfill the specified predicate into a string.
///
/// # Arguments
///
/// * `input` - The input string to parse.
/// * `predicate` - The predicate to apply.
/// * `description` - The description of the predicate, used in error messages.
///
/// # Errors
///
/// * `ParseError::UnmatchedCharPredicate` - If the first character does not fulfill the predicate.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{chars_if, ParseError};
///
/// assert_eq!(
///     chars_if("abc", |c| c.is_ascii_alphabetic(), "should be alphabetic"),
///     Ok(("abc".to_string(), "".to_string()))
/// );
///
/// assert_eq!(
///     chars_if("123", |c| c.is_ascii_alphabetic(), "should be alphabetic"),
///     Err(ParseError::UnmatchedCharPredicate {
///         actual: '1',
///         description: "should be alphabetic".to_string()
///     })
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
/// * `ParseError::UnmatchedCharPredicate` - If the character is not alphabetic.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{alphabetic, ParseError};
///
/// assert!(alphabetic("a").is_ok());
/// assert!(alphabetic("A").is_ok());
///
/// assert_eq!(
///     alphabetic("1"),
///     Err(ParseError::UnmatchedCharPredicate {
///         actual: '1',
///         description: "should be alphabetic".to_string()
///     })
/// );
/// ```
pub fn alphabetic(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_alphabetic(),
        "should be alphabetic",
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
/// * `ParseError::UnmatchedCharPredicate` - If the first character is not alphabetic.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// TODO: add examples
pub fn alphabetics(input: &str) -> ParseResult<String> {
    chars_if(
        input,
        |char| char.is_ascii_alphabetic(),
        "should be alphabetic",
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
/// * `ParseError::UnmatchedCharPredicate` - If the character is not alphanumeric.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{alphanumeric, ParseError};
///
/// assert!(alphanumeric("a").is_ok());
/// assert!(alphanumeric("A").is_ok());
/// assert!(alphanumeric("1").is_ok());
///
/// assert_eq!(
///     alphanumeric(" "),
///     Err(ParseError::UnmatchedCharPredicate {
///         actual: ' ',
///         description: "should be alphanumeric".to_string()
///     })
/// );
/// ```
pub fn alphanumeric(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_alphanumeric(),
        "should be alphanumeric",
    )
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
/// use dragonfly::parser::{literal, maybe, ParseError};
///
/// assert_eq!(
///     maybe(
///         "abc",
///         |input| literal(input, "abc")
///     ),
///     Ok((Some("abc".to_string()), "".to_string()))
/// );
///
/// assert_eq!(
///     maybe(
///         "def",
///         |input| literal(input, "abc")
///     ),
///     Ok((None, "def".to_string()))
/// );
/// ```
pub fn maybe<T>(input: &str, parser: ParseFn<T>) -> ParseResult<Option<T>> {
    match parser(input) {
        Ok((value, input)) => Ok((Some(value), input)),
        Err(_) => Ok((None, input.to_string())),
    }
}

/// Parse an ASCII decimal digit.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedCharPredicate` - If the character is not a digit.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{digit, ParseError};
///
/// assert!(digit("1").is_ok());
///
/// assert_eq!(
///     digit("a"),
///     Err(ParseError::UnmatchedCharPredicate {
///         actual: 'a',
///         description: "should be a decimal digit".to_string()
///     })
/// );
/// ```
pub fn digit(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_digit(),
        "should be a decimal digit",
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
/// * `ParseError::UnmatchedCharPredicate` - If the character is not lowercase.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{lowercase, ParseError};
///
/// assert!(lowercase("a").is_ok());
///
/// assert_eq!(lowercase("A"), Err(ParseError::UnmatchedCharPredicate {
///     actual: 'A',
///     description: "should be lowercase".to_string()
/// }));
/// ```
pub fn lowercase(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_lowercase(),
        "should be lowercase",
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
/// * `ParseError::UnmatchedCharPredicate` - If the character is not uppercase.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{uppercase, ParseError};
///
/// assert!(uppercase("A").is_ok());
///
/// assert_eq!(uppercase("a"), Err(ParseError::UnmatchedCharPredicate {
///     actual: 'a',
///     description: "should be uppercase".to_string()
/// }));
/// ```
pub fn uppercase(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_uppercase(),
        "should be uppercase",
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
/// * `ParseError::UnmatchedCharPredicate` - If the character is not whitespace.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{whitespace, ParseError};
///
/// assert!(whitespace(" ").is_ok());
/// assert!(whitespace("\t").is_ok());
/// assert!(whitespace("\r").is_ok());
/// assert!(whitespace("\n").is_ok());
///
/// assert_eq!(whitespace("a"), Err(ParseError::UnmatchedCharPredicate {
///     actual: 'a',
///     description: "should be whitespace".to_string()
/// }));
/// ```
pub fn whitespace(input: &str) -> ParseResult<char> {
    char_if(
        input,
        |char| char.is_ascii_whitespace(),
        "should be whitespace",
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
/// None. This parser always succeeds.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{spaces, ParseError};
///
/// assert_eq!(spaces(" \t\r\n"), Ok((vec![' ', '\t', '\r', '\n'], "".to_string())));
/// assert_eq!(spaces("abc"), Ok((vec![], "abc".to_string())));
/// ```
pub fn spaces(input: &str) -> ParseResult<Vec<char>> {
    many(input, whitespace)
}

/// Consume one or more whitespace characters.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedCharPredicate` - If there are no whitespace characters.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{spaces1, ParseError};
///
/// assert_eq!(spaces1(" \t\r\n"), Ok((" \t\r\n".to_string(), "".to_string())));
///
/// assert_eq!(spaces1("abc"), Err(ParseError::UnmatchedCharPredicate {
///     actual: 'a',
///     description: "should be whitespace".to_string()
/// }));
/// ```
pub fn spaces1(input: &str) -> ParseResult<String> {
    let (whitespace, input) = many1(input, whitespace)?;

    Ok((whitespace.iter().collect::<String>(), input))
}

/// Parse an opening brace.
///
/// # Arguments
///
/// * `input` - The input string to parse.
///
/// # Errors
///
/// * `ParseError::UnmatchedChar` - If the next character is not an opening brace.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{brace_open, ParseError};
///
/// assert_eq!(brace_open("{"), Ok(('{', "".to_string())));
/// assert_eq!(brace_open("}"), Err(ParseError::UnmatchedChar { expected: '{', actual: '}' }));
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
/// * `ParseError::UnmatchedChar` - If the next character is not a closing brace.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{brace_close, ParseError};
///
/// assert_eq!(brace_close("}"), Ok(('}', "".to_string())));
/// assert_eq!(brace_close("{"), Err(ParseError::UnmatchedChar { expected: '}', actual: '{' }));
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
/// * `ParseError::UnmatchedChar` - If the next character is not a colon.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{colon, ParseError};
///
/// assert_eq!(colon(":"), Ok((':', "".to_string())));
/// assert_eq!(colon("a"), Err(ParseError::UnmatchedChar { expected: ':', actual: 'a' }));
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
/// * `ParseError::UnmatchedChar` - If the next character is not an opening parenthesis.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{paren_open, ParseError};
///
/// assert_eq!(paren_open("("), Ok(('(', "".to_string())));
/// assert_eq!(paren_open(")"), Err(ParseError::UnmatchedChar { expected: '(', actual: ')' }));
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
/// * `ParseError::UnmatchedChar` - If the next character is not a closing parenthesis.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{paren_close, ParseError};
///
/// assert_eq!(paren_close(")"), Ok((')', "".to_string())));
/// assert_eq!(paren_close("("), Err(ParseError::UnmatchedChar { expected: ')', actual: '(' }));
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
/// * `ParseError::UnmatchedChar` - If the next character is not a dollar sign.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{dollar, ParseError};
///
/// assert_eq!(dollar("$"), Ok(('$', "".to_string())));
/// assert_eq!(dollar("a"), Err(ParseError::UnmatchedChar { expected: '$', actual: 'a' }));
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
/// * `ParseError::UnmatchedChar` - If the next character is not a comma.
/// * `ParseError::UnexpectedEof` - If the input is empty.
///
/// # Examples
///
/// ```rust
/// use dragonfly::parser::{comma, ParseError};
///
/// assert_eq!(comma(","), Ok((',', "".to_string())));
/// assert_eq!(comma("a"), Err(ParseError::UnmatchedChar { expected: ',', actual: 'a' }));
/// ```
pub fn comma(input: &str) -> ParseResult<char> {
    char(input, ',')
}
