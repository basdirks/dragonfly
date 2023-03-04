use {
    super::r#type::Type,
    crate::parser::{
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
        ParseResult,
    },
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
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Query {
    /// The name of the query. Used as the name of the generated function.
    pub name: String,
    /// The arguments of the query.
    pub arguments: Vec<Argument>,
    /// The schema of the query.
    pub schema: Schema,
    /// The return type of the query. Must be a model reference or an array of
    /// a model reference.
    pub r#type: ReturnType,
    /// The where clause of the query.
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
    /// Returns `ParseError` if the input does not start with a valid query
    /// argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Query::parse_arguments("($id: Int)"),
    ///     Ok((vec![QueryArgument::int("id")], String::new()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Query::parse_arguments("($id: Int, $name: [String])"),
    ///     Ok((
    ///         vec![QueryArgument::int("id"), QueryArgument::strings("name")],
    ///         String::new()
    ///     ))
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

        Ok((vec![], input.to_owned()))
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// assert_eq!(
    ///     Query::parse_reference("$name"),
    ///     Ok(("name".to_owned(), String::new()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::Query;
    ///
    /// assert!(Query::parse_reference("name").is_err());
    /// ```
    pub fn parse_reference(input: &str) -> ParseResult<String> {
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryReturnType,
    ///     QuerySchema,
    ///     QuerySchemaNode,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "
    ///
    /// query images: [Image] {
    ///   image {
    ///     title
    ///   }
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Query {
    ///     name: "images".to_owned(),
    ///     arguments: vec![],
    ///     schema: QuerySchema::new("image", &[QuerySchemaNode::field("title")]),
    ///     r#type: QueryReturnType::array("Image"),
    ///     r#where: None,
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, String::new())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryPath,
    ///     QueryReturnType,
    ///     QuerySchema,
    ///     QuerySchemaNode,
    ///     QueryWhere,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "
    ///
    /// query images($tag: String, $title: String): [Image] {
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
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Query {
    ///     name: "images".to_owned(),
    ///     arguments: vec![
    ///         QueryArgument::string("tag"),
    ///         QueryArgument::string("title"),
    ///     ],
    ///     schema: QuerySchema {
    ///         name: "image".to_owned(),
    ///         nodes: vec![QuerySchemaNode::Field("title".to_owned())],
    ///     },
    ///     r#type: QueryReturnType::array("Image"),
    ///     r#where: Some(QueryWhere::new(
    ///         "image",
    ///         &[
    ///             QueryCondition {
    ///                 path: QueryPath::new(&["title"]),
    ///                 operator: QueryOperator::Equals,
    ///                 argument_name: "title".to_owned(),
    ///             },
    ///             QueryCondition {
    ///                 path: QueryPath::new(&["title", "tags"]),
    ///                 operator: QueryOperator::Contains,
    ///                 argument_name: "tag".to_owned(),
    ///             },
    ///         ],
    ///     )),
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, String::new())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Query,
    ///     QueryArgument,
    ///     QueryCondition,
    ///     QueryOperator,
    ///     QueryPath,
    ///     QueryReturnType,
    ///     QuerySchema,
    ///     QuerySchemaNode,
    ///     QueryWhere,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "
    ///
    /// query imagesByCountryName($name: CountryName): [Image] {
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
    /// }
    ///
    /// "
    /// .trim();
    ///
    /// let expected = Query {
    ///     name: "imagesByCountryName".to_owned(),
    ///     arguments: vec![QueryArgument::reference("name", "CountryName")],
    ///     schema: QuerySchema {
    ///         name: "image".to_owned(),
    ///         nodes: vec![
    ///             QuerySchemaNode::field("title"),
    ///             QuerySchemaNode::field("category"),
    ///         ],
    ///     },
    ///     r#type: QueryReturnType::array("Image"),
    ///     r#where: Some(QueryWhere::new(
    ///         "image",
    ///         &[QueryCondition {
    ///             path: QueryPath::new(&["country", "name"]),
    ///             operator: QueryOperator::Equals,
    ///             argument_name: "name".to_owned(),
    ///         }],
    ///     )),
    /// };
    ///
    /// assert_eq!(Query::parse(input), Ok((expected, String::new())));
    /// ```
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
