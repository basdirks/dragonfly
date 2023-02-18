use {
    super::{
        ConstDirective,
        ConstValue,
        Directive,
        Selection,
        Type,
    },
    crate::{
        ast::QueryArgument as AstQueryArgument,
        generator::printer::{
            comma_separated,
            space_separated,
            Print,
        },
    },
    std::fmt::Display,
};

/// A variable definition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Variable {
    /// The name of the variable.
    pub name: String,
    /// The type of the variable.
    pub r#type: Type,
    /// The default value of the variable.
    pub default_value: Option<ConstValue>,
    /// The directives of the variable.
    pub directives: Vec<ConstDirective>,
}

impl From<AstQueryArgument> for Variable {
    fn from(AstQueryArgument { name, r#type }: AstQueryArgument) -> Self {
        Self {
            name,
            r#type: r#type.into(),
            default_value: None,
            directives: vec![],
        }
    }
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
#[derive(Clone, Debug, Eq, PartialEq)]
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
            query.push_str(&format!("({})", comma_separated(&self.variables)));
        }

        if !self.directives.is_empty() {
            query.push_str(&format!(" {}", space_separated(&self.directives)));
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
        crate::{
            ast::{
                QueryArgument as AstQueryArgument,
                Scalar as AstScalar,
                Type as AstType,
            },
            generator::graphql::{
                Argument,
                Field,
                Value,
            },
        },
    };

    #[test]
    fn test_variable_from_ast() {
        assert_eq!(
            Variable::from(AstQueryArgument {
                name: "foo".to_string(),
                r#type: AstType::Array(AstScalar::String),
            }),
            Variable {
                name: "foo".to_string(),
                r#type: Type::NonNull(Box::new(Type::List(Box::new(
                    Type::Name("String".to_string())
                )))),
                default_value: None,
                directives: vec![],
            }
        );
    }

    #[test]
    fn test_display_variable() {
        assert_eq!(
            Variable {
                name: "foo".to_string(),
                r#type: Type::Name("String".to_string()),
                default_value: None,
                directives: vec![],
            }
            .to_string(),
            "$foo: String"
        );

        assert_eq!(
            Variable {
                name: "foo".to_string(),
                r#type: Type::Name("String".to_string()),
                default_value: Some(ConstValue::String("bar".to_string())),
                directives: vec![],
            }
            .to_string(),
            "$foo: String = \"bar\""
        );

        assert_eq!(
            Variable {
                name: "foo".to_string(),
                r#type: Type::NonNull(Box::new(Type::List(Box::new(
                    Type::NonNull(Box::new(Type::Name("String".to_string())))
                )))),
                default_value: None,
                directives: vec![
                    ConstDirective {
                        name: "bar".to_string(),
                        arguments: vec![],
                    },
                    ConstDirective {
                        name: "baz".to_string(),
                        arguments: vec![],
                    },
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
                name: "imagesByCountryName".to_string(),
                directives: vec![
                    Directive {
                        name: "bar".to_string(),
                        arguments: vec![],
                    },
                    Directive {
                        name: "baz".to_string(),
                        arguments: vec![],
                    },
                ],
                selections: vec![Selection::Field(Field {
                    name: "images".to_string(),
                    arguments: vec![Argument {
                        name: "country".to_string(),
                        value: Value::Variable("country".to_string()),
                    }],
                    directives: vec![],
                    selections: vec![
                        Selection::Field(Field {
                            name: "url".to_string(),
                            arguments: vec![],
                            directives: vec![Directive {
                                name: "deprecated".to_string(),
                                arguments: vec![Argument {
                                    name: "reason".to_string(),
                                    value: Value::String(
                                        "Use `link` instead.".to_string()
                                    ),
                                }],
                            }],
                            selections: vec![],
                        }),
                        Selection::Field(Field {
                            name: "link".to_string(),
                            arguments: vec![],
                            directives: vec![],
                            selections: vec![],
                        }),
                        Selection::Field(Field {
                            name: "title".to_string(),
                            arguments: vec![],
                            directives: vec![],
                            selections: vec![],
                        }),
                    ],
                })],
                variables: vec![
                    Variable {
                        name: "country".to_string(),
                        r#type: Type::NonNull(Box::new(Type::Name(
                            "String".to_string()
                        ))),
                        default_value: None,
                        directives: vec![],
                    },
                    Variable {
                        name: "limit".to_string(),
                        r#type: Type::Name("Int".to_string()),
                        default_value: Some(ConstValue::Int("10".to_string())),
                        directives: vec![],
                    },
                ],
            }
            .print(0),
            "\
query imagesByCountryName($country: String!, $limit: Int = 10) @bar @baz {
  images(country: $country) {
    url @deprecated(reason: \"Use `link` instead.\")
    link
    title
  }
}"
        );
    }
}