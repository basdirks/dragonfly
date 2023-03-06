use std::path::PathBuf;

/// A command.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Command {
    /// Show the general help message.
    Help,
    /// Show the help message for a specific command.
    HelpCommand {
        /// The command to show the help message for.
        command: String,
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

impl Command {
    /// Create a new help command.
    ///
    /// # Arguments
    ///
    /// * `command` - The command to show the help message for.
    #[must_use]
    pub fn help_command<S>(command: S) -> Self
    where
        S: Into<String>,
    {
        Self::HelpCommand {
            command: command.into(),
        }
    }

    /// Create a new check command.
    ///
    /// # Arguments
    ///
    /// * `input` - The path to the input file.
    #[must_use]
    pub fn check<P>(input: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::Check {
            input: input.into(),
        }
    }

    /// Create a new build command.
    ///
    /// # Arguments
    ///
    /// * `input` - The path to the input file.
    /// * `output` - The output directory.
    #[must_use]
    pub fn build<P>(
        input: P,
        output: P,
    ) -> Self
    where
        P: Into<PathBuf>,
    {
        Self::Build {
            input: input.into(),
            output: output.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help() {
        assert_eq!(
            Command::help_command("foo"),
            Command::HelpCommand {
                command: "foo".to_owned()
            }
        );
    }

    #[test]
    fn test_check() {
        assert_eq!(
            Command::check("foo/bar/baz"),
            Command::Check {
                input: "foo/bar/baz".into()
            }
        );
    }

    #[test]
    fn test_build() {
        assert_eq!(
            Command::build("foo/bar/baz", "foo/bar/baz"),
            Command::Build {
                input: "foo/bar/baz".into(),
                output: "foo/bar/baz".into()
            }
        );
    }
}
