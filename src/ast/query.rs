use crate::parser::{
    brace_close, brace_open, chars_if, choice, colon, comma, dollar, literal, many1, maybe,
    paren_close, paren_open, spaces, ParseResult,
};

use super::r#type::Type;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Selector {
    Contains(String),
    Equals(String),
}

impl Selector {
    fn parse_contains(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "contains")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (value, input) = Query::parse_variable(&input)?;

        Ok((Self::Contains(value), input))
    }

    fn parse_equals(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "equals")?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (value, input) = Query::parse_variable(&input)?;

        Ok((Self::Equals(value), input))
    }

    /// Parse a selector from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid selector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::query::{Selector, Where};
    ///
    /// let input = "contains: $foo";
    ///
    /// assert_eq!(
    ///     Selector::parse(input),
    ///     Ok((
    ///         Selector::Contains("foo".to_string()),
    ///         "".to_string(),
    ///     ))
    /// );
    ///
    /// let input = "equals: $bar";
    ///
    /// assert_eq!(
    ///     Selector::parse(input),
    ///     Ok((
    ///         Selector::Equals("bar".to_string()),
    ///         "".to_string(),
    ///     ))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(input, vec![Self::parse_contains, Self::parse_equals])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Where {
    Selector(Selector),
    Node(String, Vec<Where>),
}

impl Where {
    /// Parse a selector from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid selector.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::query::{Selector, Where};
    ///
    /// let input = "contains: $foo";
    ///
    /// assert_eq!(
    ///     Where::parse_selector(input),
    ///     Ok((
    ///         Where::Selector(Selector::Contains("foo".to_string())),
    ///         "".to_string(),
    ///     ))
    /// );
    ///
    /// let input = "equals: $bar";
    ///
    /// assert_eq!(
    ///     Where::parse_selector(input),
    ///     Ok((
    ///         Where::Selector(Selector::Equals("bar".to_string())),
    ///         "".to_string(),
    ///     ))
    /// );
    /// ```
    pub fn parse_selector(input: &str) -> ParseResult<Self> {
        let (selector, input) = Selector::parse(input)?;

        Ok((Self::Selector(selector), input))
    }

    /// Parse a node from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::query::{Selector, Where};
    ///
    /// let input = "foo {
    ///     contains: $foo
    /// }";
    ///
    /// assert_eq!(
    ///     Where::parse_node(input),
    ///     Ok((
    ///         Where::Node(
    ///             "foo".to_string(),
    ///             vec![
    ///                 Where::Selector(
    ///                     Selector::Contains("foo".to_string())
    ///                 )
    ///             ]
    ///         ),
    ///         "".to_string()
    ///     ))
    /// );
    ///
    /// let input = "foo {
    ///     bar {
    ///         contains: $foo
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     Where::parse_node(input),
    ///     Ok((
    ///         Where::Node(
    ///             "foo".to_string(),
    ///             vec![
    ///                 Where::Node(
    ///                     "bar".to_string(),
    ///                     vec![
    ///                         Where::Selector(
    ///                             Selector::Contains("foo".to_string())
    ///                         )
    ///                     ]
    ///                 )
    ///             ]
    ///         ),
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    pub fn parse_node(input: &str) -> ParseResult<Self> {
        let (name, input) = chars_if(input, |c| c.is_ascii_alphabetic())?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (structure, input) = many1(&input, |input| {
            let (_, input) = spaces(input)?;
            let (where_clause, input) =
                choice(&input, vec![Self::parse_selector, Self::parse_node])?;
            let (_, input) = spaces(&input)?;

            Ok((where_clause, input))
        })?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self::Node(name, structure), input))
    }

    /// Parse a where clause from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid where clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::query::{Selector, Where};
    ///
    /// let input = "where {
    ///     foo {
    ///         contains: $foo
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     Where::parse(input),
    ///     Ok((
    ///         Where::Node(
    ///             "foo".to_string(),
    ///             vec![
    ///                 Where::Selector(
    ///                     Selector::Contains("foo".to_string())
    ///                 )
    ///             ]
    ///         ),
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "where")?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (where_clause, input) = choice(&input, vec![Self::parse_selector, Self::parse_node])?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((where_clause, input))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Argument {
    pub name: String,
    pub r#type: Type,
}

