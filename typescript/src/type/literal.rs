use {
    printer::PrintInline,
    std::io,
};

/// A JavaScript literal.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
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

impl PrintInline for Literal {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        match self {
            Self::BigInt(value) => write!(f, "{value}n"),
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Number(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "\"{value}\""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_bigint() {
        let literal = Literal::BigInt("1".to_owned());
        let mut f = Vec::new();

        literal.print(&mut f).unwrap();

        assert_eq!(f, b"1n");
    }

    #[test]
    fn test_print_boolean() {
        let literal = Literal::Boolean(true);
        let mut f = Vec::new();

        literal.print(&mut f).unwrap();

        assert_eq!(f, b"true");
    }

    #[test]
    fn test_print_number() {
        let literal = Literal::Number("1.0".to_owned());
        let mut f = Vec::new();

        literal.print(&mut f).unwrap();

        assert_eq!(f, b"1.0");
    }

    #[test]
    fn test_print_string() {
        let literal = Literal::String("hello".to_owned());
        let mut f = Vec::new();

        literal.print(&mut f).unwrap();

        assert_eq!(f, b"\"hello\"");
    }
}
