use {
    crate::io::{
        check_file,
        compile,
    },
    std::{
        borrow::Cow,
        io::{
            self,
            Write,
        },
        path::PathBuf,
    },
};

/// The help message for the `check` command.
pub const HELP_CHECK_MESSAGE: &str = "USAGE: dragonfly check <FILE>";

/// The help message for the `build` command.
pub const HELP_BUILD_MESSAGE: &str = "USAGE: dragonfly build [FLAGS] <FILE>

FLAGS:
    -o, --output <output-directory>   The output directory. Default: `./out`.";

/// The general help message.
pub const HELP_MESSAGE: &str = "USAGE:
    dragonfly [COMMAND] [ARGS]

COMMANDS:
    help                    Print this help message.
    help <COMMAND>          Print help message for a command.
    version                 Print the version number.
    check <FILE>            Check a source file for errors.
    build [FLAGS] <FILE>    Generate code from a source file. See `help build`.";

/// Print version number.
#[must_use]
pub fn version() -> String {
    env!("CARGO_PKG_VERSION").to_owned()
}

/// A command.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Command<'a> {
    /// Show the general help message.
    Help,
    /// Show the help message for a specific command.
    HelpCommand {
        /// The command to show the help message for.
        command: Cow<'a, str>,
    },
    /// Show the version number.
    Version,
    /// Check a source file for errors.
    Check {
        /// The path to the input file.
        input: PathBuf,
    },
    /// Compile a source file and generate code.
    Build {
        /// The path to the input file.
        input: PathBuf,
        /// The output directory.
        output: PathBuf,
    },
}

impl<'a> Command<'a> {
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
    /// use crate::cli::command::Command;
    ///
    /// assert_eq!(Command::parse_help(&mut [].iter()), Command::Help);
    /// ```
    ///
    /// Show help message for a specific command:
    ///
    /// ```rust
    /// use crate::cli::command::Command;
    ///
    /// assert_eq!(
    ///     Command::parse_help(&mut ["check".to_owned()].iter()),
    ///     Command::HelpCommand {
    ///         command: "check".into(),
    ///     }
    /// );
    /// ```
    pub fn parse_help<T>(args: &mut T) -> Self
    where
        T: Iterator<Item = &'a String>,
    {
        args.next().map_or(Command::Help, |command| {
            Command::HelpCommand {
                command: command.into(),
            }
        })
    }

    /// Parse command line arguments.
    ///
    /// # Arguments
    ///
    /// * `args` - The command line arguments.
    #[must_use]
    pub fn parse_args(args: &'a [String]) -> Self {
        let mut args = args.iter();

        args.nth(1).map_or(Command::Help, |arg| {
            match arg.as_str() {
                "help" => Command::parse_help(&mut args),
                "version" => Command::Version,
                "check" => {
                    args.next().map_or_else(
                        || {
                            Command::HelpCommand {
                                command: "check".into(),
                            }
                        },
                        |input| {
                            Command::Check {
                                input: PathBuf::from(input),
                            }
                        },
                    )
                }
                "build" => {
                    args.next().map_or_else(
                        || {
                            Command::HelpCommand {
                                command: "build".into(),
                            }
                        },
                        |arg| {
                            if matches!(arg.as_str(), "-o" | "--output") {
                                match (args.next(), args.next()) {
                                    (Some(output), Some(input)) => {
                                        Command::Build {
                                            input: PathBuf::from(input),
                                            output: PathBuf::from(output),
                                        }
                                    }
                                    _ => {
                                        Command::HelpCommand {
                                            command: "build".into(),
                                        }
                                    }
                                }
                            } else {
                                Command::Build {
                                    input: PathBuf::from(arg),
                                    output: PathBuf::from("./out"),
                                }
                            }
                        },
                    )
                }
                _ => Command::Help,
            }
        })
    }

