use {
    super::Const,
    printer::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};
/// A constant object field.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ConstObjectField<'a> {
    /// The name of the field.
    pub name: Cow<'a, str>,
    /// The value of the field.
    pub value: Const<'a>,
}

impl PrintInline for ConstObjectField<'_> {
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
    fn test_new() {
        let field = ConstObjectField {
            name: "foo".into(),
            value: Const::String("bar".into()),
        };

        assert_eq!(field.name, Cow::from("foo"));
        assert_eq!(field.value, Const::String("bar".into()));
    }

    #[test]
    fn test_print() {
        let field = ConstObjectField {
            name: "foo".into(),
            value: Const::String("bar".into()),
        };

        let mut f = Vec::new();

        field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: \"bar\"");
    }
}
