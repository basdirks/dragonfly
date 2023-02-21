/// A CLI error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    /// An error occurred while parsing command line arguments.
    ParseArgs,
    /// An error occurred while reading a file.
    ReadFile,
    /// An error occurred while parsing a file.
    ParseFile,
    /// An error occurred while writing a file.
    WriteFile,
}
