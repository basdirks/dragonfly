use {
    super::Condition,
    crate::parser::{
        alphabetics,
        brace_close,
        brace_open,
        choice,
        literal,
        many1,
        spaces,
        ParseResult,
    },
    std::collections::HashSet,
};

/// The conditions that queried data must meet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Where {
    /// A condition that must be met for a query to return a result.
    Condition(Condition),
    /// A field.
    Node {
        /// The name of the field.
        name: String,
        /// In the case of a relation, the fields and their conditions. This is
        /// empty if the field does not refer to another model.
        nodes: Vec<Where>,
    },
}

impl Where {
    /// Parse a condition from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid
    /// condition.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "contains: $foo";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse_condition(input),
    ///     Ok((
    ///         QueryWhere::Condition(QueryCondition::Contains("foo".to_string())),
    ///         "".to_string(),
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "equals: $bar";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse_condition(input),
    ///     Ok((
    ///         QueryWhere::Condition(QueryCondition::Equals("bar".to_string())),
    ///         "".to_string(),
    ///     ))
    /// );
    /// ```
    pub fn parse_condition(input: &str) -> ParseResult<Self> {
        let (condition, input) = Condition::parse(input)?;

        Ok((Self::Condition(condition), input))
    }

    /// Parse a node from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid node.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "foo {
    ///     contains: $foo
    /// }";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse_node(input),
    ///     Ok((
    ///         QueryWhere::Node {
    ///             name: "foo".to_string(),
    ///             nodes: vec![QueryWhere::Condition(QueryCondition::Contains(
    ///                 "foo".to_string()
    ///             ))]
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "foo {
    ///     bar {
    ///         contains: $foo
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse_node(input),
    ///     Ok((
    ///         QueryWhere::Node {
    ///             name: "foo".to_string(),
    ///             nodes: vec![QueryWhere::Node {
    ///                 name: "bar".to_string(),
    ///                 nodes: vec![QueryWhere::Condition(
    ///                     QueryCondition::Contains("foo".to_string())
    ///                 )]
    ///             }]
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    pub fn parse_node(input: &str) -> ParseResult<Self> {
        let (name, input) = alphabetics(input)?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;

        let (structure, input) = many1(&input, |input| {
            let (_, input) = spaces(input)?;
            let (where_clause, input) =
                choice(&input, vec![Self::parse_condition, Self::parse_node])?;
            let (_, input) = spaces(&input)?;

            Ok((where_clause, input))
        })?;

        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((
            Self::Node {
                name,
                nodes: structure,
            },
            input,
        ))
    }

    /// Parse a where clause from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns a `ParseError` if the input does not start with a valid where
    /// clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let input = "where {
    ///     foo {
    ///         contains: $foo
    ///     }
    /// }";
    ///
    /// assert_eq!(
    ///     QueryWhere::parse(input),
    ///     Ok((
    ///         QueryWhere::Node {
    ///             name: "foo".to_string(),
    ///             nodes: vec![QueryWhere::Condition(QueryCondition::Contains(
    ///                 "foo".to_string()
    ///             ))]
    ///         },
    ///         "".to_string()
    ///     ))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (_, input) = literal(input, "where")?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_open(&input)?;
        let (_, input) = spaces(&input)?;
        let (where_clause, input) =
            choice(&input, vec![Self::parse_condition, Self::parse_node])?;
        let (_, input) = spaces(&input)?;
        let (_, input) = brace_close(&input)?;

        Ok((where_clause, input))
    }

    /// Return all references used in the conditions of this where clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let where_clause = QueryWhere::Node {
    ///     name: "foo".to_string(),
    ///     nodes: vec![
    ///         QueryWhere::Node {
    ///             name: "bar".to_string(),
    ///             nodes: vec![
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "bar".to_string(),
    ///                 )),
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "baz".to_string(),
    ///                 )),
    ///             ],
    ///         },
    ///         QueryWhere::Node {
    ///             name: "baz".to_string(),
    ///             nodes: vec![
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "bar".to_string(),
    ///                 )),
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "foo".to_string(),
    ///                 )),
    ///             ],
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     where_clause.references(),
    ///     vec!["foo", "bar", "baz"]
    ///         .iter()
    ///         .map(ToString::to_string)
    ///         .collect()
    /// );
    /// ```
    pub fn references(&self) -> HashSet<String> {
        let mut references = HashSet::new();

        match self {
            Self::Condition(
                Condition::Contains(reference) | Condition::Equals(reference),
            ) => {
                let _ = references.insert(reference.clone());
            }
            Self::Node { nodes, .. } => {
                references.extend(
                    nodes
                        .iter()
                        .flat_map(Self::references)
                        .collect::<HashSet<String>>(),
                );
            }
        }

        references
    }

    /// Return all condition nodes in this where clause.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryCondition,
    ///     QueryWhere,
    /// };
    ///
    /// let where_clause = QueryWhere::Node {
    ///     name: "foo".to_string(),
    ///     nodes: vec![
    ///         QueryWhere::Node {
    ///             name: "bar".to_string(),
    ///             nodes: vec![
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "bar".to_string(),
    ///                 )),
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "baz".to_string(),
    ///                 )),
    ///             ],
    ///         },
    ///         QueryWhere::Node {
    ///             name: "baz".to_string(),
    ///             nodes: vec![
    ///                 QueryWhere::Condition(QueryCondition::Equals(
    ///                     "bar".to_string(),
    ///                 )),
    ///                 QueryWhere::Condition(QueryCondition::Contains(
    ///                     "foo".to_string(),
    ///                 )),
    ///             ],
    ///         },
    ///     ],
    /// };
    ///
    /// assert_eq!(
    ///     where_clause.conditions(),
    ///     vec![
    ///         QueryCondition::Equals("bar".to_string()),
    ///         QueryCondition::Contains("baz".to_string()),
    ///         QueryCondition::Contains("bar".to_string()),
    ///         QueryCondition::Contains("foo".to_string()),
    ///     ]
    ///     .iter()
    ///     .cloned()
    ///     .collect()
    /// );
    /// ```
    pub fn conditions(&self) -> HashSet<Condition> {
        let mut conditions = HashSet::new();

        match self {
            Self::Condition(condition) => {
                let _ = conditions.insert(condition.clone());
            }
            Self::Node { nodes, .. } => {
                conditions.extend(
                    nodes
                        .iter()
                        .flat_map(Self::conditions)
                        .collect::<HashSet<Condition>>(),
                );
            }
        }

        conditions
    }
}
