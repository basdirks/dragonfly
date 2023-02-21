pub use self::error::Error;
use {
    self::command::Command,
    crate::ast::Ast,
    std::{
        fs::read_to_string,
        path::Path,
    },
};

/// Commands.
pub mod command;
/// CLI errors.
pub mod error;

/// Parse a file as a Dragonfly AST.
///
/// # Arguments
///
/// * `path` - The path to the file to parse.
///
/// # Errors
///
/// * Returns `Error::ReadFile` if the file could not be read.
/// * Returns `Error::ParseError` if the file cannot be parsed as Dragonfly.
pub fn parse_file(path: &Path) -> Result<Ast, Error> {
    let source = read_to_string(path).map_err(|_| Error::ReadFile)?;
    let (ast, _) = Ast::parse(&source).map_err(|_| Error::ParseFile)?;

    Ok(ast)
}

/// Print usage summary.
#[must_use]
pub fn usage() -> String {
    "

Usage: dragonfly [options] [file]

Options:
    -h, --help      Print this help message.
    -v, --version   Print the version number.
    -o, --output    Specify the output directory.

If no output directory is specified, the program will write to a new directory 
called \"out\" in the current directory.

"
    .trim()
    .to_string()
}

/// Print version number.
#[must_use]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

/// Parse command line arguments.
///
/// # Arguments
///
/// * `args` - The command line arguments.
///
/// # Errors
///
/// Returns `Error::ParseArgs` if the arguments could not be parsed.
///
/// # Examples
///
/// If no arguments are given, show help message:
///
/// ```rust
/// use dragonfly::cli::{
///     command::Command,
///     parse_args,
/// };
///
/// assert_eq!(parse_args(&["dragonfly".to_string()]), Ok(Command::Help));
/// ```
///
/// Show help message:
///
/// ```rust
/// use dragonfly::cli::{
///     command::Command,
///     parse_args,
/// };
///
/// assert_eq!(
///     parse_args(&["dragonfly".to_string(), "-h".to_string()]),
///     Ok(Command::Help)
/// );
///
/// assert_eq!(
///     parse_args(&["dragonfly".to_string(), "--help".to_string()]),
///     Ok(Command::Help)
/// );
/// ```
///
/// Show version number:
///
/// ```rust
/// use dragonfly::cli::{
///     command::Command,
///     parse_args,
/// };
///
/// assert_eq!(
///     parse_args(&["dragonfly".to_string(), "-v".to_string()]),
///     Ok(Command::Version)
/// );
///
/// assert_eq!(
///     parse_args(&["dragonfly".to_string(), "--version".to_string()]),
///     Ok(Command::Version)
/// );
/// ```
///
/// Compile a file:
///
/// ```rust
/// use dragonfly::cli::{
///     command::Command,
///     parse_args,
/// };
///
/// assert_eq!(
///     parse_args(&["dragonfly".to_string(), "file.dfly".to_string()]),
///     Ok(Command::Compile {
///         input: "file.dfly".to_string(),
///         output: None,
///     })
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_string(),
///         "-o".to_string(),
///         "output".to_string(),
///         "file.dfly".to_string(),
///     ]),
///     Ok(Command::Compile {
///         input: "file.dfly".to_string(),
///         output: Some("output".to_string()),
///     })
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_string(),
///         "--output".to_string(),
///         "output".to_string(),
///         "file.dfly".to_string(),
///     ]),
///     Ok(Command::Compile {
///         input: "file.dfly".to_string(),
///         output: Some("output".to_string()),
///     })
/// );
/// ```
pub fn parse_args(args: &[String]) -> Result<Command, Error> {
    let mut args = args.iter().skip(1);

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                return Ok(Command::Help);
            }
            "-v" | "--version" => {
                return Ok(Command::Version);
            }
            "-o" | "--output" => {
                let output = args.next().ok_or(Error::ParseArgs)?;
                let input = args.next().ok_or(Error::ParseArgs)?;

                return Ok(Command::Compile {
                    input: input.to_string(),
                    output: Some(output.to_string()),
                });
            }
            _ => {
                let input = arg.to_string();

                if args.next().is_some() {
                    return Err(Error::ParseArgs);
                }

                return Ok(Command::Compile {
                    input,
                    output: None,
                });
            }
        }
    }

    Ok(Command::Help)
}
