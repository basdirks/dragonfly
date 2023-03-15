use {
    super::Value,
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A model attribute argument.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Argument<'a> {
    /// The name of the argument.
    pub name: Option<Cow<'a, str>>,
    /// The value of the argument.
    pub value: Value<'a>,
}

impl PrintInline for Argument<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self { name, value } = self;

        match &name {
            Some(name) => {
                write!(f, "{name}: ")?;
                value.print(f)
            }
            None => value.print(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Function,
    };

    #[test]
    fn test_unnamed() {
        let argument = Argument {
            name: None,
            value: Value::String("bar".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "\"bar\"");
    }

    #[test]
    fn test_array() {
        let argument = Argument {
            name: Some("foo".into()),
            value: Value::Array(vec![
                Value::String("bar".into()),
                Value::String("baz".into()),
            ]),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: [\"bar\", \"baz\"]");
    }

    #[test]
    fn test_boolean() {
        let argument = Argument {
            name: Some("foo".into()),
            value: Value::Boolean(true),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: true");
    }

    #[test]
    fn test_function() {
        let argument = Argument {
            name: Some("foo".into()),
            value: Value::Function(Function {
                name: "bar".into(),
                parameters: vec![
                    Value::String("baz".into()),
                    Value::String("qux".into()),
                ],
            }),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: bar(\"baz\", \"qux\")");
    }

    #[test]
    fn test_keyword() {
        let argument = Argument {
            name: Some("foo".into()),
            value: Value::Keyword("bar".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: bar");
    }

    #[test]
    fn test_number() {
        let argument = Argument {
            name: Some("foo".into()),
            value: Value::Number("42".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: 42");
    }

    #[test]
    fn test_string() {
        let argument = Argument {
            name: Some("foo".into()),
            value: Value::String("bar".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: \"bar\"");
    }
}
