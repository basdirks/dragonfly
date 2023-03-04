use {
    super::field_type::FieldType,
    crate::generator::prisma::FieldAttribute,
};

/// A field.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Field {
    /// The name of the field. Must adhere to the following regular expression:
    /// [A-Za-z][A-Za-z0-9_]*. Must start with an alphabetic character. Usually
    /// camel case.
    pub name: String,
    /// The type of the field.
    pub r#type: FieldType,
    /// Is the field required?
    pub required: bool,
    /// Is the field an array?
    pub array: bool,
    /// Field attributes.
    pub attributes: Vec<FieldAttribute>,
}

impl Field {
    /// Create a new field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `r#type` - The type of the field.
    /// * `required` - Is the field required?
    /// * `array` - Is the field an array?
    /// * `attributes` - Field attributes.
    #[must_use]
    pub fn new(
        name: &str,
        r#type: FieldType,
        required: bool,
        array: bool,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self {
            name: name.to_owned(),
            r#type,
            required,
            array,
            attributes: attributes.to_owned(),
        }
    }

    /// Create a new boolean field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `attributes` - Field attributes.
    #[must_use]
    pub fn boolean(
        name: &str,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self::new(name, FieldType::name("Boolean"), true, false, attributes)
    }

    /// Create a new integer field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `attributes` - Field attributes.
    #[must_use]
    pub fn int(
        name: &str,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self::new(name, FieldType::name("Int"), true, false, attributes)
    }

    /// Create a new date time field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `attributes` - Field attributes.
    #[must_use]
    pub fn date_time(
        name: &str,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self::new(name, FieldType::name("DateTime"), true, false, attributes)
    }

    /// Create a new string field.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    /// * `attributes` - Field attributes.
    #[must_use]
    pub fn string(
        name: &str,
        attributes: &[FieldAttribute],
    ) -> Self {
        Self::new(name, FieldType::name("String"), true, false, attributes)
    }

    /// Standard `id` field.
    #[must_use]
    pub fn id() -> Self {
        Self::int(
            "id",
            &[
                FieldAttribute::id(),
                FieldAttribute::default_auto_increment(),
            ],
        )
    }

    /// Standard `createdAt` field.
    #[must_use]
    pub fn created_at() -> Self {
        Self::date_time("createdAt", &[FieldAttribute::default_now()])
    }

    /// Standard `updatedAt` field.
    #[must_use]
    pub fn updated_at() -> Self {
        Self::date_time("updatedAt", &[])
    }

    /// Print the type of the field.
    #[must_use]
    pub fn print_type(&self) -> String {
        let Self {
            r#type,
            required,
            array,
            ..
        } = self;

        let array = if *array { "[]" } else { "" };
        let optional = if *required { "" } else { "?" };

        format!("{type}{array}{optional}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_print_type_new() {
        assert_eq!(
            Field::new(
                "id",
                FieldType::Name("Int".to_owned()),
                true,
                true,
                &[]
            )
            .print_type(),
            "Int[]"
        );
    }

    #[test]
    fn test_print_type_boolean() {
        assert_eq!(Field::boolean("isAdmin", &[]).print_type(), "Boolean");
    }

    #[test]
    fn test_print_type_int() {
        assert_eq!(Field::int("id", &[]).print_type(), "Int");
    }

    #[test]
    fn test_print_type_date_time() {
        assert_eq!(Field::date_time("createdAt", &[]).print_type(), "DateTime");
    }

    #[test]
    fn test_print_type_string() {
        assert_eq!(Field::string("name", &[]).print_type(), "String");
    }

    #[test]
    fn test_boolean() {
        let field = Field::boolean("isAdmin", &[]);

        assert_eq!(field.name, "isAdmin");
        assert_eq!(field.r#type, FieldType::name("Boolean"));
        assert!(field.required);
        assert!(!field.array);
        assert_eq!(field.attributes, vec![]);
    }

    #[test]
    fn test_int() {
        let field = Field::int("id", &[]);

        assert_eq!(field.name, "id");
        assert_eq!(field.r#type, FieldType::name("Int"));
        assert!(field.required);
        assert!(!field.array);
        assert_eq!(field.attributes, vec![]);
    }

    #[test]
    fn test_date_time() {
        let field = Field::date_time("createdAt", &[]);

        assert_eq!(field.name, "createdAt");
        assert_eq!(field.r#type, FieldType::name("DateTime"));
        assert!(field.required);
        assert!(!field.array);
        assert_eq!(field.attributes, vec![]);
    }

    #[test]
    fn test_string() {
        let field = Field::string("name", &[]);

        assert_eq!(field.name, "name");
        assert_eq!(field.r#type, FieldType::name("String"));
        assert!(field.required);
        assert!(!field.array);
        assert_eq!(field.attributes, vec![]);
    }

    #[test]
    fn test_id() {
        let field = Field::id();

        assert_eq!(field.name, "id");
        assert_eq!(field.r#type, FieldType::name("Int"));
        assert!(field.required);
        assert!(!field.array);
        assert_eq!(
            field.attributes,
            vec![
                FieldAttribute::id(),
                FieldAttribute::default_auto_increment()
            ]
        );
    }

    #[test]
    fn test_created_at() {
        let field = Field::created_at();

        assert_eq!(field.name, "createdAt");
        assert_eq!(field.r#type, FieldType::name("DateTime"));
        assert!(field.required);
        assert!(!field.array);
        assert_eq!(field.attributes, vec![FieldAttribute::default_now()]);
    }

    #[test]
    fn test_updated_at() {
        let field = Field::updated_at();

        assert_eq!(field.name, "updatedAt");
        assert_eq!(field.r#type, FieldType::name("DateTime"));
        assert!(field.required);
        assert!(!field.array);
        assert_eq!(field.attributes, vec![]);
    }
}
