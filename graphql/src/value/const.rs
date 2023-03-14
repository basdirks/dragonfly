use {
    super::const_object_field::ConstObjectField,
    printer::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A constant value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Const<'a> {
    /// A boolean constant.
    Boolean(bool),
    /// An enum constant.
    Enum(Cow<'a, str>),
    /// A floating point constant.
    Float(Cow<'a, str>),
    /// An integer constant.
    Int(Cow<'a, str>),
    /// A list of constants.
    List(Vec<Const<'a>>),
    /// Null.
    Null,
    /// An object constant.
    Object(Vec<ConstObjectField<'a>>),
    /// A string constant.
    String(Cow<'a, str>),
}

impl PrintInline for Const<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        match self {
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Enum(value) | Self::Float(value) | Self::Int(value) => {
                write!(f, "{value}")
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
    use super::*;

    #[test]
    fn test_print_boolean() {
        let r#const = Const::Boolean(true);
        let mut f = Vec::new();

        r#const.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "true");
    }

    #[test]
    fn test_print_enum() {
        let r#const = Const::Enum("Foo.BAR".into());
        let mut f = Vec::new();

        r#const.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "Foo.BAR");
    }

    #[test]
    fn test_print_float() {
        let r#const = Const::Float("1.0".into());
        let mut f = Vec::new();

        r#const.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "1.0");
    }

    #[test]
    fn test_print_int() {
        let r#const = Const::Int("1".into());
        let mut f = Vec::new();

        r#const.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "1");
    }

    #[test]
    fn test_print_list() {
        let r#const =
            Const::List(vec![Const::Int("1".into()), Const::Int("2".into())]);

        let mut f = Vec::new();

        r#const.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "[1, 2]");
    }

    #[test]
    fn test_print_null() {
        let r#const = Const::Null;
        let mut f = Vec::new();

        r#const.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "null");
    }

    #[test]
    fn test_print_object() {
        let r#const = Const::Object(vec![ConstObjectField {
            name: "foo".into(),
            value: Const::Int("1".into()),
        }]);

        let mut f = Vec::new();

        r#const.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "{foo: 1}");
    }

    #[test]
    fn test_print_string() {
        let r#const = Const::String("foo".into());
        let mut f = Vec::new();

        r#const.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "\"foo\"");
    }
}
