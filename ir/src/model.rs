#![allow(clippy::module_name_repetitions)]
use {
    crate::{
        Cardinality,
        TypeError,
    },
    ord_str_map::OrdStrMap,
    std::{
        borrow::Cow,
        collections::BTreeSet,
    },
};
pub use {
    enum_relation::EnumRelation,
    field::Field,
    model_relation::ModelRelation,
};

/// Enum relations.
pub mod enum_relation;
/// Model fields.
pub mod field;
/// Model relations.
pub mod model_relation;

/// A model.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct Model<'a> {
    /// The name of the model.
    pub name: Cow<'a, str>,
    /// Names of fields and relations.
    pub keys: BTreeSet<Cow<'a, str>>,
    /// The data fields of the model.
    pub fields: OrdStrMap<Field<'a>>,
    /// Relations to models that this model references.
    pub relations: OrdStrMap<ModelRelation<'a>>,
    /// Relations to enum values.
    pub enums: OrdStrMap<EnumRelation<'a>>,
}

impl<'a> Model<'a> {
    /// Create an empty model.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    #[must_use]
    pub fn new<S>(name: S) -> Self
    where
        S: Into<Cow<'a, str>>,
    {
        Self {
            name: name.into(),
            keys: BTreeSet::new(),
            fields: OrdStrMap::new(),
            relations: OrdStrMap::new(),
            enums: OrdStrMap::new(),
        }
    }

    /// Register the name of a field or relation.
    ///
    /// # Arguments
    ///
    /// * `key` - The name of the field or relation.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the key is already registered.
    pub fn insert_key<S>(
        &mut self,
        key: S,
    ) -> Result<(), TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
    {
        if !self.keys.insert(key.clone().into()) {
            return Err(TypeError::duplicate_model_field(
                self.name.clone(),
                key,
            ));
        }

        Ok(())
    }

