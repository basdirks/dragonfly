use std::fmt::Display;

/// A JavaScript literal.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Literal {
    /// A BigInt literal: a number followed by `n`.
    BigInt(String),
    /// A boolean literal: `true` or `false`.
    Boolean(bool),
    /// A number literal.
    Number(String),
    /// A string literal: characters surrounded by double quotes.
    String(String),
}

impl Display for Literal {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::BigInt(value) => write!(f, "{value}n"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "\"{value}\""),
        }
    }
}
