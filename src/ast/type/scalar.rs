use {
    crate::{
        literal,
        map,
        parser::{
            at,
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
    /// A reference to a unique model.
    Owned(String),
    /// A reference to an enum or model.
    Reference(String),
    /// A UTF-8 string.
    String,
}

impl Scalar {
    /// Create a reference.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the reference.
    #[must_use]
    pub fn reference(name: &str) -> Self {
        Self::Reference(name.to_owned())
    }

    /// Create an owned reference.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the reference.
    #[must_use]
    pub fn owned_reference(name: &str) -> Self {
        Self::Owned(name.to_owned())
    }

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
    ///     Ok((Scalar::Boolean, String::new()))
    /// );
    ///
    /// assert_eq!(
    ///     Scalar::parse("DateTime"),
    ///     Ok((Scalar::DateTime, String::new()))
    /// );
    ///
    /// assert_eq!(Scalar::parse("Float"), Ok((Scalar::Float, String::new())));
    /// assert_eq!(Scalar::parse("Int"), Ok((Scalar::Int, String::new())));
    /// assert_eq!(Scalar::parse("String"), Ok((Scalar::String, String::new())));
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::Scalar;
    ///
    /// assert_eq!(
    ///     Scalar::parse("Foo"),
    ///     Ok((Scalar::Reference("Foo".to_owned()), String::new()))
    /// );
    /// ```
    ///
    /// ```rust
    /// use dragonfly::ast::Scalar;
    ///
    /// assert_eq!(
    ///     Scalar::parse("@Foo"),
    ///     Ok((Scalar::Owned("Foo".to_owned()), String::new()))
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
                map!(
                    |input| {
                        let (_, input) = at(input)?;
                        let (name, input) = capitalized(&input)?;

                        Ok((name, input))
                    },
                    Self::Owned
                ),
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
            Self::Owned(name) => write!(f, "@{name}"),
            Self::Reference(name) => write!(f, "{name}"),
            Self::String => write!(f, "String"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_boolean() {
        assert_eq!(Scalar::Boolean.to_string(), "Boolean");
    }

    #[test]
    fn test_display_date_time() {
        assert_eq!(Scalar::DateTime.to_string(), "DateTime");
    }

    #[test]
    fn test_display_float() {
        assert_eq!(Scalar::Float.to_string(), "Float");
    }

    #[test]
    fn test_display_int() {
        assert_eq!(Scalar::Int.to_string(), "Int");
    }

    #[test]
    fn test_display_string() {
        assert_eq!(Scalar::String.to_string(), "String");
    }

    #[test]
    fn test_display_reference() {
        assert_eq!(Scalar::reference("Foo").to_string(), "Foo");
    }

    #[test]
    fn test_display_owned() {
        assert_eq!(Scalar::owned_reference("Foo").to_string(), "@Foo");
    }
}
