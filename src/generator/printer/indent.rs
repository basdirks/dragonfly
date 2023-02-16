const GRAPHQL_INDENT: usize = 2;
const TYPESCRIPT_INDENT: usize = 4;

/// Return a string of spaces for the given indentation level.
#[must_use]
pub fn graphql(level: usize) -> String {
    " ".repeat(level * GRAPHQL_INDENT)
}

/// Return a string of spaces for the given indentation level.
#[must_use]
pub fn typescript(level: usize) -> String {
    " ".repeat(level * TYPESCRIPT_INDENT)
}
