/// A scalar type.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    /// A boolean.
    Boolean,
    /// A date time.
    DateTime,
    /// A floating point number.
    Float,
    /// An integer.
    Int,
    /// A string.
    String,
}
