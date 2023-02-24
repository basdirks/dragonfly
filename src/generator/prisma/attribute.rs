use {
    super::{
        Function,
        KeyValuePair,
        Value,
    },
    crate::generator::printer::comma_separated,
    std::fmt::Display,
};

/// A model attribute argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Argument {
    /// A key value pair.
    KeyValuePair(KeyValuePair),
    /// A value.
    Value(Value),
    /// A function.
    Function(Function),
}

impl Display for Argument {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::KeyValuePair(key_value_pair) => {
                write!(f, "{key_value_pair}")
            }
            Self::Value(value) => write!(f, "{value}"),
            Self::Function(function) => write!(f, "{function}"),
        }
    }
}

/// A model attribute.
#[derive(Clone, Debug, Eq, PartialEq)]
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
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Field {
    /// The name of the group to which the attribute belongs.
    pub group: Option<String>,
    /// The name of the attribute.
    pub name: String,
    /// The fields of the attribute.
    pub arguments: Vec<Argument>,
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

impl Field {
    /// Standard `@id` attribute.
    #[must_use]
    pub fn id() -> Self {
        Self {
            group: None,
            name: "id".to_owned(),
            arguments: vec![],
        }
    }

    /// Standard `@default(autoincrement())` attribute.
    #[must_use]
    pub fn default_auto_increment() -> Self {
        Self {
            group: None,
            name: "default".to_owned(),
            arguments: vec![Argument::Function(Function {
                name: "autoincrement".to_owned(),
                parameters: vec![],
            })],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_argument() {
        assert_eq!(
            Argument::KeyValuePair(KeyValuePair {
                key: "foo".to_owned(),
                value: Value::String("bar".to_owned()),
            })
            .to_string(),
            "foo: bar"
        );

        assert_eq!(
            Argument::Value(Value::String("foo".to_owned())).to_string(),
            "foo"
        );

        assert_eq!(
            Argument::Function(Function {
                name: "foo".to_owned(),
                parameters: vec![
                    Value::String("bar".to_owned()),
                    Value::String("baz".to_owned()),
                ],
            })
            .to_string(),
            "foo(bar, baz)"
        );
    }

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
                    Argument::KeyValuePair(KeyValuePair {
                        key: "foo".to_owned(),
                        value: Value::String("bar".to_owned()),
                    }),
                    Argument::Value(Value::String("baz".to_owned())),
                    Argument::Function(Function {
                        name: "qux".to_owned(),
                        parameters: vec![],
                    }),
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
                    Argument::KeyValuePair(KeyValuePair {
                        key: "foo".to_owned(),
                        value: Value::String("bar".to_owned()),
                    }),
                    Argument::Value(Value::String("baz".to_owned())),
                    Argument::Function(Function {
                        name: "qux".to_owned(),
                        parameters: vec![],
                    }),
                ],
            }
            .to_string(),
            "@foo(foo: bar, baz, qux())"
        );
    }
}
