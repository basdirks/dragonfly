use {
    crate::{
        ast::r#type::Type,
        parser::{
            camel_case,
            colon,
            spaces,
            ParseResult,
        },
    },
    std::fmt::Display,
};

/// A field belonging to a model.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Field {
    /// The name of the field. Used inside query schemas and where clauses.
    pub name: String,
    /// The type of the field.
    pub r#type: Type,
}

impl Field {
    /// Create a new field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `r#type` - The type of the field.
    #[must_use]
    pub fn new(
        name: &str,
        r#type: Type,
    ) -> Self {
        Self {
            name: name.to_owned(),
            r#type,
        }
    }

    /// Create a new boolean field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn boolean(name: &str) -> Self {
        Self::new(name, Type::boolean())
    }

    /// Create a new date and time field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn date_time(name: &str) -> Self {
        Self::new(name, Type::date_time())
    }

    /// Create a new float field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn float(name: &str) -> Self {
        Self::new(name, Type::float())
    }

    /// Create a new integer field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn int(name: &str) -> Self {
        Self::new(name, Type::int())
    }

    /// Create a new string field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self::new(name, Type::string())
    }

    /// Create a new reference field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `reference` - The name of the reference.
    #[must_use]
    pub fn reference(
        name: &str,
        reference: &str,
    ) -> Self {
        Self::new(name, Type::reference(reference))
    }

    /// Create a new owned reference field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `reference` - The name of the reference.
    #[must_use]
    pub fn owned_reference(
        name: &str,
        reference: &str,
    ) -> Self {
        Self::new(name, Type::owned_reference(reference))
    }

    /// Create a new boolean array field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn booleans(name: &str) -> Self {
        Self::new(name, Type::booleans())
    }

    /// Create a new date and time array field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn date_times(name: &str) -> Self {
        Self::new(name, Type::date_times())
    }

    /// Create a new float array field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn floats(name: &str) -> Self {
        Self::new(name, Type::floats())
    }

    /// Create a new integer array field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn ints(name: &str) -> Self {
        Self::new(name, Type::ints())
    }

    /// Create a new string array field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn strings(name: &str) -> Self {
        Self::new(name, Type::strings())
    }

    /// Create a new reference array field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `reference` - The name of the reference.
    #[must_use]
    pub fn references(
        name: &str,
        reference: &str,
    ) -> Self {
        Self::new(name, Type::references(reference))
    }

    /// Create a new owned reference array field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `reference` - The name of the reference.
    #[must_use]
    pub fn owned_references(
        name: &str,
        reference: &str,
    ) -> Self {
        Self::new(name, Type::owned_references(reference))
    }

    /// Parse a field from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid field.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Field,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// let input = "bar: String";
    ///
    /// let expected = Field {
    ///     name: "bar".to_owned(),
    ///     r#type: Type::string(),
    /// };
    ///
    /// assert_eq!(Field::parse(input), Ok((expected, String::new())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::{
    ///         Field,
    ///         Scalar,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "Baz: Int";
    ///
    /// assert_eq!(
    ///     Field::parse(input),
    ///     Err(ParseError::UnexpectedChar {
    ///         message: "Expected camelCase identifier to start with lowercase \
    ///                   character, found 'B'."
    ///             .to_string(),
    ///         actual: 'B'
    ///     })
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::{
    ///         Field,
    ///         Scalar,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// let input = "baz= Int";
    ///
    /// assert_eq!(
    ///     Field::parse(input),
    ///     Err(ParseError::UnexpectedChar {
    ///         message: "Expected character ':', found '='.".to_owned(),
    ///         actual: '=',
    ///     })
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::{
    ///         Field,
    ///         Scalar,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(
    ///     Field::parse("foo: @Bar"),
    ///     Ok((
    ///         Field {
    ///             name: "foo".to_owned(),
    ///             r#type: Type::owned_reference("Bar"),
    ///         },
    ///         String::new(),
    ///     ))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = camel_case(input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((Self { name, r#type }, input))
    }
}

impl Display for Field {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self { name, r#type } = self;

        write!(f, "{name}: {type}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field() {
        assert_eq!(
            Field::new("foo", Type::string()),
            Field {
                name: "foo".to_owned(),
                r#type: Type::string(),
            }
        );
    }

    #[test]
    fn test_boolean() {
        assert_eq!(
            Field::boolean("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::boolean(),
            }
        );
    }

    #[test]
    fn test_date_time() {
        assert_eq!(
            Field::date_time("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::date_time(),
            }
        );
    }

    #[test]
    fn test_float() {
        assert_eq!(
            Field::float("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::float(),
            }
        );
    }

    #[test]
    fn test_int() {
        assert_eq!(
            Field::int("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::int(),
            }
        );
    }

    #[test]
    fn test_string() {
        assert_eq!(
            Field::string("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::string(),
            }
        );
    }

    #[test]
    fn test_reference() {
        assert_eq!(
            Field::reference("foo", "Bar"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::reference("Bar"),
            }
        );
    }

    #[test]
    fn test_owned_reference() {
        assert_eq!(
            Field::owned_reference("foo", "Bar"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::owned_reference("Bar"),
            }
        );
    }

    #[test]
    fn test_booleans() {
        assert_eq!(
            Field::booleans("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::booleans(),
            }
        );
    }

    #[test]
    fn test_date_times() {
        assert_eq!(
            Field::date_times("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::date_times(),
            }
        );
    }

    #[test]
    fn test_floats() {
        assert_eq!(
            Field::floats("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::floats(),
            }
        );
    }

    #[test]
    fn test_ints() {
        assert_eq!(
            Field::ints("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::ints(),
            }
        );
    }

    #[test]
    fn test_strings() {
        assert_eq!(
            Field::strings("foo"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::strings(),
            }
        );
    }

    #[test]
    fn test_references() {
        assert_eq!(
            Field::references("foo", "Bar"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::references("Bar"),
            }
        );
    }

    #[test]
    fn test_owned_references() {
        assert_eq!(
            Field::owned_references("foo", "Bar"),
            Field {
                name: "foo".to_owned(),
                r#type: Type::owned_references("Bar"),
            }
        );
    }

    #[test]
    fn test_parse() {
        let input = "foo: String";

        let expected = Field {
            name: "foo".to_owned(),
            r#type: Type::string(),
        };

        assert_eq!(Field::parse(input), Ok((expected, String::new())));
    }

    #[test]
    fn test_display() {
        let field = Field {
            name: "foo".to_owned(),
            r#type: Type::string(),
        };

        assert_eq!(field.to_string(), "foo: String");
    }
}
