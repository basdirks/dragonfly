use {
    super::Type,
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};
/// An expression with type arguments.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ExpressionWithTypeArguments<'a> {
    /// The name of the expression. Usually pascal case.
    pub identifier: Cow<'a, str>,
    /// The type arguments of the expression.
    pub type_arguments: Vec<Type<'a>>,
}

impl PrintInline for ExpressionWithTypeArguments<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            identifier,
            type_arguments,
        } = self;

        write!(f, "{identifier}")?;

        if !type_arguments.is_empty() {
            write!(f, "<")?;
            PrintInline::intercalate(type_arguments.clone(), f, ", ")?;
            write!(f, ">")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Keyword,
    };

    #[test]
    fn test_print_without_arguments() {
        let expression = ExpressionWithTypeArguments {
            identifier: "Foo".into(),
            type_arguments: Vec::new(),
        };

        let mut f = Vec::new();

        expression.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Foo");
    }

    #[test]
    fn test_print_with_arguments() {
        let expression = ExpressionWithTypeArguments {
            identifier: "Foo".into(),
            type_arguments: vec![Type::Keyword(Keyword::String)],
        };

        let mut f = Vec::new();

        expression.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Foo<string>");
    }
}
