use serde::ser::SerializeStruct;

use super::{PropertyErrorsMap, VecErrors};

#[derive(Debug, Clone, thiserror::Error)]
pub struct ObjectErrors<E = crate::validation::Error> {
    pub errors: VecErrors<E>,
    pub properties: PropertyErrorsMap<E>,
}

impl<E> serde::Serialize for ObjectErrors<E>
where
    E: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut object_errors = serializer.serialize_struct("ObjectErrors", 2)?;
        object_errors.serialize_field("errors", &self.errors)?;
        object_errors.serialize_field("properties", &self.properties)?;
        object_errors.end()
    }
}

impl<E> ObjectErrors<E> {
    pub fn new(errors: VecErrors<E>, properties: PropertyErrorsMap<E>) -> Self {
        Self { errors, properties }
    }
}

impl<E> ObjectErrors<E>
where
    E: Clone,
{
    pub fn merge(mut self, other: ObjectErrors<E>) -> Self {
        self.errors.extend(other.errors);

        for (property, errors) in other.properties {
            match self.properties.get_mut(&property) {
                Some(existing) => existing.merge(errors),
                None => {
                    self.properties.insert(property, errors);
                }
            }
        }

        self
    }
}

impl<E> std::fmt::Display for ObjectErrors<E>
where
    E: std::fmt::Display + serde::Serialize,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match serde_json::to_string(&self) {
            Ok(json_string) => {
                write!(f, "{}", json_string)
            }
            Err(_) => Err(std::fmt::Error),
        }
    }
}
