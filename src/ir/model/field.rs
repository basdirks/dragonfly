use crate::ir::{
    Cardinality,
    Type,
};

/// A data field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Field {
    /// The name of the field.
    pub name: String,
    /// The type of the field.
    pub r#type: Type,
    /// The cardinality of the field.
    pub cardinality: Cardinality,
}

impl Field {
    /// Create a new field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `r#type` - The type of the field.
    /// * `cardinality` - The cardinality of the field.
    #[must_use]
    pub fn new(
        name: &str,
        r#type: Type,
        cardinality: Cardinality,
    ) -> Self {
        Self {
            name: name.to_owned(),
            r#type,
            cardinality,
        }
    }

    /// Create a field with a single boolean.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn boolean(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::Boolean,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with a single date time.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn date_time(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::DateTime,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with a single floating point number.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn float(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::Float,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with a single integer.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn int(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::Int,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with a single string.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn string(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::String,
            cardinality: Cardinality::One,
        }
    }

    /// Create a field with an array of booleans.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn booleans(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::Boolean,
            cardinality: Cardinality::Many,
        }
    }

    /// Create a field with an array of date times.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn date_times(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::DateTime,
            cardinality: Cardinality::Many,
        }
    }

    /// Create a field with an array of floating point numbers.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn floats(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::Float,
            cardinality: Cardinality::Many,
        }
    }

    /// Create a field with an array of integers.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn ints(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::Int,
            cardinality: Cardinality::Many,
        }
    }

    /// Create a field with an array of strings.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn strings(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            r#type: Type::String,
            cardinality: Cardinality::Many,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_field() {
        assert_eq!(
            Field::new("age", Type::Int, Cardinality::One),
            Field {
                name: "age".to_owned(),
                r#type: Type::Int,
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_field_boolean() {
        assert_eq!(
            Field::boolean("is_active"),
            Field {
                name: "is_active".to_owned(),
                r#type: Type::Boolean,
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_field_date_time() {
        assert_eq!(
            Field::date_time("created_at"),
            Field {
                name: "created_at".to_owned(),
                r#type: Type::DateTime,
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_field_float() {
        assert_eq!(
            Field::float("price"),
            Field {
                name: "price".to_owned(),
                r#type: Type::Float,
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_field_int() {
        assert_eq!(
            Field::int("age"),
            Field {
                name: "age".to_owned(),
                r#type: Type::Int,
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_field_string() {
        assert_eq!(
            Field::string("name"),
            Field {
                name: "name".to_owned(),
                r#type: Type::String,
                cardinality: Cardinality::One,
            }
        );
    }

    #[test]
    fn test_field_booleans() {
        assert_eq!(
            Field::booleans("bits"),
            Field {
                name: "bits".to_owned(),
                r#type: Type::Boolean,
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_field_date_times() {
        assert_eq!(
            Field::date_times("events"),
            Field {
                name: "events".to_owned(),
                r#type: Type::DateTime,
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_field_floats() {
        assert_eq!(
            Field::floats("intervals"),
            Field {
                name: "intervals".to_owned(),
                r#type: Type::Float,
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_field_ints() {
        assert_eq!(
            Field::ints("ages"),
            Field {
                name: "ages".to_owned(),
                r#type: Type::Int,
                cardinality: Cardinality::Many,
            }
        );
    }

    #[test]
    fn test_field_strings() {
        assert_eq!(
            Field::strings("names"),
            Field {
                name: "names".to_owned(),
                r#type: Type::String,
                cardinality: Cardinality::Many,
            }
        );
    }
}
