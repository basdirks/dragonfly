use {
    crate::generator::{
        printer::{
            indent,
            Print,
        },
        typescript::r#type::Type,
    },
    std::io,
};

/// An interface property.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Property {
    /// The name of the property. Usually camel case.
    pub identifier: String,
    /// The type of the property.
    pub r#type: Type,
    /// Whether the property is optional.
    pub optional: bool,
}

impl Property {
    /// Create a new property.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The name of the property.
    /// * `r#type` - The type of the property.
    /// * `optional` - Whether the property is optional.
    #[must_use]
    pub fn new(
        identifier: &str,
        r#type: Type,
        optional: bool,
    ) -> Self {
        Self {
            identifier: identifier.to_owned(),
            r#type,
            optional,
        }
    }

    /// Create an optional property.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The name of the property.
    /// * `r#type` - The type of the property.
    #[must_use]
    pub fn optional(
        identifier: &str,
        r#type: Type,
    ) -> Self {
        Self::new(identifier, r#type, true)
    }

    /// Create a required property.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The name of the property.
    /// * `r#type` - The type of the property.
    #[must_use]
    pub fn required(
        identifier: &str,
        r#type: Type,
    ) -> Self {
        Self::new(identifier, r#type, false)
    }
}

impl Print for Property {
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

        let indent = indent::typescript(level);

        write!(f, "{indent}{identifier}")?;

        if *optional {
            write!(f, "?")?;
        }

        writeln!(f, ": {type_reference};")
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::typescript::r#type::{
            Keyword,
            Type,
        },
    };

    #[test]
    fn test_new() {
        assert_eq!(
            Property::new("foo", Type::Keyword(Keyword::String), false),
            Property {
                identifier: "foo".to_owned(),
                r#type: Type::Keyword(Keyword::String),
                optional: false,
            }
        );
    }

    #[test]
    fn test_optional() {
        assert_eq!(
            Property::optional("foo", Type::Keyword(Keyword::String)),
            Property {
                identifier: "foo".to_owned(),
                r#type: Type::Keyword(Keyword::String),
                optional: true,
            }
        );
    }

    #[test]
    fn test_required() {
        assert_eq!(
            Property::required("foo", Type::Keyword(Keyword::String)),
            Property {
                identifier: "foo".to_owned(),
                r#type: Type::Keyword(Keyword::String),
                optional: false,
            }
        );
    }

    #[test]
    fn test_print() {
        let property = Property {
            identifier: "foo".to_owned(),
            r#type: Type::Keyword(Keyword::String),
            optional: false,
        };

        let mut f = Vec::new();

        property.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo: string;\n");

        let property = Property {
            identifier: "foo".to_owned(),
            r#type: Type::Keyword(Keyword::String),
            optional: true,
        };

        let mut f = Vec::new();

        property.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "foo?: string;\n");
    }
}
