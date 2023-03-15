pub use variable::Variable;
use {
    super::{
        Const,
        ConstDirective,
        Directive,
        Selection,
        Type,
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

/// Query variables.
pub mod variable;

/// A GraphQL query.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Query<'a> {
    /// The name of the query.
    pub name: Cow<'a, str>,
    /// The directives of the query.
    pub directives: Vec<Directive<'a>>,
    /// The selection set of the query.
    pub selections: Vec<Selection<'a>>,
    /// The variables of the query.
    pub variables: Vec<Variable<'a>>,
}

impl Print for Query<'_> {
    const TAB_SIZE: usize = crate::TAB_SIZE;

    fn print(
        &self,
        level: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "query {}", self.name)?;

        if !self.variables.is_empty() {
            write!(f, "(")?;
            PrintInline::intercalate(self.variables.clone(), f, ", ")?;
            write!(f, ")")?;
        }

        for directive in &self.directives {
            directive.print(f)?;
        }

        if !self.selections.is_empty() {
            Selection::print_multiple(self.selections.iter(), level, f)?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            Argument,
            Field,
            Value,
        },
    };

    #[test]
    fn test_print() {
        let query = Query {
            name: "imagesByCountryName".into(),
            directives: vec![
                Directive {
                    name: "bar".into(),
                    arguments: vec![],
                },
                Directive {
                    name: "baz".into(),
                    arguments: vec![],
                },
            ],
            selections: vec![Selection::Field(Field {
                name: "images".into(),
                arguments: vec![Argument {
                    name: "country".into(),
                    value: Value::Variable("country".into()),
                }],
                directives: vec![],
                selections: vec![
                    Selection::Field(Field {
                        name: "url".into(),
                        arguments: vec![],
                        directives: vec![Directive {
                            name: "deprecated".into(),
                            arguments: vec![Argument {
                                name: "reason".into(),
                                value: Value::String(
                                    "Use `link` instead.".into(),
                                ),
                            }],
                        }],
                        selections: vec![],
                    }),
                    Selection::Field(Field {
                        name: "link".into(),
                        arguments: vec![],
                        directives: vec![],
                        selections: vec![],
                    }),
                    Selection::Field(Field {
                        name: "title".into(),
                        arguments: vec![],
                        directives: vec![],
                        selections: vec![],
                    }),
                ],
            })],
            variables: vec![
                Variable {
                    name: "country".into(),
                    r#type: Type::NonNull(Box::new(Type::Name(
                        "String".into(),
                    ))),
                    default_value: None,
                    directives: vec![],
                },
                Variable {
                    name: "limit".into(),
                    r#type: Type::Name("Int".into()),
                    default_value: Some(Const::Int("10".into())),
                    directives: vec![],
                },
            ],
        };

        let mut f = Vec::new();

        query.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "query imagesByCountryName($country: String!, $limit: Int = 10) \
             @bar @baz {
  images(country: $country) {
    url @deprecated(reason: \"Use `link` instead.\")
    link
    title
  }
}
"
        );
    }
}
