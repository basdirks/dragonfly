use {
    crate::r#type::Type,
    print::{
        Print,
        PrintInline,
    },
    std::{
        borrow::Cow,
        io,
    },
};

/// An interface property.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Property<'a> {
    /// The name of the property. Usually camel case.
    pub identifier: Cow<'a, str>,
    /// The type of the property.
    pub r#type: Type<'a>,
    /// Whether the property is optional.
    pub optional: bool,
}

impl Print for Property<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            identifier,
            r#type: type_reference,
            optional,
        } = self;

        write!(f, "{}{identifier}", Self::indent(level))?;

        if *optional {
            write!(f, "?")?;
        }

        write!(f, ": ")?;
        type_reference.print(f)?;
        writeln!(f, ";")
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::r#type::{
            Keyword,
            Type,
        },
    };

    #[test]
    fn test_print() {
        let property = Property {
            identifier: "foo".into(),
            r#type: Type::Keyword(Keyword::String),
            optional: false,
        };

        let mut f = Vec::new();

        property.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: string;\n");

        let property = Property {
            identifier: "foo".into(),
            r#type: Type::Keyword(Keyword::String),
            optional: true,
        };

        let mut f = Vec::new();

        property.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo?: string;\n");
    }
}
