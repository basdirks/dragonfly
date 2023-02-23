use self::command::Command;

/// Commands.
pub mod command;
/// I/O.
pub mod io;

/// Print help message for the `check` command.
#[must_use]
pub fn help_check_message() -> String {
    "

Usage: dragonfly check <source-file>

"
    .trim()
    .to_string()
}

/// Print help message for the `build` command.
#[must_use]
pub fn help_build_message() -> String {
    "

Usage: dragonfly build [flags] <source-file>

Flags:
  -o, --output <output-directory>   The output directory. (default: `./out`)

"
    .trim()
    .to_string()
}

/// Print help message.
#[must_use]
pub fn help_message() -> String {
    "

Usage: dragonfly [command] [command-args]

Commands:
  help                          Print this help message.
  help <command>                Print help message for a command.
  version                       Print the version number.
  check <source-file>           Check a source file for errors.
  build <flags> <source-file>   Generate code from a source file. (see `help \
     build`).

"
    .trim()
    .to_string()
}

/// Print version number.
#[must_use]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
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
///     parse_help(&mut ["check".to_string()].iter()),
///     Command::HelpCommand {
///         command: "check".to_string(),
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
/// assert_eq!(parse_args(&["dragonfly".to_string()]), Command::Help);
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
///     parse_args(&["dragonfly".to_string(), "help".to_string()]),
///     Command::Help
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_string(),
///         "help".to_string(),
///         "check".to_string()
///     ]),
///     Command::HelpCommand {
///         command: "check".to_string(),
///     }
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_string(),
///         "help".to_string(),
///         "build".to_string()
///     ]),
///     Command::HelpCommand {
///         command: "build".to_string(),
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
///     parse_args(&["dragonfly".to_string(), "version".to_string()]),
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
///         "dragonfly".to_string(),
///         "check".to_string(),
///         "file.dfly".to_string()
///     ]),
///     Command::Check {
///         input: "file.dfly".to_string(),
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
///         "dragonfly".to_string(),
///         "build".to_string(),
///         "file.dfly".to_string()
///     ]),
///     Command::Build {
///         input: "file.dfly".to_string(),
///         output: None,
///     }
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_string(),
///         "build".to_string(),
///         "-o".to_string(),
///         "output".to_string(),
///         "file.dfly".to_string(),
///     ]),
///     Command::Build {
///         input: "file.dfly".to_string(),
///         output: Some("output".to_string()),
///     }
/// );
///
/// assert_eq!(
///     parse_args(&[
///         "dragonfly".to_string(),
///         "build".to_string(),
///         "--output".to_string(),
///         "output".to_string(),
///         "file.dfly".to_string(),
///     ]),
///     Command::Build {
///         input: "file.dfly".to_string(),
///         output: Some("output".to_string()),
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
                        command: "check".to_string(),
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
                                command: "build".to_string(),
                            }
                        }
                    }
                }
                Some(input) => {
                    Command::Build {
                        input: input.to_string(),
                        output: None,
                    }
                }
                None => {
                    Command::HelpCommand {
                        command: "build".to_string(),
                    }
                }
            }
        }
        _ => Command::Help,
    }
}
