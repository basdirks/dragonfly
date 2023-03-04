pub use self::{
    enum_relation::EnumRelation,
    field::Field,
    relation::Relation,
};
use std::collections::BTreeMap;

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
    pub name: String,
    /// The data fields of the model.
    pub fields: BTreeMap<String, Field>,
    /// Relations to models that this model owns.
    pub owned_models: BTreeMap<String, Relation>,
    /// Relations to models that this model references.
    pub models: BTreeMap<String, Relation>,
    /// Relations to enum values.
    pub enums: BTreeMap<String, EnumRelation>,
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
            fields: BTreeMap::new(),
            owned_models: BTreeMap::new(),
            models: BTreeMap::new(),
            enums: BTreeMap::new(),
        }
    }

    /// Insert a field into the model.
    ///
    /// # Arguments
    ///
    /// * `field` - The field to insert.
    pub fn insert_field(
        &mut self,
        field: Field,
    ) {
        let _ = self.fields.insert(field.name.clone(), field);
    }

    /// Insert an enum relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The enum relation to insert.
    /// * `enum_name` - The name of the enum.
    pub fn insert_enum(
        &mut self,
        field_name: &str,
        enum_name: &str,
    ) {
        let _ = self
            .enums
            .insert(field_name.to_owned(), EnumRelation::one(enum_name));
    }

    /// Insert a model relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    pub fn insert_model(
        &mut self,
        field_name: &str,
        model_name: &str,
    ) {
        let _ = self
            .models
            .insert(field_name.to_owned(), Relation::one(model_name));
    }

    /// Insert an owned model relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    pub fn insert_owned_model(
        &mut self,
        field_name: &str,
        model_name: &str,
    ) {
        let _ = self
            .owned_models
            .insert(field_name.to_owned(), Relation::one(model_name));
    }

    /// Insert an enum array relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `enum_name` - The name of the enum.
    pub fn insert_enums(
        &mut self,
        field_name: &str,
        enum_name: &str,
    ) {
        let _ = self
            .enums
            .insert(field_name.to_owned(), EnumRelation::many(enum_name));
    }

    /// Insert a model array relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    pub fn insert_models(
        &mut self,
        field_name: &str,
        model_name: &str,
    ) {
        let _ = self
            .models
            .insert(field_name.to_owned(), Relation::many(model_name));
    }

    /// Insert an owned model array relation into the model.
    ///
    /// # Arguments
    ///
    /// * `field_name` - The name of the field.
    /// * `model_name` - The name of the model.
    pub fn insert_owned_models(
        &mut self,
        field_name: &str,
        model_name: &str,
    ) {
        let _ = self
            .owned_models
            .insert(field_name.to_owned(), Relation::many(model_name));
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
