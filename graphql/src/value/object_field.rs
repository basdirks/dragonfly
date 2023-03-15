use {
    super::Value,
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};
/// An object field.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectField<'a> {
    /// The name of the field.
    pub name: Cow<'a, str>,
    /// The value of the field.
    pub value: Value<'a>,
}

impl PrintInline for ObjectField<'_> {
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
    use super::*;

    #[test]
    fn test_boolean() {
        let object_field = ObjectField {
            name: "foo".into(),
            value: Value::Boolean(true),
        };

        let mut f = Vec::new();

        object_field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: true");
    }

    #[test]
    fn test_enum() {
        let object_field = ObjectField {
            name: "foo".into(),
            value: Value::Enum("bar".into()),
        };

        let mut f = Vec::new();

        object_field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: bar");
    }

    #[test]
    fn test_float() {
        let object_field = ObjectField {
            name: "foo".into(),
            value: Value::Float("1.23".into()),
        };

        let mut f = Vec::new();

        object_field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: 1.23");
    }

    #[test]
    fn test_int() {
        let object_field = ObjectField {
            name: "foo".into(),
            value: Value::Int("123".into()),
        };

        let mut f = Vec::new();

        object_field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: 123");
    }

    #[test]
    fn test_list() {
        let object_field = ObjectField {
            name: "foo".into(),
            value: Value::List(vec![Value::Int("123".into())]),
        };

        let mut f = Vec::new();

        object_field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: [123]");
    }

    #[test]
    fn test_null() {
        let object_field = ObjectField {
            name: "foo".into(),
            value: Value::Null,
        };

        let mut f = Vec::new();

        object_field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: null");
    }

    #[test]
    fn test_object() {
        let object_field = ObjectField {
            name: "foo".into(),
            value: Value::Object(vec![ObjectField {
                name: "bar".into(),
                value: Value::Int("123".into()),
            }]),
        };

        let mut f = Vec::new();

        object_field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: {bar: 123}");
    }

    #[test]
    fn test_string() {
        let object_field = ObjectField {
            name: "foo".into(),
            value: Value::String("bar".into()),
        };

        let mut f = Vec::new();

        object_field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: \"bar\"");
    }

    #[test]
    fn test_variable() {
        let object_field = ObjectField {
            name: "foo".into(),
            value: Value::Variable("bar".into()),
        };

        let mut f = Vec::new();

        object_field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: $bar");
    }
}
