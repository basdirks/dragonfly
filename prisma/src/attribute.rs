use {
    super::{
        argument::Argument,
        Function,
        Value,
    },
    print::{
        Print,
        PrintInline,
    },
    std::{
        borrow::Cow,
        io,
    },
};

/// A block attribute.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Block<'a> {
    /// The name of the group to which the attribute belongs.
    pub group: Option<Cow<'a, str>>,
    /// The name of the attribute.
    pub name: Cow<'a, str>,
    /// The fields of the attribute.
    pub arguments: Vec<Argument<'a>>,
}

impl Print for Block<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{}@@", Self::indent(level))?;

        if let Some(group) = &self.group {
            write!(f, "{group}.")?;
        }

        write!(f, "{name}", name = self.name)?;

        if !self.arguments.is_empty() {
            write!(f, "(")?;
            PrintInline::intercalate(self.arguments.clone(), f, ", ")?;
            write!(f, ")")?;
        }

        writeln!(f)
    }
}

/// A field attribute.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Field<'a> {
    /// The name of the group to which the attribute belongs.
    pub group: Option<Cow<'a, str>>,
    /// The name of the attribute.
    pub name: Cow<'a, str>,
    /// The fields of the attribute.
    pub arguments: Vec<Argument<'a>>,
}

impl<'a> Field<'a> {
    /// Standard `@id` attribute.
    ///
    /// # Examples
    /// ```rust
    /// use {
    ///     print::PrintInline,
    ///     prisma::attribute::Field,
    /// };
    ///
    /// let attribute = Field::id();
    /// let mut f = Vec::new();
    ///
    /// attribute.print(&mut f).unwrap();
    ///
    /// assert_eq!(String::from_utf8(f).unwrap(), " @id");
    /// ```
    #[must_use]
    pub fn id() -> Self {
        Self {
            group: None,
            name: "id".into(),
            arguments: Vec::new(),
        }
    }

    /// Standard `@unique` attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     print::PrintInline,
    ///     prisma::attribute::Field,
    /// };
    ///
    /// let attribute = Field::unique();
    /// let mut f = Vec::new();
    ///
    /// attribute.print(&mut f).unwrap();
    ///
    /// assert_eq!(String::from_utf8(f).unwrap(), " @unique");
    /// ```
    #[must_use]
    pub fn unique() -> Self {
        Self {
            group: None,
            name: "unique".into(),
            arguments: Vec::new(),
        }
    }

    /// Standard `@default(autoincrement())` attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     print::PrintInline,
    ///     prisma::attribute::Field,
    /// };
    ///
    /// let attribute = Field::default_auto_increment();
    /// let mut f = Vec::new();
    ///
    /// attribute.print(&mut f).unwrap();
    ///
    /// assert_eq!(String::from_utf8(f).unwrap(), " @default(autoincrement())");
    /// ```
    #[must_use]
    pub fn default_auto_increment() -> Self {
        Self {
            group: None,
            name: "default".into(),
            arguments: vec![Argument {
                name: None,
                value: Value::Function(Function {
                    name: "autoincrement".into(),
                    parameters: Vec::new(),
                }),
            }],
        }
    }

    /// Standard `@default(now)` attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use {
    ///     print::PrintInline,
    ///     prisma::attribute::Field,
    /// };
    ///
    /// let attribute = Field::default_now();
    /// let mut f = Vec::new();
    ///
    /// attribute.print(&mut f).unwrap();
    ///
    /// assert_eq!(String::from_utf8(f).unwrap(), " @default(now())");
    /// ```
    #[must_use]
    pub fn default_now() -> Self {
        Self {
            group: None,
            name: "default".into(),
            arguments: vec![Argument {
                name: None,
                value: Value::Function(Function {
                    name: "now".into(),
                    parameters: Vec::new(),
                }),
            }],
        }
    }
}

impl PrintInline for Field<'_> {
    fn print(
        &self,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        let Self {
            group,
            name,
            arguments,
        } = self;

        let group = group
            .as_ref()
            .map_or_else(String::new, |group| format!("{group}."));

        write!(f, " @{group}{name}")?;

        if !arguments.is_empty() {
            write!(f, "(")?;
            PrintInline::intercalate(arguments.clone(), f, ", ")?;
            write!(f, ")")?;
        };

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_block() {
        let block = Block {
            name: "foo".into(),
            arguments: vec![
                Argument {
                    name: Some("foo".into()),
                    value: Value::Keyword("bar".into()),
                },
                Argument {
                    name: None,
                    value: Value::Keyword("baz".into()),
                },
                Argument {
                    name: None,
                    value: Value::Function(Function {
                        name: "qux".into(),
                        parameters: Vec::new(),
                    }),
                },
            ],
            group: Some("bar".into()),
        };

        let mut f = Vec::new();

        block.print(1, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "  @@bar.foo(foo: bar, baz, qux())\n"
        );
    }

    #[test]
    fn test_print_field_attribute_basic() {
        let field = Field {
            group: None,
            name: "foo".into(),
            arguments: Vec::new(),
        };

        let mut f = Vec::new();

        field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), " @foo");
    }

    #[test]
    fn test_print_field_attribute_with_group() {
        let field = Field {
            group: Some("bar".into()),
            name: "foo".into(),
            arguments: Vec::new(),
        };

        let mut f = Vec::new();

        field.print(&mut f).unwrap();

        assert_eq!(String::from_utf8(f).unwrap(), " @bar.foo");
    }

    #[test]
    fn test_print_field_attribute_with_arguments() {
        let field = Field {
            name: "foo".into(),
            arguments: vec![
                Argument {
                    name: Some("foo".into()),
                    value: Value::Keyword("bar".into()),
                },
                Argument {
                    name: None,
                    value: Value::Keyword("baz".into()),
                },
                Argument {
                    name: None,
                    value: Value::Function(Function {
                        name: "qux".into(),
                        parameters: Vec::new(),
                    }),
                },
            ],
            group: None,
        };

        let mut f = Vec::new();

        field.print(&mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            " @foo(foo: bar, baz, qux())"
        );
    }
}
