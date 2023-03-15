use {
    super::value::Value,
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A field argument.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Argument<'a> {
    /// The name of the argument.
    pub name: Cow<'a, str>,
    /// The value of the argument.
    pub value: Value<'a>,
}

impl PrintInline for Argument<'_> {
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
        crate::ObjectField,
    };

    #[test]
    fn test_new() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::String("bar".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: \"bar\"");
    }

    #[test]
    fn test_boolean() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::Boolean(true),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: true");
    }

    #[test]
    fn test_enum() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::Enum("bar".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: bar");
    }

    #[test]
    fn test_float() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::Float("1.23".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: 1.23");
    }

    #[test]
    fn test_int() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::Int("123".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: 123");
    }

    #[test]
    fn test_list() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::List(vec![
                Value::String("bar".into()),
                Value::String("baz".into()),
            ]),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: [\"bar\", \"baz\"]");
    }

    #[test]
    fn test_object() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::Object(vec![
                ObjectField {
                    name: "bar".into(),
                    value: Value::String("baz".into()),
                },
                ObjectField {
                    name: "qux".into(),
                    value: Value::List(vec![
                        Value::String("baz".into()),
                        Value::String("qux".into()),
                    ]),
                },
            ]),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "foo: {bar: \"baz\", qux: [\"baz\", \"qux\"]}"
        );
    }

    #[test]
    fn test_string() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::String("bar".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: \"bar\"");
    }

    #[test]
    fn test_variable() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::Variable("bar".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: $bar");
    }
}
