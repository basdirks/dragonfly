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
        /// The input file.
        input: String,
    },
    /// Compile a source file and generate code.
    Build {
        /// The input file.
        input: String,
        /// The output directory.
        output: Option<String>,
    },
}
