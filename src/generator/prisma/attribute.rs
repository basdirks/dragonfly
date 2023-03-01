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

impl Block {
    /// Create a new block attribute.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the attribute.
    /// * `arguments` - The fields of the attribute.
    /// * `group` - The name of the group to which the attribute belongs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::attribute::Block;
    ///
    /// let attribute = Block::new("foo", &[], None);
    ///
    /// assert_eq!(attribute.name, "foo");
    /// assert!(attribute.arguments.is_empty());
    /// assert!(attribute.group.is_none());
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        arguments: &[Argument],
        group: Option<&str>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            arguments: arguments.iter().map(ToOwned::to_owned).collect(),
            group: group.map(ToString::to_string),
        }
    }
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
    /// Create a new field attribute.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the attribute.
    /// * `arguments` - The fields of the attribute.
    /// * `group` - The name of the group to which the attribute belongs.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::attribute::Field;
    ///
    /// let attribute = Field::new("foo", &[], None);
    ///
    /// assert_eq!(attribute.name, "foo");
    /// assert!(attribute.arguments.is_empty());
    /// assert!(attribute.group.is_none());
    ///
    /// let attribute =
    ///     Field::new("foo", &[Argument::boolean("bar", true)], Some("bar"));
    ///
    /// assert_eq!(attribute.name, "foo");
    /// assert_eq!(attribute.arguments.len(), 1);
    /// assert_eq!(attribute.group, Some("bar".to_owned()));
    /// ```
    #[must_use]
    pub fn new(
        name: &str,
        arguments: &[Argument],
        group: Option<&str>,
    ) -> Self {
        Self {
            name: name.to_owned(),
            arguments: arguments.iter().map(ToOwned::to_owned).collect(),
            group: group.map(ToString::to_string),
        }
    }

    /// Standard `@id` attribute.
    ///
    /// # Examples
    /// ```rust
    /// use dragonfly::generator::prisma::attribute::Field;
    ///
    /// assert_eq!(Field::id().to_string(), "@id");
    /// ```
    #[must_use]
    pub fn id() -> Self {
        Self {
            group: None,
            name: "id".to_owned(),
            arguments: vec![],
        }
    }

    /// Standard `@unique` attribute.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::attribute::Field;
    ///
    /// assert_eq!(Field::unique().to_string(), "@unique");
    /// ```
    #[must_use]
    pub fn unique() -> Self {
        Self {
            group: None,
            name: "unique".to_owned(),
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

    /// A relation attribute.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the relation.
    /// * `references` - The fields to which the relation refers.
    /// * `fields` - The relation scalar fields.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::generator::prisma::attribute::Field;
    ///
    /// assert_eq!(
    ///     Field::relation("foo", "bar", "baz").to_string(),
    ///     "@relation(name: \"foo\", references: [bar], fields: [baz])"
    /// );
    /// ```
    #[must_use]
    pub fn relation(
        name: &str,
        references: &[&str],
        fields: &[&str],
    ) -> Self {
        let mut arguments = Vec::new();

        arguments.push(Argument::string("name", name));

        if !fields.is_empty() {
            arguments.push(Argument::array(
                "fields",
                &fields
                    .iter()
                    .map(|field| Value::keyword(field))
                    .collect::<Vec<Value>>(),
            ));
        }

        if !references.is_empty() {
            arguments.push(Argument::array(
                "references",
                &references
                    .iter()
                    .map(|reference| Value::keyword(reference))
                    .collect::<Vec<Value>>(),
            ));
        }

        Self {
            group: None,
            name: "relation".to_owned(),
            arguments,
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
        assert_eq!(Block::new("foo", &[], None).to_string(), "  @@foo");

        assert_eq!(
            Block::new("foo", &[], Some("bar")).to_string(),
            "  @@bar.foo"
        );

        assert_eq!(
            Block {
                group: None,
                name: "foo".to_owned(),
                arguments: vec![
                    Argument::keyword("foo", "bar"),
                    Argument::unnamed(&Value::keyword("baz")),
                    Argument::unnamed(&Value::function("qux", &[])),
                ],
            }
            .to_string(),
            "  @@foo(foo: bar, baz, qux())"
        );
    }

    #[test]
    fn test_display_field() {
        assert_eq!(Field::new("foo", &[], None).to_string(), "@foo");
        assert_eq!(Field::new("foo", &[], Some("bar")).to_string(), "@bar.foo");

        assert_eq!(
            Field {
                group: None,
                name: "foo".to_owned(),
                arguments: vec![
                    Argument::keyword("foo", "bar"),
                    Argument::unnamed(&Value::keyword("baz")),
                    Argument::unnamed(&Value::function("qux", &[])),
                ],
            }
            .to_string(),
            "@foo(foo: bar, baz, qux())"
        );
    }
}
