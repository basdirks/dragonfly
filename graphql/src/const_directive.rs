use {
    super::ConstArgument,
    printer::PrintInline,
    std::{
        borrow::Cow,
        io,
    },
};

/// A constant directive.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Directive<'a> {
    /// The name of the directive.
    pub name: Cow<'a, str>,
    /// The arguments of the directive.
    pub arguments: Vec<ConstArgument<'a>>,
}

impl PrintInline for Directive<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, " @{}", self.name)?;

        if !self.arguments.is_empty() {
            write!(f, "(")?;
            PrintInline::intercalate(self.arguments.clone(), f, ", ")?;
            write!(f, ")")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Const,
            ConstArgument,
        },
    };

    #[test]
    fn test_print_no_arguments() {
        let directive = Directive {
            name: "foo".into(),
            arguments: Vec::new(),
        };

        let mut f = Vec::new();

        directive.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), " @foo");
    }

    #[test]
    fn test_print_one_argument() {
        let directive = Directive {
            name: "foo".into(),
            arguments: vec![ConstArgument {
                name: "bar".into(),
                value: Const::String("baz".into()),
            }],
        };

        let mut f = Vec::new();

        directive.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), " @foo(bar: \"baz\")");
    }

    #[test]
    fn test_print_multiple_arguments() {
        let directive = Directive {
            name: "foo".into(),
            arguments: vec![
                ConstArgument {
                    name: "bar".into(),
                    value: Const::String("baz".into()),
                },
                ConstArgument {
                    name: "qux".into(),
                    value: Const::String("quux".into()),
                },
            ],
        };

        let mut f = Vec::new();

        directive.print(&mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            " @foo(bar: \"baz\", qux: \"quux\")",
        );
    }
}
