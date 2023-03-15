use {
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A function.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Function<'a> {
    /// The name of the function.
    pub name: Cow<'a, str>,
    /// The parameters of the function.
    pub parameters: Vec<Value<'a>>,
}

impl PrintInline for Function<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}", self.name)?;
        write!(f, "(")?;
        PrintInline::intercalate(self.parameters.clone(), f, ", ")?;
        write!(f, ")")
    }
}

/// A value.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Value<'a> {
    /// An array of values.
    Array(Vec<Value<'a>>),
    /// A boolean value.
    Boolean(bool),
    /// A keyword.
    Keyword(Cow<'a, str>),
    /// A function.
    Function(Function<'a>),
    /// A number.
    Number(Cow<'a, str>),
    /// A string.
    String(Cow<'a, str>),
}

impl PrintInline for Value<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        match self {
            Self::Array(values) => {
                write!(f, "[")?;
                PrintInline::intercalate(values.clone(), f, ", ")?;
                write!(f, "]")
            }
            Self::Boolean(value) => write!(f, "{value}"),
            Self::Function(function) => function.print(f),
            Self::Number(value) | Self::Keyword(value) => write!(f, "{value}"),
            Self::String(value) => write!(f, "\"{value}\""),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_array() {
        let value = Value::Array(vec![
            Value::String("foo".into()),
            Value::String("bar".into()),
        ]);

        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "[\"foo\", \"bar\"]");
    }

    #[test]
    fn test_print_boolean() {
        let value = Value::Boolean(true);
        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "true");
    }

    #[test]
    fn test_print_function() {
        let value = Value::Function(Function {
            name: "foo".into(),
            parameters: Vec::new(),
        });

        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo()");
    }

    #[test]
    fn test_print_number() {
        let value = Value::Number("1".into());
        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "1");
    }

    #[test]
    fn test_print_string() {
        let value = Value::String("foo".into());
        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "\"foo\"");
    }

    #[test]
    fn test_print_keyword() {
        let value = Value::Keyword("foo".into());
        let mut f = Vec::new();

        value.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo");
    }
}
