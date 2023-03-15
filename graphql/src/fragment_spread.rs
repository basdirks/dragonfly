use {
    super::Directive,
    print::{
        Print,
        PrintInline,
    },
    std::{
        self,
        borrow::Cow,
        io,
    },
};

/// A fragment spread.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct FragmentSpread<'a> {
    /// The name of the fragment.
    pub name: Cow<'a, str>,
    /// The directives of the fragment.
    pub directives: Vec<Directive<'a>>,
}

impl Print for FragmentSpread<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}...{}", Self::indent(level), self.name)?;

        for directive in &self.directives {
            directive.print(f)?;
        }

        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Argument,
            Value,
        },
    };

    #[test]
    fn test_print() {
        let spread = FragmentSpread {
            name: "foo".into(),
            directives: Vec::new(),
        };

        let mut f = Vec::new();

        spread.print(0, &mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), "...foo\n");

        let spread = FragmentSpread {
            name: "daboi".into(),
            directives: vec![Directive {
                name: "is".into(),
                arguments: vec![Argument {
                    name: "a".into(),
                    value: Value::String("good boi.".into()),
                }],
            }],
        };

        let mut f = Vec::new();

        spread.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "...daboi @is(a: \"good boi.\")\n"
        );
    }
}
