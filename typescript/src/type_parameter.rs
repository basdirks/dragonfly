use {
    super::r#type::Type,
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A type parameter.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct TypeParameter<'a> {
    /// The name of the type parameter.
    pub identifier: Cow<'a, str>,
    /// The types that the type parameter extends.
    pub type_references: Vec<Type<'a>>,
}

impl PrintInline for TypeParameter<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            identifier,
            type_references,
        } = self;

        write!(f, "{identifier}")?;

        if !type_references.is_empty() {
            write!(f, " extends ")?;

            PrintInline::intercalate(type_references.clone(), f, ", ")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Keyword,
    };

    #[test]
    fn test_print_no_type_reference() {
        let type_parameter = TypeParameter {
            identifier: "T".into(),
            type_references: Vec::new(),
        };

        let mut f = Vec::new();

        type_parameter.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "T");
    }

    #[test]
    fn test_print_one_type_reference() {
        let type_parameter = TypeParameter {
            identifier: "T".into(),
            type_references: vec![Type::Keyword(Keyword::Number)],
        };

        let mut f = Vec::new();

        type_parameter.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "T extends number");
    }

    #[test]
    fn test_print_two_type_references() {
        let type_parameter = TypeParameter {
            identifier: "T".into(),
            type_references: vec![
                Type::Keyword(Keyword::Number),
                Type::Keyword(Keyword::String),
            ],
        };

        let mut f = Vec::new();

        type_parameter.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "T extends number, string");
    }
}
