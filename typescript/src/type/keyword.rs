use {
    print::PrintInline,
    std::io,
};

/// A TypeScript type keyword.
#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Keyword {
    /// The `any` type.
    Any,
    /// The `bigint` type.
    BigInt,
    /// The `boolean` type.
    Boolean,
    /// The `intrinsic` type.
    Intrinsic,
    /// The `never` type.
    Never,
    /// The `null` type.
    Null,
    /// The `number` type.
    Number,
    /// The `object` type.
    Object,
    /// The `string` type.
    String,
    /// The `symbol` type.
    Symbol,
    /// An `undefined` type.
    Undefined,
    /// An `unknown` type.
    Unknown,
    /// The `void` type.
    Void,
}

impl PrintInline for Keyword {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        match self {
            Self::Any => write!(f, "any"),
            Self::BigInt => write!(f, "bigint"),
            Self::Boolean => write!(f, "boolean"),
            Self::Intrinsic => write!(f, "intrinsic"),
            Self::Never => write!(f, "never"),
            Self::Null => write!(f, "null"),
            Self::Number => write!(f, "number"),
            Self::Object => write!(f, "object"),
            Self::String => write!(f, "string"),
            Self::Symbol => write!(f, "symbol"),
            Self::Undefined => write!(f, "undefined"),
            Self::Unknown => write!(f, "unknown"),
            Self::Void => write!(f, "void"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_any() {
        let keyword = Keyword::Any;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"any");
    }

    #[test]
    fn test_print_bigint() {
        let keyword = Keyword::BigInt;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"bigint");
    }

    #[test]
    fn test_print_boolean() {
        let keyword = Keyword::Boolean;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"boolean");
    }

    #[test]
    fn test_print_intrinsic() {
        let keyword = Keyword::Intrinsic;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"intrinsic");
    }

    #[test]
    fn test_print_never() {
        let keyword = Keyword::Never;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"never");
    }

    #[test]
    fn test_print_null() {
        let keyword = Keyword::Null;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"null");
    }

    #[test]
    fn test_print_number() {
        let keyword = Keyword::Number;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"number");
    }

    #[test]
    fn test_print_object() {
        let keyword = Keyword::Object;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"object");
    }

    #[test]
    fn test_print_string() {
        let keyword = Keyword::String;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"string");
    }

    #[test]
    fn test_print_symbol() {
        let keyword = Keyword::Symbol;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"symbol");
    }

    #[test]
    fn test_print_undefined() {
        let keyword = Keyword::Undefined;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"undefined");
    }

    #[test]
    fn test_print_unknown() {
        let keyword = Keyword::Unknown;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"unknown");
    }

    #[test]
    fn test_print_void() {
        let keyword = Keyword::Void;
        let mut f = Vec::new();

        keyword.print(&mut f).unwrap();

        assert_eq!(f, b"void");
    }
}
