use {
    super::Type,
    crate::generator::printer::common::comma_separated,
    std::fmt::Display,
};
/// An expression with type arguments.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExpressionWithTypeArguments {
    /// The name of the expression. Usually pascal case.
    pub identifier: String,
    /// The type arguments of the expression.
    pub type_arguments: Vec<Type>,
}

impl ExpressionWithTypeArguments {
    /// Create a new expression with type arguments.
    ///
    /// # Arguments
    ///
    /// * `identifier` - The name of the expression. Usually pascal case.
    /// * `type_arguments` - The type arguments of the expression.
    #[must_use]
    pub fn new(
        identifier: &str,
        type_arguments: &[Type],
    ) -> Self {
        Self {
            identifier: identifier.to_owned(),
            type_arguments: type_arguments.to_vec(),
        }
    }
}

impl Display for ExpressionWithTypeArguments {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            identifier,
            type_arguments,
        } = self;

        if type_arguments.is_empty() {
            write!(f, "{identifier}")
        } else {
            write!(f, "{identifier}<{}>", comma_separated(type_arguments))
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
    fn new() {
        assert_eq!(
            ExpressionWithTypeArguments::new("Foo", &[]),
            ExpressionWithTypeArguments {
                identifier: "Foo".to_owned(),
                type_arguments: vec![],
            }
        );
    }

    #[test]
    fn test_display_without_arguments() {
        assert_eq!(
            ExpressionWithTypeArguments::new("Foo", &[]).to_string(),
            "Foo"
        );
    }

    #[test]
    fn test_display_with_arguments() {
        assert_eq!(
            ExpressionWithTypeArguments::new("Foo", &[Keyword::string()])
                .to_string(),
            "Foo<string>"
        );
    }
}
