use {
    crate::{
        literal,
        map,
        parser::{
            between,
            capitalized,
            choice,
            literal,
            map,
            tag,
            ParseResult,
        },
        tag,
    },
    std::fmt::Display,
};

/// Scalar types.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Scalar {
    /// A boolean.
    Boolean,
    /// A date and time.
    DateTime,
    /// A 64-bit floating point number.
    Float,
    /// A 64-bit integer.
    Int,
    /// A reference to an enum or model. Might deserve its own type.
    Reference(String),
    /// A UTF-8 string.
    String,
}

impl Scalar {
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
    /// use dragonfly::ast::Scalar;
    ///
    /// assert_eq!(
    ///     Scalar::parse("Boolean"),
    ///     Ok((Scalar::Boolean, "".to_owned()))
    /// );
    ///
    /// assert_eq!(
    ///     Scalar::parse("DateTime"),
    ///     Ok((Scalar::DateTime, "".to_owned()))
    /// );
    ///
    /// assert_eq!(Scalar::parse("Float"), Ok((Scalar::Float, "".to_owned())));
    /// assert_eq!(Scalar::parse("Int"), Ok((Scalar::Int, "".to_owned())));
    ///
    /// assert_eq!(Scalar::parse("String"), Ok((Scalar::String, "".to_owned())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::Scalar;
    ///
    /// assert_eq!(
    ///     Scalar::parse("Foo"),
    ///     Ok((Scalar::Reference("Foo".to_owned()), "".to_owned()))
    /// );
    /// ```
    pub fn parse(input: &str) -> ParseResult<Self> {
        choice::<Self>(
            input,
            vec![
                tag!(literal!("Boolean"), Self::Boolean),
                tag!(literal!("DateTime"), Self::DateTime),
                tag!(literal!("Float"), Self::Float),
                tag!(literal!("Int"), Self::Int),
                tag!(literal!("String"), Self::String),
                map!(capitalized, Self::Reference),
            ],
        )
    }
}

impl Display for Scalar {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        match self {
            Self::Boolean => write!(f, "Boolean"),
            Self::DateTime => write!(f, "DateTime"),
            Self::Float => write!(f, "Float"),
            Self::Int => write!(f, "Int"),
            Self::Reference(name) => write!(f, "{name}"),
            Self::String => write!(f, "String"),
        }
    }
}

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
    /// assert_eq!(
    ///     Type::parse("String"),
    ///     Ok((Type::Scalar(Scalar::String), "".to_owned()))
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("Int"),
    ///     Ok((Type::Scalar(Scalar::Int), "".to_owned()))
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("Float"),
    ///     Ok((Type::Scalar(Scalar::Float), "".to_owned()))
    /// );
    ///
    /// assert_eq!(
    ///     Type::parse("Boolean"),
    ///     Ok((Type::Scalar(Scalar::Boolean), "".to_owned()))
    /// );
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
    ///     Ok((Type::Array(Scalar::String), "".to_owned())),
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(
    ///     Type::parse("[Foo]"),
    ///     Ok((
    ///         Type::Array(Scalar::Reference("Foo".to_owned())),
    ///         "".to_owned()
    ///     )),
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

    /// Return the scalar type of this type.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use dragonfly::ast::{
    ///     Scalar,
    ///     Type,
    /// };
    ///
    /// assert_eq!(Type::Scalar(Scalar::String).scalar(), &Scalar::String);
    /// assert_eq!(Type::Array(Scalar::String).scalar(), &Scalar::String);
    /// ```
    #[must_use]
    pub const fn scalar(&self) -> &Scalar {
        match self {
            Self::Scalar(scalar) | Self::Array(scalar) => scalar,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_scalar() {
        assert_eq!(Scalar::Boolean.to_string(), "Boolean");
        assert_eq!(Scalar::DateTime.to_string(), "DateTime");
        assert_eq!(Scalar::Float.to_string(), "Float");
        assert_eq!(Scalar::Int.to_string(), "Int");
        assert_eq!(Scalar::String.to_string(), "String");
        assert_eq!(Scalar::Reference("Foo".to_owned()).to_string(), "Foo");
    }

    #[test]
    fn test_display_type() {
        assert_eq!(Type::Scalar(Scalar::String).to_string(), "String");
        assert_eq!(Type::Scalar(Scalar::Int).to_string(), "Int");
        assert_eq!(Type::Scalar(Scalar::Float).to_string(), "Float");
        assert_eq!(Type::Scalar(Scalar::Boolean).to_string(), "Boolean");

        assert_eq!(
            Type::Scalar(Scalar::Reference("Foo".to_owned())).to_string(),
            "Foo"
        );

        assert_eq!(Type::Array(Scalar::String).to_string(), "[String]");
        assert_eq!(Type::Array(Scalar::Int).to_string(), "[Int]");
        assert_eq!(Type::Array(Scalar::Float).to_string(), "[Float]");
        assert_eq!(Type::Array(Scalar::Boolean).to_string(), "[Boolean]");

        assert_eq!(
            Type::Array(Scalar::Reference("Foo".to_owned())).to_string(),
            "[Foo]"
        );
    }
}
