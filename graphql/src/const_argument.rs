use {
    super::value::Const,
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A const argument.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConstArgument<'a> {
    /// The name of the argument.
    pub name: Cow<'a, str>,
    /// The value of the argument.
    pub value: Const<'a>,
}

impl PrintInline for ConstArgument<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}: ", self.name)?;
        self.value.print(f)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::ConstObjectField,
    };

    #[test]
    fn test_boolean() {
        let argument = ConstArgument {
            name: "foo".into(),
            value: Const::Boolean(true),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: true");
    }

    #[test]
    fn test_enum() {
        let argument = ConstArgument {
            name: "foo".into(),
            value: Const::Enum("bar".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: bar");
    }

    #[test]
    fn test_float() {
        let argument = ConstArgument {
            name: "foo".into(),
            value: Const::Float("1.0".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: 1.0");
    }

    #[test]
    fn test_int() {
        let argument = ConstArgument {
            name: "foo".into(),
            value: Const::Int("1".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: 1");
    }

    #[test]
    fn test_list() {
        let argument = ConstArgument {
            name: "foo".into(),
            value: Const::List(vec![
                Const::String("bar".into()),
                Const::String("baz".into()),
            ]),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: [\"bar\", \"baz\"]");
    }

    #[test]
    fn test_object() {
        let argument = ConstArgument {
            name: "foo".into(),
            value: Const::Object(vec![
                ConstObjectField {
                    name: "bar".into(),
                    value: Const::String("baz".into()),
                },
                ConstObjectField {
                    name: "qux".into(),
                    value: Const::String("quux".into()),
                },
            ]),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "foo: {bar: \"baz\", qux: \"quux\"}",
        );
    }

    #[test]
    fn test_string() {
        let argument = ConstArgument {
            name: "foo".into(),
            value: Const::String("bar".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: \"bar\"");
    }
}
