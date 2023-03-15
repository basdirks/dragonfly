use {
    super::{
        Const,
        ConstDirective,
        Type,
    },
    print::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A variable definition.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Variable<'a> {
    /// The name of the variable.
    pub name: Cow<'a, str>,
    /// The type of the variable.
    pub r#type: Type<'a>,
    /// The default value of the variable.
    pub default_value: Option<Const<'a>>,
    /// The directives of the variable.
    pub directives: Vec<ConstDirective<'a>>,
}

impl PrintInline for Variable<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "${}: ", self.name)?;
        self.r#type.print(f)?;

        if let Some(default_value) = &self.default_value {
            write!(f, " = ")?;
            default_value.print(f)?;
        }

        if !self.directives.is_empty() {
            for directive in &self.directives {
                directive.print(f)?;
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_no_default() {
        let variable = Variable {
            name: "foo".into(),
            r#type: Type::Name("String".into()),
            default_value: None,
            directives: Vec::new(),
        };

        let mut f = Vec::new();

        variable.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "$foo: String");
    }

    #[test]
    fn test_print_with_default() {
        let variable = Variable {
            name: "foo".into(),
            r#type: Type::Name("String".into()),
            default_value: Some(Const::String("bar".into())),
            directives: Vec::new(),
        };

        let mut f = Vec::new();

        variable.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "$foo: String = \"bar\"");
    }

    #[test]
    fn test_print_with_directives() {
        let variable = Variable {
            name: "foo".into(),
            r#type: Type::NonNull(Box::new(Type::List(Box::new(
                Type::NonNull(Box::new(Type::Name("String".into()))),
            )))),
            default_value: None,
            directives: vec![
                ConstDirective {
                    name: "bar".into(),
                    arguments: Vec::new(),
                },
                ConstDirective {
                    name: "baz".into(),
                    arguments: Vec::new(),
                },
            ],
        };

        let mut f = Vec::new();

        variable.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "$foo: [String!]! @bar @baz");
    }
}
