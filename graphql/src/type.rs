use {
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// GraphQL types.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Type<'a> {
    /// A list type.
    List(Box<Type<'a>>),
    /// A non-null type.
    NonNull(Box<Type<'a>>),
    /// A name.
    Name(Cow<'a, str>),
}

impl PrintInline for Type<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        match self {
            Self::NonNull(inner) => {
                inner.print(f)?;
                write!(f, "!")
            }
            Self::List(inner) => {
                write!(f, "[")?;
                inner.print(f)?;
                write!(f, "]")
            }
            Self::Name(name) => write!(f, "{name}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_non_null() {
        let type_ = Type::NonNull(Box::new(Type::Name("String".into())));
        let mut f = Vec::new();

        type_.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "String!");
    }

    #[test]
    fn test_print_list() {
        let type_ = Type::List(Box::new(Type::Name("String".into())));
        let mut f = Vec::new();

        type_.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "[String]");
    }

    #[test]
    fn test_print_name() {
        let type_ = Type::Name("String".into());
        let mut f = Vec::new();

        type_.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "String");
    }
}
