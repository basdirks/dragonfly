use std::fmt::Display;

/// Join a list of items as a comma separated string.
pub fn comma_separated<T: Display>(items: &[T]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(", ")
}

/// Join a list of items as a space separated string.
pub fn space_separated<T: Display>(items: &[T]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ")
}

/// Join a list of items as a newline separated string.
pub fn newline_separated<T: Display>(items: &[T]) -> String {
    items
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join("\n")
}

/// Join a list of items as a string of items separated by a given separator.
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
