use serde::ser::SerializeStruct;

use super::{ItemErrorsMap, PropertyErrorsMap, VecErrors};

#[derive(Debug, Clone, thiserror::Error)]
pub struct MixedErrors<E = crate::validation::Error> {
    pub errors: VecErrors<E>,
    pub items: ItemErrorsMap<E>,
    pub properties: PropertyErrorsMap<E>,
}

impl<E> MixedErrors<E> {
    pub fn new(
        errors: VecErrors<E>,
        items: ItemErrorsMap<E>,
        properties: PropertyErrorsMap<E>,
    ) -> Self {
        Self {
            errors,
            items,
            properties,
        }
    }
}

impl<E> serde::Serialize for MixedErrors<E>
where
    E: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut mixed_errors = serializer.serialize_struct("MixedErrors", 3)?;
        mixed_errors.serialize_field("errors", &self.errors)?;
        mixed_errors.serialize_field("items", &self.items)?;
        mixed_errors.serialize_field("properties", &self.properties)?;
        mixed_errors.end()
    }
}

impl<E> std::fmt::Display for MixedErrors<E>
where
    E: serde::Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(self) {
            Ok(json_string) => f.write_str(&json_string),
            Err(_) => Err(std::fmt::Error),
        }
    }
}