impl Argument {
    /// Parse an argument from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::query::Argument;
    /// use dragonfly::ast::r#type::{Primitive, Type};
    ///
    /// assert_eq!(
    ///     Argument::parse("$name: String"),
    ///     Ok((
    ///         Argument {
    ///             name: "name".to_string(),
    ///             r#type: Type::One(Primitive::String),
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = Query::parse_variable(input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((Self { name, r#type }, input))
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Schema {
    Identifier(String),
    Node(String, Vec<Schema>),
}

impl Schema {
    fn parse_node(input: &str) -> ParseResult<Self> {
        let (name, input) = chars_if(input, |c| c.is_ascii_alphabetic())?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (structure, input) = many1(&input, |input| {
            let (_, input) = spaces(input)?;
            let (schema, input) = Self::parse(&input)?;
            let (_, input) = spaces(&input)?;

            Ok((schema, input))
        })?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((Self::Node(name, structure), input))
    }

    fn parse_identifier(input: &str) -> ParseResult<Self> {
        let (identifier, input) = chars_if(input, |c| c.is_ascii_alphabetic())?;

        Ok((Self::Identifier(identifier), input))
    }

    /// Parse a schema from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid schema.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::query::Schema;
    ///
    /// assert_eq!(
    ///     Schema::parse("user"),
    ///     Ok((Schema::Identifier("user".to_string()), "".to_string())),
    /// );
    ///
    /// let input = "user {
    ///   name
    /// }";
    ///
    /// assert_eq!(
    ///     Schema::parse(input),
    ///     Ok((
    ///         Schema::Node(
    ///             "user".to_string(),
    ///             vec![Schema::Identifier("name".to_string())],
    ///         ),
    ///         "".to_string()
    ///     )),
    /// );
    ///
    /// let input = "user {
    ///   name {
    ///     first
    ///     last
    ///   }
    /// }";
    ///
    /// assert_eq!(
    ///     Schema::parse(input),
    ///     Ok((
    ///         Schema::Node(
    ///             "user".to_string(),
    ///             vec![
    ///                 Schema::Node(
    ///                     "name".to_string(),
    ///                     vec![
    ///                         Schema::Identifier("first".to_string()),
    ///                         Schema::Identifier("last".to_string()),
    ///                     ]
    ///                 )
    ///             ]
    ///         ),
    ///         "".to_string()
    ///     )),
    /// );
    ///
    /// let input = "user";
    ///
    /// assert_eq!(
    ///    Schema::parse(input),
    ///    Ok((Schema::Identifier("user".to_string()), "".to_string())),
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice(input, vec![Self::parse_node, Self::parse_identifier])
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Query {
    pub name: String,
    pub arguments: Vec<Argument>,
    pub schema: Schema,
    pub r#type: Type,
    pub r#where: Option<Where>,
}

impl Query {
    /// Parse query arguments from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid query argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::{Primitive, Type};
    /// use dragonfly::ast::query::{Argument, Query};
    ///
    /// assert_eq!(
    ///     Query::parse_arguments("($id: UUID)"),
    ///     Ok((
    ///         vec![Argument {
    ///             name: "id".to_string(),
    ///             r#type: Type::One(Primitive::Identifier("UUID".to_string()))
    ///         }],
    ///         "".to_string()
    ///    ))
    /// );
    ///
    /// assert_eq!(
    ///     Query::parse_arguments("($id: UUID, $name: [String])"),
    ///     Ok((
    ///         vec![
    ///             Argument {
    ///                 name: "id".to_string(),
    ///                 r#type: Type::One(Primitive::Identifier("UUID".to_string()))
    ///             },
    ///             Argument {
    ///                 name: "name".to_string(),
    ///                 r#type: Type::Array(Primitive::String)
    ///             }
    ///         ],
    ///         "".to_string()
    ///    ))
    /// );
    /// ```
    pub fn parse_arguments(input: &str) -> ParseResult<Vec<Argument>> {
        if let Ok((_, input)) = paren_open(input) {
            let (argument, mut input) = Argument::parse(&input)?;
            let mut arguments = vec![argument];

            while let Ok((_, new_input)) = comma(&input) {
                let (_, new_input) = spaces(&new_input)?;
                let (argument, new_input) = Argument::parse(&new_input)?;

                arguments.push(argument);
                input = new_input;
            }

            let (_, input) = paren_close(&input)?;

            return Ok((arguments, input));
        }

        Ok((vec![], input.to_string()))
    }

    /// Parse a variable from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid variable.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::query::Query;
    ///
    /// assert_eq!(
    ///    Query::parse_variable("$name"),
    ///    Ok(("name".to_string(), "".to_string()))
    /// );
    ///
    /// assert!(Query::parse_variable("name").is_err());
    /// ```
    pub fn parse_variable(input: &str) -> ParseResult<String> {
        let (_, input) = dollar(input)?;

        chars_if(&input, |c| c.is_ascii_alphabetic())
    }

    /// Parse a query from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// * If the input is not a valid query.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::r#type::{Primitive, Type};
    /// use dragonfly::ast::query::{Argument, Query, Schema, Selector, Where};
    ///
    /// let input = "query images: [Image] {
    ///   image {
    ///     title
    ///   }
    /// }";
    ///
    /// let expected = Query {
    ///     name: "images".to_string(),
    ///     arguments: vec![],
    ///     schema: Schema::Node(
    ///         "image".to_string(),
    ///         vec![
    ///             Schema::Identifier("title".to_string()),
    ///         ]
    ///     ),
    ///     r#type: Type::Array(Primitive::Identifier("Image".to_string())),
    ///     r#where: None,
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, "".to_string())));
    ///
    /// let input = "query images($tag: String, $title: String): [Image] {
    ///   image {
    ///     title
    ///   }
    ///   where {
    ///     image {
    ///       title {
    ///         equals: $title
    ///         tags {
    ///           contains: $tag
    ///         }
    ///       }
    ///     }
    ///   }
    /// }";
    ///
    /// let expected = Query {
    ///     name: "images".to_string(),
    ///     arguments: vec![
    ///         Argument {
    ///             name: "tag".to_string(),
    ///             r#type: Type::One(Primitive::String),
    ///         },   
    ///         Argument {
    ///             name: "title".to_string(),
    ///             r#type: Type::One(Primitive::String),
    ///         },
    ///     ],
    ///     schema: Schema::Node(
    ///         "image".to_string(),
    ///         vec![
    ///             Schema::Identifier("title".to_string()),
    ///         ]
    ///     ),
    ///     r#type: Type::Array(Primitive::Identifier("Image".to_string())),
    ///     r#where: Some(Where::Node(
    ///         "image".to_string(),
    ///         vec![
    ///             Where::Node(
    ///                 "title".to_string(),
    ///                 vec![
    ///                     Where::Selector(Selector::Equals("title".to_string())),
    ///                     Where::Node(
    ///                         "tags".to_string(),
    ///                         vec![
    ///                             Where::Selector(Selector::Contains("tag".to_string())),
    ///                         ]
    ///                     ),
    ///                 ],
    ///             ),
    ///         ],
    ///     )),
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, "".to_string())));
    ///
    /// let input = "query imagesByCountryName($name: CountryName): [Image] {
    ///   image {
    ///     title
    ///     category
    ///   }
    ///   where {
    ///     image {
    ///       country {
    ///         name {
    ///           equals: $name
    ///         }
    ///       }
    ///     }
    ///   }
    /// }";
    ///
    /// let expected = Query {
    ///     name: "imagesByCountryName".to_string(),
    ///     arguments: vec![
    ///         Argument {
    ///             name: "name".to_string(),
    ///             r#type: Type::One(Primitive::Identifier("CountryName".to_string())),
    ///         },
    ///     ],
    ///     schema: Schema::Node(
    ///         "image".to_string(),
    ///         vec![
    ///             Schema::Identifier("title".to_string()),
    ///             Schema::Identifier("category".to_string()),
    ///         ],
    ///     ),
    ///     r#type: Type::Array(Primitive::Identifier("Image".to_string())),
    ///     r#where: Some(Where::Node(
    ///         "image".to_string(),
    ///         vec![Where::Node(
    ///             "country".to_string(),
    ///             vec![Where::Node(
    ///                 "name".to_string(),
    ///                 vec![Where::Selector(Selector::Equals("name".to_string()))],
    ///             )],
    ///         )],
    ///     )),
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, "".to_string())));
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "query")?;
        let (_, input) = spaces(&input)?;
        let (name, input) = chars_if(&input, |c| c.is_ascii_alphabetic())?;
        let (_, input) = spaces(&input)?;
        let (arguments, input) = Self::parse_arguments(&input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (schema, input) = Schema::parse(&input)?;
        let (r#where, input) = maybe(&input, |input| {
            let (_, input) = spaces(input)?;
            Where::parse(&input)
        })?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((
            Self {
                name,
                arguments,
                schema,
                r#type,
                r#where,
            },
            input,
        ))
    }
}
