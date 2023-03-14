use {
    super::r#type::Type,
    ord_str_map::OrdStrMap,
    parser::{
        alphabetics,
        brace_close,
        brace_open,
        colon,
        comma,
        dollar,
        literal,
        option,
        paren_close,
        paren_open,
        spaces,
        ParseError,
        ParseResult,
    },
    std::borrow::Cow,
};
pub use {
    argument::Argument,
    r#where::{
        Condition,
        Operator,
        Path,
        Where,
    },
    return_type::ReturnType,
    schema::{
        Node as SchemaNode,
        Schema,
    },
};

/// Query arguments.
pub mod argument;
/// The return type of a query.
pub mod return_type;
/// The structure of the data that the query should return.
pub mod schema;
/// Sets of conditions that queried data must meet.
pub mod r#where;

/// A query.
#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Query<'a> {
    /// The name of the query. Used as the name of the generated function.
    pub name: Cow<'a, str>,
    /// The arguments of the query.
    pub arguments: OrdStrMap<Argument<'a>>,
    /// The schema of the query.
    pub schema: Schema<'a>,
    /// The return type of the query. Must be a model reference or an array of
    /// a model reference.
    pub r#type: ReturnType<'a>,
    /// The where clause of the query.
    pub r#where: Option<Where<'a>>,
}

impl<'a> Query<'a> {
    /// Parse query arguments from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid query
    /// argument.
    fn parse_arguments(input: &str) -> ParseResult<OrdStrMap<Argument<'a>>> {
        if let Ok((_, input)) = paren_open(input) {
            let mut arguments = OrdStrMap::new();
            let (argument, mut input) = Argument::parse(&input)?;

            let _: Option<Argument> =
                arguments.insert(argument.name.clone(), argument.clone());

            while let Ok((_, new_input)) = comma(&input) {
                let (_, new_input) = spaces(&new_input)?;
                let (argument, new_input) = Argument::parse(&new_input)?;

                if arguments
                    .insert(argument.name.clone(), argument.clone())
                    .is_some()
                {
                    return Err(ParseError::custom(format!(
                        "duplicate argument `{}`.",
                        argument.name
                    )));
                }

                input = new_input;
            }

            let (_, input) = paren_close(&input)?;

            return Ok((arguments, input));
        }

        Ok((OrdStrMap::new(), input.to_owned()))
    }

    /// Parse a reference from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid
    /// reference.
    fn parse_reference(input: &str) -> ParseResult<String> {
        let (_, input) = dollar(input)?;

        alphabetics(&input)
    }

    /// Parse a query from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid query.
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "query")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = alphabetics(&input)?;
        let (_, input) = spaces(&input)?;
        let (arguments, input) = Self::parse_arguments(&input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = ReturnType::parse(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (schema, input) = Schema::parse(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#where, input) = option(&input, Where::parse)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((
            Self {
                name: name.into(),
                arguments,
                schema,
                r#type,
                r#where,
            },
            input,
        ))
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::{
            QueryArgument,
            QueryCondition,
            QueryOperator,
            QueryPath,
            QueryReturnType,
            QuerySchema,
            QuerySchemaNode,
            QueryWhere,
            Scalar,
        },
        ord_str_map::OrdStrMap,
    };

    #[test]
    fn test_parse_basic() {
        let input = "
        
        query images: [Image] {
          image {
            title
          }
        }
        
        "
        .trim();

        let expected = Query {
            name: "images".into(),
            arguments: OrdStrMap::new(),
            schema: QuerySchema {
                name: "image".into(),
                nodes: vec![QuerySchemaNode::Field {
                    name: "title".into(),
                }],
            },
            r#type: QueryReturnType::Array("Image".into()),
            r#where: None,
        };

        assert_eq!(Query::parse(input), Ok((expected, String::new())));
    }

    #[test]
    fn test_parse_with_arguments() {
        let input = "
        
        query images($tag: String, $title: String): [Image] {
          image {
            title
          }
          where {
            image {
              title {
                equals: $title
                tags {
                  contains: $tag
                }
              }
            }
          }
        }
        
        "
        .trim();

        let expected = Query {
            name: "images".into(),
            arguments: {
                let mut arguments = OrdStrMap::new();

                let _: Option<Argument> = arguments.insert(
                    "tag",
                    QueryArgument {
                        name: "tag".into(),
                        r#type: Type::Scalar(Scalar::String),
                    },
                );

                let _: Option<Argument> = arguments.insert(
                    "title",
                    QueryArgument {
                        name: "title".into(),
                        r#type: Type::Scalar(Scalar::String),
                    },
                );

                arguments
            },
            schema: QuerySchema {
                name: "image".into(),
                nodes: vec![QuerySchemaNode::Field {
                    name: "title".into(),
                }],
            },
            r#type: QueryReturnType::Array("Image".into()),
            r#where: Some(QueryWhere {
                name: "image".into(),
                conditions: vec![
                    QueryCondition {
                        path: QueryPath::from_iter(["title"]),
                        operator: QueryOperator::Equals,
                        argument_name: "title".into(),
                    },
                    QueryCondition {
                        path: QueryPath::from_iter(["title", "tags"]),
                        operator: QueryOperator::Contains,
                        argument_name: "tag".into(),
                    },
                ],
            }),
        };

        assert_eq!(Query::parse(input), Ok((expected, String::new())));
    }

    #[test]
    fn test_duplicate_argument() {
        let input = "
        
        query images($tag: String, $tag: String): [Image] {
          image {
            title
          }
        }
        
        "
        .trim();

        assert_eq!(
            Query::parse(input),
            Err(ParseError::custom("duplicate argument `tag`."))
        );
    }
}
