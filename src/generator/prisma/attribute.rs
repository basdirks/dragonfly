use {
    super::{
        argument::Argument,
        Function,
        Value,
    },
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// A block attribute.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Block {
    /// The name of the group to which the attribute belongs.
    pub group: Option<String>,
    /// The name of the attribute.
    pub name: String,
    /// The fields of the attribute.
    pub arguments: Vec<Argument>,
}

impl Display for Block {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "  @@")?;

        if let Some(group) = &self.group {
            write!(f, "{group}.")?;
        }

        write!(f, "{name}", name = self.name)?;

        if !self.arguments.is_empty() {
            write!(f, "({})", comma_separated(&self.arguments))?;
        }

        Ok(())
    }
}

/// A field attribute.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Field {
    /// The name of the group to which the attribute belongs.
    pub group: Option<String>,
    /// The name of the attribute.
    pub name: String,
    /// The fields of the attribute.
    pub arguments: Vec<Argument>,
}

impl Field {
    /// Standard `@id` attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::attribute::Field;
    ///
    /// assert_eq!(Field::id().to_string(), "@id",);
    /// ```
    #[must_use]
    pub fn id() -> Self {
        Self {
            group: None,
            name: "id".to_owned(),
            arguments: vec![],
        }
    }

    /// Standard `@default(autoincrement())` attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::attribute::Field;
    ///
    /// assert_eq!(
    ///     Field::default_auto_increment().to_string(),
    ///     "@default(autoincrement())"
    /// );
    /// ```
    #[must_use]
    pub fn default_auto_increment() -> Self {
        Self {
            group: None,
            name: "default".to_owned(),
            arguments: vec![Argument {
                name: None,
                value: Value::Function(Function {
                    name: "autoincrement".to_owned(),
                    parameters: vec![],
                }),
            }],
        }
    }

    /// Standard `@default(now)` attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::attribute::Field;
    ///
    /// assert_eq!(Field::default_now().to_string(), "@default(now())");
    /// ```
    #[must_use]
    pub fn default_now() -> Self {
        Self {
            group: None,
            name: "default".to_owned(),
            arguments: vec![Argument {
                name: None,
                value: Value::Function(Function {
                    name: "now".to_owned(),
                    parameters: vec![],
                }),
            }],
        }
    }
}

impl Display for Field {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            group,
            name,
            arguments,
        } = self;

        let group = group
            .as_ref()
            .map_or_else(String::new, |group| format!("{group}."));

        let arguments = if arguments.is_empty() {
            String::new()
        } else {
            format!("({})", comma_separated(arguments))
        };

        write!(f, "@{group}{name}{arguments}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_block() {
        assert_eq!(
            Block {
                group: None,
                name: "foo".to_owned(),
                arguments: vec![],
            }
            .to_string(),
            "  @@foo"
        );

        assert_eq!(
            Block {
                group: Some("bar".to_owned()),
                name: "foo".to_owned(),
                arguments: vec![],
            }
            .to_string(),
            "  @@bar.foo"
        );

        assert_eq!(
            Block {
                group: None,
                name: "foo".to_owned(),
                arguments: vec![
                    Argument {
                        name: Some("foo".to_owned()),
                        value: Value::Keyword("bar".to_owned()),
                    },
                    Argument {
                        name: None,
                        value: Value::Keyword("baz".to_owned()),
                    },
                    Argument {
                        name: None,
                        value: Value::Function(Function {
                            name: "qux".to_owned(),
                            parameters: vec![],
                        }),
                    },
                ],
            }
            .to_string(),
            "  @@foo(foo: bar, baz, qux())"
        );
    }

    #[test]
    fn test_display_field() {
        assert_eq!(
            Field {
                group: None,
                name: "foo".to_owned(),
                arguments: vec![],
            }
            .to_string(),
            "@foo"
        );

        assert_eq!(
            Field {
                group: Some("bar".to_owned()),
                name: "foo".to_owned(),
                arguments: vec![],
            }
            .to_string(),
            "@bar.foo"
        );

        assert_eq!(
            Field {
                group: None,
                name: "foo".to_owned(),
                arguments: vec![
                    Argument {
                        name: Some("foo".to_owned()),
                        value: Value::Keyword("bar".to_owned()),
                    },
                    Argument {
                        name: None,
                        value: Value::Keyword("baz".to_owned()),
                    },
                    Argument {
                        name: None,
                        value: Value::Function(Function {
                            name: "qux".to_owned(),
                            parameters: vec![],
                        }),
                    },
                ],
            }
            .to_string(),
            "@foo(foo: bar, baz, qux())"
        );
    }
}
