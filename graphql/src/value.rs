pub use self::{
    const_object_field::ConstObjectField,
    object_field::ObjectField,
    r#const::Const,
};
use {
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// Const values.
pub mod r#const;
/// Const object fields.
pub mod const_object_field;
/// Object fields.
pub mod object_field;

/// A value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Value<'a> {
    /// A boolean value.
    Boolean(bool),
    /// An enum value.
    Enum(Cow<'a, str>),
    /// A floating point value.
    Float(Cow<'a, str>),
    /// An integer value.
    Int(Cow<'a, str>),
    /// A list of values.
    List(Vec<Value<'a>>),
    /// Null.
    Null,
    /// An object of fields.
    Object(Vec<ObjectField<'a>>),
    /// A string value.
    String(Cow<'a, str>),
    /// A variable.
    Variable(Cow<'a, str>),
}

impl PrintInline for Value<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        match self {
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Enum(value) | Self::Float(value) | Self::Int(value) => {
                write!(f, "{value}")
            }
            Self::Variable(value) => {
                write!(f, "${value}")
            }
            Self::List(values) => {
                write!(f, "[")?;
                PrintInline::intercalate(values.clone(), f, ", ")?;
                write!(f, "]")
            }
            Self::Null => write!(f, "null"),
            Self::Object(fields) => {
                write!(f, "{{")?;
                PrintInline::intercalate(fields.clone(), f, ", ")?;
                write!(f, "}}")
            }
            Self::String(value) => write!(f, "\"{value}\""),
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Argument,
    };

    #[test]
    fn test_argument() {
        let argument = Argument {
            name: "foo".into(),
            value: Value::Int("1".into()),
        };

        let mut f = Vec::new();

        argument.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: 1");
    }

    #[test]
    fn test_enum() {
        let value = Value::Enum("Foo.BAR".into());
        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Foo.BAR");
    }

    #[test]
    fn test_float() {
        let value = Value::Float("1.0".into());
        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "1.0");
    }

    #[test]
    fn test_int() {
        let value = Value::Int("1".into());
        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "1");
    }

    #[test]
    fn test_list() {
        let value =
            Value::List(vec![Value::Int("1".into()), Value::Int("2".into())]);

        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "[1, 2]");
    }

    #[test]
    fn test_object() {
        let value = Value::Object(vec![
            ObjectField {
                name: "foo".into(),
                value: Value::Int("1".into()),
            },
            ObjectField {
                name: "bar".into(),
                value: Value::String("barrr".into()),
            },
        ]);

        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "{foo: 1, bar: \"barrr\"}");
    }

    #[test]
    fn test_string() {
        let value = Value::String("foo".into());
        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "\"foo\"");
    }

    #[test]
    fn test_variable() {
        let value = Value::Variable("foo".into());
        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "$foo");
    }
}
