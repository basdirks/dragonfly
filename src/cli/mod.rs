use self::command::Command;

/// Commands.
pub mod command;
/// I/O.
pub mod io;

/// Print help message for the `check` command.
#[must_use]
pub fn help_check_message() -> String {
    "

USAGE:
    dragonfly check <FILE>

"
    .trim()
    .to_owned()
}

/// Print help message for the `build` command.
#[must_use]
pub fn help_build_message() -> String {
    "

USAGE:
    dragonfly build [FLAGS] <FILE>

FLAGS:
    -o, --output <output-directory>   The output directory. (default: `./out`)

"
    .trim()
    .to_owned()
}

/// Print help message.
#[must_use]
pub fn help_message() -> String {
    "

USAGE:
    dragonfly [COMMAND] [ARGS]

COMMANDS:
    help                    Print this help message.
    help <COMMAND>          Print help message for a command.
    version                 Print the version number.
    check <FILE>            Check a source file for errors.
    build [FLAGS] <FILE>    Generate code from a source file. (see `help \
     build`).

"
    .trim()
    .to_owned()
}

/// Print version number.
#[must_use]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// Parse a `help` command.
///
/// # Arguments
///
/// * `args` - The command line arguments.
///
/// # Examples
///
/// If no arguments are given, show general help message:
///
/// ```rust
/// use dragonfly::cli::{
///     command::Command,
///     parse_help,
/// };
///
/// assert_eq!(parse_help(&mut [].iter()), Command::Help);
/// ```
///
/// Show help message for a specific command:
///
/// ```rust
/// use dragonfly::cli::{
///     command::Command,
///     parse_help,
/// };
///
/// assert_eq!(
///     parse_help(&mut ["check".to_owned()].iter()),
///     Command::HelpCommand {
///         command: "check".to_owned(),
///     }
/// );
/// ```
pub fn parse_help<'a>(args: &mut impl Iterator<Item = &'a String>) -> Command {
    args.next().map_or(Command::Help, |command| {
        Command::HelpCommand {
            command: command.to_string(),
        }
    })
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
/// assert_eq!(parse_args(&["dragonfly".to_owned()]), Command::Help);
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
///     parse_args(&["dragonfly".to_owned(), "help".to_owned()]),
///     Command::Help
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_owned(),
///         "help".to_owned(),
///         "check".to_owned()
///     ]),
///     Command::HelpCommand {
///         command: "check".to_owned(),
///     }
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_owned(),
///         "help".to_owned(),
///         "build".to_owned()
///     ]),
///     Command::HelpCommand {
///         command: "build".to_owned(),
///     }
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
///     parse_args(&["dragonfly".to_owned(), "version".to_owned()]),
///     Command::Version
/// );
/// ```
///
/// Check a source file for errors:
///
/// ```rust
/// use dragonfly::cli::{
///     command::Command,
///     parse_args,
/// };
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_owned(),
///         "check".to_owned(),
///         "file.dfly".to_owned()
///     ]),
///     Command::Check {
///         input: "file.dfly".to_owned(),
///     }
/// );
/// ```
///
/// Build from a source file:
///
/// ```rust
/// use dragonfly::cli::{
///     command::Command,
///     parse_args,
/// };
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_owned(),
///         "build".to_owned(),
///         "file.dfly".to_owned()
///     ]),
///     Command::Build {
///         input: "file.dfly".to_owned(),
///         output: None,
///     }
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_owned(),
///         "build".to_owned(),
///         "-o".to_owned(),
///         "output".to_owned(),
///         "file.dfly".to_owned(),
///     ]),
///     Command::Build {
///         input: "file.dfly".to_owned(),
///         output: Some("output".to_owned()),
///     }
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_owned(),
///         "build".to_owned(),
///         "--output".to_owned(),
///         "output".to_owned(),
///         "file.dfly".to_owned(),
///     ]),
///     Command::Build {
///         input: "file.dfly".to_owned(),
///         output: Some("output".to_owned()),
///     }
/// );
/// ```
#[must_use]
pub fn parse_args(args: &[String]) -> Command {
    let mut args = args.iter().skip(1);

    match args.next().map(String::as_str) {
        Some("help") => parse_help(&mut args),
        Some("version") => Command::Version,
        Some("check") => {
            args.next().map_or_else(
                || {
                    Command::HelpCommand {
                        command: "check".to_owned(),
                    }
                },
                |input| {
                    Command::Check {
                        input: input.to_string(),
                    }
                },
            )
        }
        Some("build") => {
            match args.next().map(String::as_str) {
                Some("-o" | "--output") => {
                    match (args.next(), args.next()) {
                        (Some(output), Some(input)) => {
                            Command::Build {
                                input: input.to_string(),
                                output: Some(output.to_string()),
                            }
                        }
                        _ => {
                            Command::HelpCommand {
                                command: "build".to_owned(),
                            }
                        }
                    }
                }
                Some(input) => {
                    Command::Build {
                        input: input.to_owned(),
                        output: None,
                    }
                }
                None => {
                    Command::HelpCommand {
                        command: "build".to_owned(),
                    }
                }
            }
        }
        _ => Command::Help,
    }
}
