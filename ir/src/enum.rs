use {
    std::borrow::Cow,
    token_set::TokenSet,
};

/// An enum type.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Enum<'a> {
    /// The name of the enum.
    pub name: Cow<'a, str>,
    /// The values of the enum.
    pub values: TokenSet,
}

impl<'a> From<ast::Enum<'a>> for Enum<'a> {
    fn from(ast: ast::Enum<'a>) -> Self {
        let mut r#enum = Self {
            name: ast.name.clone(),
            values: TokenSet::new(),
        };

        for value in ast.values.iter() {
            let _: bool = r#enum.values.insert(value);
        }

        r#enum
    }
}
