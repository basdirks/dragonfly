use self::command::Command;

/// Commands.
pub mod command;

/// Print usage summary.
#[must_use]
pub fn usage() -> String {
    "

Usage: dragonfly [options] [file]

Options:
    -h, --help      Print this help message.
    -v, --version   Print the version number.
    -o, --output    Specify the output directory.

If no output directory is specified, the current directory is used.

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
/// assert_eq!(parse_args(&["dragonfly".to_string()]), Some(Command::Help));
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
///     Some(Command::Help)
/// );
///
/// assert_eq!(
///     parse_args(&["dragonfly".to_string(), "--help".to_string()]),
///     Some(Command::Help)
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
///     Some(Command::Version)
/// );
///
/// assert_eq!(
///     parse_args(&["dragonfly".to_string(), "--version".to_string()]),
///     Some(Command::Version)
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
///     Some(Command::Compile {
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
///     Some(Command::Compile {
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
///     Some(Command::Compile {
///         input: "file.dfly".to_string(),
///         output: Some("output".to_string()),
///     })
/// );
/// ```
#[must_use]
pub fn parse_args(args: &[String]) -> Option<Command> {
    let mut args = args.iter().skip(1);

    if let Some(arg) = args.next() {
        match arg.as_str() {
            "-h" | "--help" => {
                return Some(Command::Help);
            }
            "-v" | "--version" => {
                return Some(Command::Version);
            }
            "-o" | "--output" => {
                let output = args.next()?;
                let input = args.next()?;

                return Some(Command::Compile {
                    input: input.to_string(),
                    output: Some(output.to_string()),
                });
            }
            _ => {
                let input = arg.to_string();

                if args.next().is_some() {
                    return None;
                }

                return Some(Command::Compile {
                    input,
                    output: None,
                });
            }
        }
    }

    Some(Command::Help)
}
