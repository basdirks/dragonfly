use {
    super::BlockAttribute,
    crate::generator::printer::{
        indent,
        newline_separated,
        space_separated,
        Print,
    },
    std::{
        fmt::{
            Display,
            Write,
        },
        io,
    },
};
pub use {
    field::Field,
    field_type::FieldType,
};

/// Model fields.
pub mod field;
/// Field types.
pub mod field_type;

/// A Prisma model.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Model {
    /// The name of the model. Must adhere to `[A-Za-z][A-Za-z0-9_]*`. Usually
    /// pascal case. May not be a reserved Prisma keyword or a JavaScript
    /// reserved keyword. Should be singular.
    pub name: String,
    /// The fields of the model.
    pub fields: Vec<Field>,
    /// Block attributes.
    pub attributes: Vec<BlockAttribute>,
}

impl Model {
    /// Create a new model.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    /// * `fields` - The fields of the model.
    /// * `attributes` - Block attributes.
    #[must_use]
    pub fn new(
        name: &str,
        fields: &[Field],
        attributes: &[BlockAttribute],
    ) -> Self {
        Self {
            name: name.to_owned(),
            fields: fields.to_owned(),
            attributes: attributes.to_owned(),
        }
    }

    /// Create a new model with the standard `id`, `createdAt`, and `updatedAt`
    /// fields.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    /// * `fields` - The fields of the model.
    /// * `attributes` - Block attributes.
    #[must_use]
    pub fn standard(
        name: &str,
        fields: &[Field],
        attributes: &[BlockAttribute],
    ) -> Self {
        let mut fields = fields.to_owned();

        fields.push(Field::id());
        fields.push(Field::created_at());

        Self::new(name, &fields, attributes)
    }
}

impl Display for Model {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> std::fmt::Result {
        let Self {
            name,
            fields,
            attributes,
        } = self;

        let max_field_name_length = fields
            .iter()
            .map(|field| field.name.len())
            .max()
            .unwrap_or(0)
            + 1;

        let max_field_type_length = fields
            .iter()
            .map(|field| field.print_type().len())
            .max()
            .unwrap_or(0)
            + 1;

        let indent = indent::psl(1);

        let mut fields = fields
            .iter()
            .map(|field| {
                let Field {
                    name, attributes, ..
                } = field;

                let mut string =
                    format!("{indent}{name:<max_field_name_length$}");
                let r#type = field.print_type();

                if attributes.is_empty() {
                    let _: Result<(), std::fmt::Error> =
                        write!(string, "{type}");
                } else {
                    let attributes = space_separated(attributes);

                    let _: Result<(), std::fmt::Error> = write!(
                        string,
                        "{type:<max_field_type_length$}{attributes}"
                    );
                }

                string
            })
            .collect::<Vec<_>>();

        fields.sort();

        let fields = newline_separated(&fields);

        let attributes = if attributes.is_empty() {
            String::new()
        } else {
            format!("\n\n{}", space_separated(attributes))
        };

        write!(f, "model {name} {{\n{fields}{attributes}\n}}")
    }
}

impl Print for Model {
    fn print(
        &self,
        _: usize,
        f: &mut dyn io::Write,
    ) -> io::Result<()> {
        write!(f, "{self}")
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::generator::prisma::{
            Argument,
            FieldAttribute,
            Value,
        },
    };

    #[test]
    fn test_new() {
        let model = Model::new(
            "User",
            &[
                Field::id(),
                Field::boolean(
                    "isAdmin",
                    &[FieldAttribute::new(
                        "default",
                        &[Argument::unnamed(&Value::Boolean(false))],
                        None,
                    )],
                ),
                Field::string("firstName", &[]),
                Field::string("lastName", &[]),
            ],
            &[BlockAttribute::new(
                "unique",
                &[Argument::unnamed(&Value::Array(vec![
                    Value::keyword("firstName"),
                    Value::keyword("lastName"),
                ]))],
                None,
            )],
        );

        let mut f = Vec::new();

        model.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "model User {
  firstName String
  id        Int     @id @default(autoincrement())
  isAdmin   Boolean @default(false)
  lastName  String

  @@unique([firstName, lastName])
}"
        );
    }

    #[test]
    fn test_standard() {
        let model = Model::standard("User", &[], &[]);

        let mut f = Vec::new();

        model.print(0, &mut f).unwrap();

        assert_eq!(
            String::from_utf8(f).unwrap(),
            "model User {
  createdAt DateTime @default(now())
  id        Int      @id @default(autoincrement())
}"
        );
    }
}