    /// Insert a field into the model.
    ///
    /// # Arguments
    ///
    /// * `field` - The field to insert.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_field(
        &mut self,
        field: Field<'a>,
    ) -> Result<(), TypeError<'a>> {
        self.insert_key(field.name.clone())?;

        if self
            .fields
            .insert(field.name.clone(), field.clone())
            .is_some()
        {
            return Err(TypeError::duplicate_model_field(
                self.name.clone(),
                field.name,
            ));
        }

        Ok(())
    }

    /// Insert an enum relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The enum relation to insert.
    /// * `enum_name` - The name of the enum.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_enum_relation<S, T>(
        &mut self,
        field_name: S,
        enum_name: T,
    ) -> Result<(), TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
        T: Into<Cow<'a, str>> + Clone,
    {
        self.insert_key(field_name.clone())?;

        if self
            .enums
            .insert(
                field_name.clone().into().into_owned(),
                EnumRelation {
                    name: enum_name.clone().into(),
                    cardinality: Cardinality::One,
                },
            )
            .is_some()
        {
            return Err(TypeError::duplicate_model_field(
                enum_name, field_name,
            ));
        }

        Ok(())
    }

    /// Insert a relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `relation` - The relation to insert.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_relation<S>(
        &mut self,
        field_name: S,
        relation: ModelRelation<'a>,
    ) -> Result<(), TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
    {
        self.insert_key(field_name.clone())?;

        if self
            .relations
            .insert(field_name.clone().into(), relation.clone())
            .is_some()
        {
            return Err(TypeError::duplicate_model_field(
                relation.model_name,
                field_name,
            ));
        }

        Ok(())
    }

    /// Insert a one-to-one relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_one_to_one<S, T>(
        &mut self,
        field_name: S,
        model_name: T,
    ) -> Result<(), TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
        T: Into<Cow<'a, str>>,
    {
        self.insert_relation(
            field_name,
            ModelRelation {
                model_name: model_name.into(),
                r#type: model_relation::Type::OneToOne,
            },
        )
    }

    /// Insert a one-to-many relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_one_to_many<S, T>(
        &mut self,
        field_name: S,
        model_name: T,
    ) -> Result<(), TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
        T: Into<Cow<'a, str>>,
    {
        self.insert_relation(
            field_name,
            ModelRelation {
                model_name: model_name.into(),
                r#type: model_relation::Type::OneToMany,
            },
        )
    }

    /// Insert a many-to-one relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_many_to_one<S, T>(
        &mut self,
        field_name: S,
        model_name: T,
    ) -> Result<(), TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
        T: Into<Cow<'a, str>>,
    {
        self.insert_relation(
            field_name,
            ModelRelation {
                model_name: model_name.into(),
                r#type: model_relation::Type::ManyToOne,
            },
        )
    }

    /// Insert a many-to-many relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_many_to_many<S, T>(
        &mut self,
        field_name: S,
        model_name: T,
    ) -> Result<(), TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
        T: Into<Cow<'a, str>>,
    {
        self.insert_relation(
            field_name,
            ModelRelation {
                model_name: model_name.into(),
                r#type: model_relation::Type::ManyToMany,
            },
        )
    }

    /// Insert an enum array relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `enum_name` - The name of the enum.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_enums_relation<S, T>(
        &mut self,
        field_name: S,
        enum_name: T,
    ) -> Result<(), TypeError<'a>>
    where
        S: Into<Cow<'a, str>> + Clone,
        T: Into<Cow<'a, str>>,
    {
        self.insert_key(field_name.clone())?;

        if self
            .enums
            .insert(
                field_name.clone().into(),
                EnumRelation {
                    name: enum_name.into(),
                    cardinality: Cardinality::Many,
                },
            )
            .is_some()
        {
            return Err(TypeError::duplicate_model_field(
                self.name.clone(),
                field_name,
            ));
        }

        Ok(())
    }

    /// Get the name of the model.
    #[must_use]
    pub fn name(&self) -> Cow<'a, str> {
        self.name.clone()
    }

    /// Get a field by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn field<S>(
        &self,
        name: S,
    ) -> Option<Field<'a>>
    where
        S: AsRef<str>,
    {
        self.fields.get(name.as_ref()).cloned()
    }

    /// Get an enum relation by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the relation.
    #[must_use]
    pub fn enum_relation<S>(
        &self,
        name: S,
    ) -> Option<EnumRelation<'a>>
    where
        S: AsRef<str>,
    {
        self.enums.get(name.as_ref()).cloned()
    }

    /// Get a model relation by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the relation.
    #[must_use]
    pub fn model_relation<S>(
        &self,
        name: S,
    ) -> Option<ModelRelation<'a>>
    where
        S: AsRef<str>,
    {
        self.relations.get(name.as_ref()).cloned()
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
        crate::Type,
    };

    #[test]
    fn test_new() {
        assert_eq!(
            Model::new("User"),
            Model {
                name: "User".into(),
                fields: OrdStrMap::new(),
                relations: OrdStrMap::new(),
                enums: OrdStrMap::new(),
                keys: BTreeSet::new(),
            }
        );
    }

    #[test]
    fn test_insert_field_duplicate() {
        let mut model = Model::new("User");

        model
            .insert_field(Field {
                name: "name".into(),
                r#type: Type::String,
                cardinality: Cardinality::One,
            })
            .unwrap();

        assert_eq!(
            model.insert_field(Field {
                name: "name".into(),
                r#type: Type::String,
                cardinality: Cardinality::One,
            }),
            Err(TypeError::duplicate_model_field("User", "name"))
        );
    }

    #[test]
    fn test_insert_enum_relation_duplicate() {
        let mut model = Model::new("User");

        model.insert_enums_relation("name", "Name").unwrap();

        assert_eq!(
            model.insert_enums_relation("name", "Name"),
            Err(TypeError::duplicate_model_field("User", "name"))
        );
    }

    #[test]
    fn test_insert_relation_duplicate() {
        let mut model = Model::new("User");

        model.insert_one_to_many("name", "Name").unwrap();

        assert_eq!(
            model.insert_one_to_many("name", "Name"),
            Err(TypeError::duplicate_model_field("User", "name"))
        );
    }

    #[test]
    fn test_insert_key_duplicate() {
        let mut model = Model::new("User");

        model.insert_key("name").unwrap();

        assert_eq!(
            model.insert_key("name"),
            Err(TypeError::duplicate_model_field("User", "name"))
        );
    }
}
