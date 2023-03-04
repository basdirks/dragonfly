use {
    super::{
        Query,
        Type,
    },
    crate::parser::{
        colon,
        spaces,
        ParseResult,
    },
    std::fmt::Display,
};

/// A query argument.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Argument {
    /// The name of the argument. Used inside the conditions of
    /// the where clause.
    pub name: String,
    /// The type of the argument.
    pub r#type: Type,
}

impl Argument {
    /// Create a new argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `r#type` - The type of the argument.
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

    /// Create a new boolean argument.
    #[must_use]
    pub fn boolean(name: &str) -> Self {
        Self::new(name, Type::boolean())
    }

    /// Create a new date and time argument.
    #[must_use]
    pub fn date_time(name: &str) -> Self {
        Self::new(name, Type::date_time())
    }

    /// Create a new float argument.
    #[must_use]
    pub fn float(name: &str) -> Self {
        Self::new(name, Type::float())
    }

    /// Create a new integer argument.
    #[must_use]
    pub fn int(name: &str) -> Self {
        Self::new(name, Type::int())
    }

    /// Create a new string argument.
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self::new(name, Type::string())
    }

    /// Create a new reference argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `reference` - The name of the reference.
    #[must_use]
    pub fn reference(
        name: &str,
        reference: &str,
    ) -> Self {
        Self::new(name, Type::reference(reference))
    }

    /// Create a new owned reference argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `reference` - The name of the reference.
    #[must_use]
    pub fn owned_reference(
        name: &str,
        reference: &str,
    ) -> Self {
        Self::new(name, Type::owned_reference(reference))
    }

    /// Create a boolean array argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn booleans(name: &str) -> Self {
        Self::new(name, Type::booleans())
    }

    /// Create a date and time array argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn date_times(name: &str) -> Self {
        Self::new(name, Type::date_times())
    }

    /// Create a float array argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn floats(name: &str) -> Self {
        Self::new(name, Type::floats())
    }

    /// Create an integer array argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn ints(name: &str) -> Self {
        Self::new(name, Type::ints())
    }

    /// Create a string array argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    #[must_use]
    pub fn strings(name: &str) -> Self {
        Self::new(name, Type::strings())
    }

    /// Create a reference array argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `reference` - The name of the reference.
    #[must_use]
    pub fn references(
        name: &str,
        reference: &str,
    ) -> Self {
        Self::new(name, Type::references(reference))
    }

    /// Create an owned reference array argument.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the argument.
    /// * `reference` - The name of the reference.
    #[must_use]
    pub fn owned_references(
        name: &str,
        reference: &str,
    ) -> Self {
        Self::new(name, Type::owned_references(reference))
    }
}

impl Display for Argument {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self { name, r#type } = self;

        write!(f, "${name}: {type}")
    }
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
    /// Returns `ParseError` if the input does not start with a valid
    /// argument.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     QueryArgument,
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     QueryArgument::parse("$name: String"),
    ///     Ok((QueryArgument::string("name"), String::new()))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        let (name, input) = Query::parse_reference(input)?;
        let (_, input) = colon(&input)?;
        let (_, input) = spaces(&input)?;
        let (r#type, input) = Type::parse(&input)?;

        Ok((Self { name, r#type }, input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        assert_eq!(
            Argument::new("name", Type::boolean()),
            Argument {
                name: "name".to_owned(),
                r#type: Type::boolean(),
            }
        );
    }

    #[test]
    fn test_display_boolean() {
        assert_eq!(Argument::boolean("name").to_string(), "$name: Boolean");
    }

    #[test]
    fn test_display_date_time() {
        assert_eq!(Argument::date_time("name").to_string(), "$name: DateTime");
    }

    #[test]
    fn test_display_float() {
        assert_eq!(Argument::float("name").to_string(), "$name: Float");
    }

    #[test]
    fn test_display_int() {
        assert_eq!(Argument::int("name").to_string(), "$name: Int");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Argument::string("name").to_string(), "$name: String");
    }

    #[test]
    fn test_display_reference() {
        assert_eq!(
            Argument::reference("name", "reference").to_string(),
            "$name: reference"
        );
    }

    #[test]
    fn test_display_owned_reference() {
        assert_eq!(
            Argument::owned_reference("name", "reference").to_string(),
            "$name: @reference"
        );
    }

    #[test]
    fn test_display_booleans() {
        assert_eq!(Argument::booleans("name").to_string(), "$name: [Boolean]");
    }

    #[test]
    fn test_display_date_times() {
        assert_eq!(
            Argument::date_times("name").to_string(),
            "$name: [DateTime]"
        );
    }

    #[test]
    fn test_display_floats() {
        assert_eq!(Argument::floats("name").to_string(), "$name: [Float]");
    }

    #[test]
    fn test_display_ints() {
        assert_eq!(Argument::ints("name").to_string(), "$name: [Int]");
    }

    #[test]
    fn test_display_strings() {
        assert_eq!(Argument::strings("name").to_string(), "$name: [String]");
    }

    #[test]
    fn test_display_references() {
        assert_eq!(
            Argument::references("name", "reference").to_string(),
            "$name: [reference]"
        );
    }

    #[test]
    fn test_display_owned_references() {
        assert_eq!(
            Argument::owned_references("name", "reference").to_string(),
            "$name: [@reference]"
        );
    }
}
