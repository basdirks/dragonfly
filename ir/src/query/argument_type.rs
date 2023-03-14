use {
    crate::Type,
    std::borrow::Cow,
};

/// The type of an argument.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ArgumentType<'a> {
    /// A reference to an enum.
    Enum(Cow<'a, str>),
    /// A scalar type.
    Type(Type),
}
