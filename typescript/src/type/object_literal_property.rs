use {
    super::Type,
    printer::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// An object literal property.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct ObjectLiteralProperty<'a> {
    /// The name of the property.
    pub name: Cow<'a, str>,
    /// The type of the property.
    pub r#type: Type<'a>,
}

impl PrintInline for ObjectLiteralProperty<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}: ", self.name)?;
        self.r#type.print(f)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Keyword,
    };

    #[test]
    fn test_print() {
        let property = ObjectLiteralProperty {
            name: "country".into(),
            r#type: Type::ObjectLiteral(
                vec![
                    ObjectLiteralProperty {
                        name: "name".into(),
                        r#type: Type::TypeReference {
                            identifier: "CountryName".into(),
                            type_arguments: Vec::new(),
                        },
                    },
                    ObjectLiteralProperty {
                        name: "languages".into(),
                        r#type: Type::Array(Box::new(Type::Keyword(
                            Keyword::String,
                        ))),
                    },
                ]
                .into_iter()
                .collect(),
            ),
        };

        let mut f = Vec::new();

        property.print(&mut f).unwrap();

        assert_eq!(
            "country: { name: CountryName, languages: Array<string> }",
            String::from_utf8(f).unwrap()
        );
    }
}
