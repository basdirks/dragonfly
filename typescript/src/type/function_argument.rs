use {
    super::Type,
    printer::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A function argument.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FunctionArgument<'a> {
    /// The name of the argument.
    pub name: Cow<'a, str>,
    /// The type of the argument.
    pub r#type: Type<'a>,
}

impl PrintInline for FunctionArgument<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}: ", self.name)?;
        self.r#type.print(f)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Keyword,
            Type,
        },
    };

    #[test]
    fn test_print() {
        let argument = FunctionArgument {
            name: "foo".into(),
            r#type: Type::Array(Box::new(Type::Keyword(Keyword::Number))),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(f, b"foo: Array<number>");
    }
}
