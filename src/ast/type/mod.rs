pub use self::scalar::Scalar;
use {
    crate::parser::{
        between,
        choice,
        map,
        ParseResult,
    },
    std::fmt::Display,
};

/// A scalar type.
pub mod scalar;

/// A type: a scalar, a reference to a model or enum, or an array.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Type {
    /// An array of scalars.
    Array(Scalar),
    /// A basic type.
    Scalar(Scalar),
}

impl Display for Type {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Array(scalar) => write!(f, "[{scalar}]"),
            Self::Scalar(scalar) => write!(f, "{scalar}"),
        }
    }
}

impl Type {
    /// Parse a scalar type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid scalar
    /// type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::{
    ///         Scalar,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(Type::parse("String"), Ok((Type::string(), String::new())));
    /// ```
    pub fn parse_scalar(input: &str) -> ParseResult<Self> {
        map(input, Scalar::parse, Self::Scalar)
    }

    /// Parse an array type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid array
    /// type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::{
    ///         Scalar,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(
    ///     Type::parse("[String]"),
    ///     Ok((Type::strings(), String::new())),
    /// );
    ///
    /// assert!(Type::parse("[]").is_err());
    /// ```
    pub fn parse_array(input: &str) -> ParseResult<Self> {
        let (scalar, input) = between(input, "[", Scalar::parse, "]")?;

        Ok((Self::Array(scalar), input))
    }

    /// Parse a type from the given input.
    ///
    /// # Arguments
    ///
    /// * `input` - The input to parse.
    ///
    /// # Errors
    ///
    /// Returns `ParseError` if the input does not start with a valid type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::{
    ///     ast::{
    ///         Scalar,
    ///         Type,
    ///     },
    ///     parser::ParseError,
    /// };
    ///
    /// assert_eq!(Type::parse("String"), Ok((Type::string(), String::new())));
    /// assert_eq!(Type::parse("Int"), Ok((Type::int(), String::new())));
    /// assert_eq!(Type::parse("Float"), Ok((Type::float(), String::new())));
    /// assert_eq!(Type::parse("Boolean"), Ok((Type::boolean(), String::new())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Type::parse("[String]"),
    ///     Ok((Type::strings(), String::new())),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::Type;
    ///
    /// assert_eq!(
    ///     Type::parse("[Foo]"),
    ///     Ok((Type::references("Foo"), String::new())),
    /// );
    /// ```
    ///
    /// Nested arrays are not supported:
    ///
    /// ```rust
    /// use dragonfly::ast::Type;
    ///
    /// assert!(Type::parse("[[String]]").is_err());
    /// ```
    ///
    /// A type name must start with an uppercase character:
    ///
    /// ```rust
    /// use dragonfly::ast::Type;
    ///
    /// assert!(Type::parse("foo").is_err());
    /// ```
    ///
    /// An empty string is not a valid type:
    ///
    /// ```rust
    /// use dragonfly::ast::Type;
    ///
    /// assert!(Type::parse("").is_err());
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(input, vec![Self::parse_scalar, Self::parse_array])
    }

    /// Create a boolean type.
    #[must_use]
    pub const fn boolean() -> Self {
        Self::Scalar(Scalar::Boolean)
    }

    /// Create a date time type.
    #[must_use]
    pub const fn date_time() -> Self {
        Self::Scalar(Scalar::DateTime)
    }

    /// Create a float type.
    #[must_use]
    pub const fn float() -> Self {
        Self::Scalar(Scalar::Float)
    }

    /// Create an integer type.
    #[must_use]
    pub const fn int() -> Self {
        Self::Scalar(Scalar::Int)
    }

    /// Create a reference type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the reference type.
    #[must_use]
    pub fn reference(name: &str) -> Self {
        Self::Scalar(Scalar::Reference(name.to_owned()))
    }

    /// Create an owned reference type.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the reference type.
    #[must_use]
    pub fn owned_reference(name: &str) -> Self {
        Self::Scalar(Scalar::Owned(name.to_owned()))
    }

    /// Create a string type.
    #[must_use]
    pub const fn string() -> Self {
        Self::Scalar(Scalar::String)
    }

    /// Create an array of booleans.
    #[must_use]
    pub const fn booleans() -> Self {
        Self::Array(Scalar::Boolean)
    }

    /// Create an array of date times.
    #[must_use]
    pub const fn date_times() -> Self {
        Self::Array(Scalar::DateTime)
    }

    /// Create an array of floats.
    #[must_use]
    pub const fn floats() -> Self {
        Self::Array(Scalar::Float)
    }

    /// Create an array of integers.
    #[must_use]
    pub const fn ints() -> Self {
        Self::Array(Scalar::Int)
    }

    /// Create an array of strings.
    #[must_use]
    pub const fn strings() -> Self {
        Self::Array(Scalar::String)
    }

    /// Create an array of references.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the reference type.
    #[must_use]
    pub fn references(name: &str) -> Self {
        Self::Array(Scalar::Reference(name.to_owned()))
    }

    /// Create an array of owned references.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the reference type.
    #[must_use]
    pub fn owned_references(name: &str) -> Self {
        Self::Array(Scalar::Owned(name.to_owned()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_boolean() {
        assert_eq!(Type::boolean().to_string(), "Boolean");
    }

    #[test]
    fn test_display_date_time() {
        assert_eq!(Type::date_time().to_string(), "DateTime");
    }

    #[test]
    fn test_display_int() {
        assert_eq!(Type::int().to_string(), "Int");
    }

    #[test]
    fn test_display_float() {
        assert_eq!(Type::float().to_string(), "Float");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Type::string().to_string(), "String");
    }

    #[test]
    fn test_display_reference() {
        assert_eq!(Type::reference("Foo").to_string(), "Foo");
    }

    #[test]
    fn test_display_owned_reference() {
        assert_eq!(Type::owned_reference("Foo").to_string(), "@Foo");
    }

    #[test]
    fn test_display_booleans() {
        assert_eq!(Type::booleans().to_string(), "[Boolean]");
    }

    #[test]
    fn test_display_date_times() {
        assert_eq!(Type::date_times().to_string(), "[DateTime]");
    }

    #[test]
    fn test_display_ints() {
        assert_eq!(Type::ints().to_string(), "[Int]");
    }

    #[test]
    fn test_display_floats() {
        assert_eq!(Type::floats().to_string(), "[Float]");
    }

    #[test]
    fn test_display_strings() {
        assert_eq!(Type::strings().to_string(), "[String]");
    }

    #[test]
    fn test_display_references() {
        assert_eq!(Type::references("Foo").to_string(), "[Foo]");
    }

    #[test]
    fn test_display_owned_references() {
        assert_eq!(Type::owned_references("Foo").to_string(), "[@Foo]");
    }
}
