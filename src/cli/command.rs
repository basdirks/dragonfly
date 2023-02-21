/// A command.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Command {
    /// Show the help message.
    Help,
    /// Show the version number.
    Version,
    /// Compile a file.
    Compile {
        /// The input file.
        input: String,
        /// The output directory.
        output: Option<String>,
    },
}
