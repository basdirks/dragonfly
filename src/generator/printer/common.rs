use std::fmt::Display;

/// Join a list of items as a comma separated string.
///
/// # Arguments
///
/// * `items` - The list of items.
///
/// # Examples
///
/// ```rust
/// use dragonfly::generator::printer::comma_separated;
///
/// let items = vec!["foo", "bar", "baz"];
///
/// assert_eq!(comma_separated(&items), "foo, bar, baz");
/// ```
pub fn comma_separated<T: Display>(items: &[T]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

/// Join a list of items as a space separated string.
///
/// # Arguments
///
/// * `items` - The list of items.
///
/// # Examples
///
/// ```rust
/// use dragonfly::generator::printer::space_separated;
///
/// let items = vec!["foo", "bar", "baz"];
///
/// assert_eq!(space_separated(&items), "foo bar baz");
/// ```
pub fn space_separated<T: Display>(items: &[T]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Join a list of items as a newline separated string.
///
/// # Arguments
///
/// * `items` - The list of items.
///
/// # Examples
///
/// ```rust
/// use dragonfly::generator::printer::newline_separated;
///
/// let items = vec!["foo", "bar", "baz"];
///
/// assert_eq!(
///     newline_separated(&items),
///     "\
/// foo
/// bar
/// baz"
/// );
/// ```
pub fn newline_separated<T: Display>(items: &[T]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Join a list of items as a string of items separated by a given separator.
///
/// # Arguments
///
/// * `items` - The list of items.
/// * `separator` - The separator.
///
/// # Examples
///
/// ```rust
/// use dragonfly::generator::printer::separated;
///
/// let items = vec!["foo", "bar", "baz"];
/// let separator = " &&& ";
///
/// assert_eq!(separated(&items, separator), "foo &&& bar &&& baz");
/// ```
pub fn separated<T: Display>(
    items: &[T],
    separator: &str,
) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(separator)
}
