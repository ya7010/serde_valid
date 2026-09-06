use serde::ser::SerializeStruct;

use super::{ItemErrorsMap, VecErrors};

#[derive(Debug, Clone, thiserror::Error)]
pub struct ArrayErrors<E = crate::validation::Error> {
    pub errors: VecErrors<E>,
    pub items: ItemErrorsMap<E>,
}

impl<E> serde::Serialize for ArrayErrors<E>
where
    E: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut array_errors = serializer.serialize_struct("ArrayErrors", 2)?;
        array_errors.serialize_field("errors", &self.errors)?;
        array_errors.serialize_field("items", &self.items)?;
        array_errors.end()
    }
}

impl<E> ArrayErrors<E> {
    pub fn new(errors: VecErrors<E>, items: ItemErrorsMap<E>) -> Self {
        Self { errors, items }
    }
}

impl<E> ArrayErrors<E> {
    pub fn merge(mut self, other: ArrayErrors<E>) -> Self {
        self.errors.extend(other.errors);

        for (index, item) in other.items {
            match self.items.get_mut(&index) {
                Some(errors) => errors.merge(item),
                None => {
                    self.items.insert(index, item);
                }
            };
        }
        self
    }
}

impl<E> std::fmt::Display for ArrayErrors<E>
where
    E: serde::Serialize,
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
