const PSL_INDENT: usize = 2;
const GRAPHQL_INDENT: usize = 2;
const TYPESCRIPT_INDENT: usize = 4;

/// Return a string of spaces for the given indentation level.
///
/// # Arguments
///
/// * `level` - The indentation level.
///
/// # Examples
///
/// ```rust
/// use dragonfly::generator::printer::indent;
///
/// assert_eq!(indent::graphql(0), "");
/// assert_eq!(indent::graphql(1), "  ");
/// assert_eq!(indent::graphql(2), "    ");
/// ```
#[must_use]
pub fn graphql(level: usize) -> String {
    " ".repeat(level * GRAPHQL_INDENT)
}

/// Return a string of spaces for the given indentation level.
///
/// # Arguments
///
/// * `level` - The indentation level.
///
/// # Examples
///
/// ```rust
/// use dragonfly::generator::printer::indent;
///
/// assert_eq!(indent::typescript(0), "");
/// assert_eq!(indent::typescript(1), "    ");
/// assert_eq!(indent::typescript(2), "        ");
/// ```
#[must_use]
pub fn typescript(level: usize) -> String {
    " ".repeat(level * TYPESCRIPT_INDENT)
}

/// Return a string of spaces for the given indentation level.
///
/// # Arguments
///
/// * `level` - The indentation level.
///
/// # Examples
///
/// ```rust
/// use dragonfly::generator::printer::indent;
///
/// assert_eq!(indent::psl(0), "");
/// assert_eq!(indent::psl(1), "  ");
/// assert_eq!(indent::psl(2), "    ");
/// ```
#[must_use]
pub fn psl(level: usize) -> String {
    " ".repeat(level * PSL_INDENT)
}
