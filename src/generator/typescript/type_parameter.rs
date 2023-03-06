use {
    super::r#type::Type,
    crate::generator::printer::common::comma_separated,
    std::fmt::Display,
};

/// A type parameter.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct TypeParameter {
    /// The name of the type parameter.
    pub identifier: String,
    /// The types that the type parameter extends.
    pub type_references: Vec<Type>,
}

impl TypeParameter {
    /// Create a new type parameter.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The name of the type parameter.
    /// * `type_references` - The types that the type parameter extends.
    #[must_use]
    pub fn new(
        identifier: &str,
        type_references: &[Type],
    ) -> Self {
        Self {
            identifier: identifier.to_owned(),
            type_references: type_references.to_vec(),
        }
    }
}

impl Display for TypeParameter {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            identifier,
            type_references,
        } = self;

        if type_references.is_empty() {
            write!(f, "{identifier}")
        } else {
            write!(
                f,
                "{identifier} extends {}",
                comma_separated(type_references)
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::typescript::Keyword,
    };

    #[test]
    fn test_new() {
        assert_eq!(
            TypeParameter::new("T", &[]),
            TypeParameter {
                identifier: "T".to_owned(),
                type_references: vec![],
            }
        );
    }

    #[test]
    fn test_display() {
        assert_eq!(TypeParameter::new("T", &[]).to_string(), "T");

        assert_eq!(
            TypeParameter::new(
                "T",
                &[
                    Type::Keyword(Keyword::Number),
                    Type::Keyword(Keyword::String)
                ]
            )
            .to_string(),
            "T extends number, string"
        );
    }
}