    /// Execute a command.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to execute.
    ///
    /// # Errors
    ///
    /// If an error occurs during execution, an error is returned.
    pub fn execute(
        &self,
        f: &mut dyn Write,
    ) -> Result<(), io::Error> {
        match self {
            Self::Help => {
                writeln!(f, "{HELP_MESSAGE}")
            }
            Self::HelpCommand { command } => {
                if command == "build" {
                    writeln!(f, "{HELP_BUILD_MESSAGE}")
                } else if command == "check" {
                    writeln!(f, "{HELP_CHECK_MESSAGE}")
                } else {
                    writeln!(f, "Unknown command `{command}`.")?;
                    writeln!(f, "{HELP_MESSAGE}")
                }
            }
            Self::Version => {
                writeln!(f, "{}", version())
            }
            Self::Build { input, output } => {
                if let Err(error) = compile(input, output) {
                    writeln!(
                        f,
                        "An error occurred during compilation. {error}"
                    )?;
                }

                Ok(())
            }
            Self::Check { input } => {
                if let Err(error) = check_file(input) {
                    writeln!(
                        f,
                        "Error while checking `{}`.\n{error}",
                        input.display()
                    )
                } else {
                    writeln!(f, "No errors found in `{}`.", input.display())
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_help() {
        let mut buffer = Vec::new();

        Command::Help.execute(&mut buffer).unwrap();

        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            format!("{HELP_MESSAGE}\n")
        );
    }

    #[test]
    fn test_execute_help_build() {
        let mut buffer = Vec::new();

        Command::HelpCommand {
            command: "build".into(),
        }
        .execute(&mut buffer)
        .unwrap();

        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            format!("{HELP_BUILD_MESSAGE}\n")
        );
    }

    #[test]
    fn test_execute_help_check() {
        let mut buffer = Vec::new();

        Command::HelpCommand {
            command: "check".into(),
        }
        .execute(&mut buffer)
        .unwrap();

        assert_eq!(
            String::from_utf8(buffer).unwrap(),
            format!("{HELP_CHECK_MESSAGE}\n")
        );
    }

    #[test]
    fn test_parse_no_args() {
        assert_eq!(
            Command::parse_args(&["dragonfly".to_owned()]),
            Command::Help
        );
    }

    #[test]
    fn test_parse_help() {
        assert_eq!(
            Command::parse_args(&["dragonfly".to_owned(), "help".to_owned()]),
            Command::Help
        );
    }

    #[test]
    fn test_parse_help_check() {
        assert_eq!(
            Command::parse_args(&[
                "dragonfly".to_owned(),
                "help".to_owned(),
                "check".to_owned()
            ]),
            Command::HelpCommand {
                command: "check".into(),
            }
        );
    }

    #[test]
    fn test_parse_help_build() {
        assert_eq!(
            Command::parse_args(&[
                "dragonfly".to_owned(),
                "help".to_owned(),
                "build".to_owned()
            ]),
            Command::HelpCommand {
                command: "build".into(),
            }
        );
    }

    #[test]
    fn test_parse_check_no_args() {
        assert_eq!(
            Command::parse_args(&["dragonfly".to_owned(), "check".to_owned()]),
            Command::HelpCommand {
                command: "check".into(),
            }
        );
    }

    #[test]
    fn test_parse_build_no_args() {
        assert_eq!(
            Command::parse_args(&["dragonfly".to_owned(), "build".to_owned()]),
            Command::HelpCommand {
                command: "build".into(),
            }
        );
    }

    #[test]
    fn test_parse_version() {
        assert_eq!(
            Command::parse_args(&[
                "dragonfly".to_owned(),
                "version".to_owned()
            ]),
            Command::Version
        );
    }

    #[test]
    fn test_parse_build() {
        assert_eq!(
            Command::parse_args(&[
                "dragonfly".to_owned(),
                "build".to_owned(),
                "test.dfly".to_owned()
            ]),
            Command::Build {
                input: PathBuf::from("test.dfly"),
                output: PathBuf::from("./out"),
            }
        );
    }

    #[test]
    fn test_parse_build_output() {
        assert_eq!(
            Command::parse_args(&[
                "dragonfly".to_owned(),
                "build".to_owned(),
                "-o".to_owned(),
                "test".to_owned(),
                "test.dfly".to_owned()
            ]),
            Command::Build {
                input: PathBuf::from("test.dfly"),
                output: PathBuf::from("test"),
            }
        );
    }

    #[test]
    fn test_parse_check() {
        assert_eq!(
            Command::parse_args(&[
                "dragonfly".to_owned(),
                "check".to_owned(),
                "test.dfly".to_owned()
            ]),
            Command::Check {
                input: PathBuf::from("test.dfly"),
            }
        );
    }
}
