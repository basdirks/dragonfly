use {
    crate::value::Function,
    printer::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A field type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum FieldType<'a> {
    /// A name.
    Name(Cow<'a, str>),
    /// A function.
    Function(Function<'a>),
}

impl PrintInline for FieldType<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        match self {
            Self::Name(name) => write!(f, "{name}"),
            Self::Function(function) => function.print(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Value,
    };

    #[test]
    fn test_name() {
        let field_type = FieldType::Name("foo".into());
        let mut f = Vec::new();

        field_type.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo");
    }

    #[test]
    fn test_function() {
        let field_type = FieldType::Function(Function {
            name: "foo".into(),
            parameters: vec![
                Value::String("bar".into()),
                Value::String("baz".into()),
            ],
        });

        let mut f = Vec::new();

        field_type.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo(\"bar\", \"baz\")");
    }
}
