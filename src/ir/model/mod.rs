pub use self::{
    enum_relation::EnumRelation,
    field::Field,
    relation::Relation,
};
use {
    crate::ast::TypeError,
    std::collections::{
        BTreeMap,
        BTreeSet,
    },
};

/// Enum relations.
pub mod enum_relation;
/// Model fields.
pub mod field;
/// Model relations.
pub mod relation;

/// A model.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Model {
    /// The name of the model.
    name: String,
    /// Names of fields and relations.
    keys: BTreeSet<String>,
    /// The data fields of the model.
    fields: BTreeMap<String, Field>,
    /// Relations to models that this model owns.
    owned_models: BTreeMap<String, Relation>,
    /// Relations to models that this model references.
    models: BTreeMap<String, Relation>,
    /// Relations to enum values.
    enums: BTreeMap<String, EnumRelation>,
}

impl Model {
    /// Create an empty model.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the model.
    #[must_use]
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_owned(),
            keys: BTreeSet::new(),
            fields: BTreeMap::new(),
            owned_models: BTreeMap::new(),
            models: BTreeMap::new(),
            enums: BTreeMap::new(),
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
    pub fn insert_key(
        &mut self,
        key: &str,
    ) -> Result<(), TypeError> {
        if !self.keys.insert(key.to_owned()) {
            return Err(TypeError::duplicate_model_field(&self.name, key));
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
        field: Field,
    ) -> Result<(), TypeError> {
        self.insert_key(&field.name)?;

        let _ = self.fields.insert(field.name.clone(), field);

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
    pub fn insert_enum_relation(
        &mut self,
        field_name: &str,
        enum_name: &str,
    ) -> Result<(), TypeError> {
        self.insert_key(field_name)?;

        let _ = self
            .enums
            .insert(field_name.to_owned(), EnumRelation::one(enum_name));

        Ok(())
    }

    /// Insert a model relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_model_relation(
        &mut self,
        field_name: &str,
        model_name: &str,
    ) -> Result<(), TypeError> {
        self.insert_key(field_name)?;

        let _ = self
            .models
            .insert(field_name.to_owned(), Relation::one(model_name));

        Ok(())
    }

    /// Insert an owned model relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_owned_model_relation(
        &mut self,
        field_name: &str,
        model_name: &str,
    ) -> Result<(), TypeError> {
        self.insert_key(field_name)?;

        let _ = self
            .owned_models
            .insert(field_name.to_owned(), Relation::one(model_name));

        Ok(())
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
    pub fn insert_enums_relation(
        &mut self,
        field_name: &str,
        enum_name: &str,
    ) -> Result<(), TypeError> {
        self.insert_key(field_name)?;

        let _ = self
            .enums
            .insert(field_name.to_owned(), EnumRelation::many(enum_name));

        Ok(())
    }

    /// Insert a model array relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_models_relation(
        &mut self,
        field_name: &str,
        model_name: &str,
    ) -> Result<(), TypeError> {
        self.insert_key(field_name)?;

        let _ = self
            .models
            .insert(field_name.to_owned(), Relation::many(model_name));

        Ok(())
    }

    /// Insert an owned model array relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    ///
    /// # Errors
    ///
    /// Returns a `TypeError` if the field name is already registered.
    pub fn insert_owned_models_relation(
        &mut self,
        field_name: &str,
        model_name: &str,
    ) -> Result<(), TypeError> {
        self.insert_key(field_name)?;

        let _ = self
            .owned_models
            .insert(field_name.to_owned(), Relation::many(model_name));

        Ok(())
    }

    /// Get the name of the model.
    #[must_use]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Get a field by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the field.
    #[must_use]
    pub fn field(
        &self,
        name: &str,
    ) -> Option<Field> {
        self.fields.get(name).cloned()
    }

    /// Get an enum relation by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the relation.
    #[must_use]
    pub fn enum_relation(
        &self,
        name: &str,
    ) -> Option<EnumRelation> {
        self.enums.get(name).cloned()
    }

    /// Get a model relation by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the relation.
    #[must_use]
    pub fn model_relation(
        &self,
        name: &str,
    ) -> Option<Relation> {
        self.models.get(name).cloned()
    }

    /// Get an owned model relation by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the relation.
    #[must_use]
    pub fn owned_model_relation(
        &self,
        name: &str,
    ) -> Option<Relation> {
        self.owned_models.get(name).cloned()
    }

    /// Iterate over the fields.
    pub fn fields(&self) -> impl Iterator<Item = (String, Field)> + '_ {
        self.fields.iter().map(|(k, v)| (k.clone(), v.clone()))
    }

    /// Iterate over the enum relations.
    pub fn enum_relations(
        &self
    ) -> impl Iterator<Item = (String, EnumRelation)> + '_ {
        self.enums.iter().map(|(k, v)| (k.clone(), v.clone()))
    }

    /// Iterate over the model relations.
    pub fn model_relations(
        &self
    ) -> impl Iterator<Item = (String, Relation)> + '_ {
        self.models.iter().map(|(k, v)| (k.clone(), v.clone()))
    }

    /// Iterate over the owned model relations.
    pub fn owned_model_relations(
        &self
    ) -> impl Iterator<Item = (String, Relation)> + '_ {
        self.owned_models
            .iter()
            .map(|(k, v)| (k.clone(), v.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let model = Model::new("User");

        assert_eq!(model.name, "User");
        assert!(model.fields.is_empty());
        assert!(model.owned_models.is_empty());
        assert!(model.models.is_empty());
        assert!(model.enums.is_empty());
    }
}
