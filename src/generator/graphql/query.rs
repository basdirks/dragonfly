use {
    super::{
        Const,
        ConstDirective,
        Directive,
        Selection,
        Type,
    },
    crate::generator::printer::{
        comma_separated,
        space_separated,
        Print,
    },
    std::fmt::{
        Display,
        Write,
    },
};

/// A variable definition.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Variable {
    /// The name of the variable.
    pub name: String,
    /// The type of the variable.
    pub r#type: Type,
    /// The default value of the variable.
    pub default_value: Option<Const>,
    /// The directives of the variable.
    pub directives: Vec<ConstDirective>,
}

impl Display for Variable {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        write!(f, "${}: {}", self.name, self.r#type)?;

        if let Some(default_value) = &self.default_value {
            write!(f, " = {default_value}")?;
        }

        if !self.directives.is_empty() {
            write!(f, " {}", space_separated(&self.directives))?;
        }

        Ok(())
    }
}

/// A GraphQL query.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Query {
    /// The directives of the query.
    pub directives: Vec<Directive>,
    /// The name of the query.
    pub name: String,
    /// The selection set of the query.
    pub selections: Vec<Selection>,
    /// The variables of the query.
    pub variables: Vec<Variable>,
}

impl Print for Query {
    fn print(
        &self,
        level: usize,
    ) -> String {
        let mut query = format!("query {}", self.name);

        if !self.variables.is_empty() {
            let _ = write!(query, "({})", comma_separated(&self.variables));
        }

        if !self.directives.is_empty() {
            let _ = write!(query, " {}", space_separated(&self.directives));
        }

        if !self.selections.is_empty() {
            query.push_str(&Selection::print_multiple(&self.selections, level));
        }

        query
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::graphql::{
            Argument,
            Field,
        },
    };

    #[test]
    fn test_display_variable() {
        assert_eq!(
            Variable {
                name: "foo".to_owned(),
                r#type: Type::name("String"),
                default_value: None,
                directives: vec![],
            }
            .to_string(),
            "$foo: String"
        );

        assert_eq!(
            Variable {
                name: "foo".to_owned(),
                r#type: Type::name("String"),
                default_value: Some(Const::string("bar")),
                directives: vec![],
            }
            .to_string(),
            "$foo: String = \"bar\""
        );

        assert_eq!(
            Variable {
                name: "foo".to_owned(),
                r#type: Type::non_null(Type::list(Type::non_null(Type::name(
                    "String"
                )))),
                default_value: None,
                directives: vec![
                    ConstDirective::new("bar", &[]),
                    ConstDirective::new("baz", &[]),
                ],
            }
            .to_string(),
            "$foo: [String!]! @bar @baz"
        );
    }

    #[test]
    fn test_print_query() {
        assert_eq!(
            Query {
                name: "imagesByCountryName".to_owned(),
                directives: vec![
                    Directive::new("bar", &[]),
                    Directive::new("baz", &[]),
                ],
                selections: vec![Selection::Field(Field {
                    name: "images".to_owned(),
                    arguments: vec![Argument::variable("country", "country")],
                    directives: vec![],
                    selections: vec![
                        Selection::Field(Field {
                            name: "url".to_owned(),
                            arguments: vec![],
                            directives: vec![Directive::new(
                                "deprecated",
                                &[Argument::string(
                                    "reason",
                                    "Use `link` instead."
                                )],
                            )],
                            selections: vec![],
                        }),
                        Selection::Field(Field::new("link")),
                        Selection::Field(Field::new("title")),
                    ],
                })],
                variables: vec![
                    Variable {
                        name: "country".to_owned(),
                        r#type: Type::NonNull(Box::new(Type::Name(
                            "String".to_owned()
                        ))),
                        default_value: None,
                        directives: vec![],
                    },
                    Variable {
                        name: "limit".to_owned(),
                        r#type: Type::name("Int"),
                        default_value: Some(Const::int("10")),
                        directives: vec![],
                    },
                ],
            }
            .print(0),
            "

query imagesByCountryName($country: String!, $limit: Int = 10) @bar @baz {
  images(country: $country) {
    url @deprecated(reason: \"Use `link` instead.\")
    link
    title
  }
}

"
            .trim()
        );
    }
}
