use crate::generator::{
    printer::{
        indent,
        Print,
    },
    typescript::r#type::Type,
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
    ) -> String {
        let Self {
            identifier,
            r#type: type_reference,
            optional,
        } = self;

        let optional = if *optional { "?" } else { "" };
        let indent = indent::typescript(level);

        format!("{indent}{identifier}{optional}: {type_reference};")
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
        assert_eq!(
            Property {
                identifier: "foo".to_owned(),
                r#type: Type::Keyword(Keyword::String),
                optional: false,
            }
            .print(0),
            "foo: string;"
        );

        assert_eq!(
            Property {
                identifier: "foo".to_owned(),
                r#type: Type::Keyword(Keyword::String),
                optional: true,
            }
            .print(0),
            "foo?: string;"
        );
    }
}
