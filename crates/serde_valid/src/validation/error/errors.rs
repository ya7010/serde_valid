use super::{ArrayErrors, ItemErrorsMap, MixedErrors, ObjectErrors, PropertyErrorsMap, VecErrors};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Errors<E = crate::validation::Error> {
    Array(ArrayErrors<E>),
    Object(ObjectErrors<E>),
    NewType(VecErrors<E>),
    Mixed(Box<MixedErrors<E>>),
}

impl<E> serde::Serialize for Errors<E>
where
    E: serde::Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Array(a) => serde::Serialize::serialize(a, serializer),
            Self::Object(o) => serde::Serialize::serialize(o, serializer),
            Self::NewType(n) => {
                #[derive(Debug, Clone, serde::Serialize)]
                struct NewTypeErrors<'a, E> {
                    errors: &'a VecErrors<E>,
                }

                serde::Serialize::serialize(&NewTypeErrors { errors: n }, serializer)
            }
            Self::Mixed(mixed) => serde::Serialize::serialize(mixed, serializer),
        }
    }
}

impl<E> Errors<E> {
    pub fn merge(&mut self, other: Errors<E>) {
        let current = std::mem::replace(self, Errors::NewType(Vec::new()));
        let mut parts = ErrorParts::from(current);
        parts.merge(ErrorParts::from(other));
        *self = parts.into();
    }
}

struct ErrorParts<E> {
    errors: VecErrors<E>,
    items: Option<ItemErrorsMap<E>>,
    properties: Option<PropertyErrorsMap<E>>,
}

impl<E> ErrorParts<E> {
    fn merge(&mut self, other: Self) {
        self.errors.extend(other.errors);
        merge_structural_errors(&mut self.items, other.items);
        merge_structural_errors(&mut self.properties, other.properties);
    }
}

fn merge_structural_errors<K, E>(
    target: &mut Option<indexmap::IndexMap<K, Errors<E>>>,
    source: Option<indexmap::IndexMap<K, Errors<E>>>,
) where
    K: std::hash::Hash + Eq,
{
    let Some(source) = source else {
        return;
    };

    let Some(target) = target else {
        *target = Some(source);
        return;
    };

    for (key, errors) in source {
        match target.get_mut(&key) {
            Some(existing) => existing.merge(errors),
            None => {
                target.insert(key, errors);
            }
        }
    }
}

impl<E> From<Errors<E>> for ErrorParts<E> {
    fn from(errors: Errors<E>) -> Self {
        match errors {
            Errors::Array(errors) => Self {
                errors: errors.errors,
                items: Some(errors.items),
                properties: None,
            },
            Errors::Object(errors) => Self {
                errors: errors.errors,
                items: None,
                properties: Some(errors.properties),
            },
            Errors::NewType(errors) => Self {
                errors,
                items: None,
                properties: None,
            },
            Errors::Mixed(errors) => Self {
                errors: errors.errors,
                items: Some(errors.items),
                properties: Some(errors.properties),
            },
        }
    }
}

impl<E> From<ErrorParts<E>> for Errors<E> {
    fn from(parts: ErrorParts<E>) -> Self {
        match (parts.items, parts.properties) {
            (Some(items), Some(properties)) => {
                Errors::Mixed(Box::new(MixedErrors::new(parts.errors, items, properties)))
            }
            (Some(items), None) => Errors::Array(ArrayErrors::new(parts.errors, items)),
            (None, Some(properties)) => Errors::Object(ObjectErrors::new(parts.errors, properties)),
            (None, None) => Errors::NewType(parts.errors),
        }
    }
}

impl<E> std::fmt::Display for Errors<E>
where
    E: serde::Serialize + std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Array(errors) => std::fmt::Display::fmt(errors, f),
            Self::Object(errors) => std::fmt::Display::fmt(errors, f),
            Self::NewType(vec_errors) => {
                let errors = &vec_errors
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>();
                let value = serde_json::json!({ "errors": errors });
                std::fmt::Display::fmt(&value, f)
            }
            Self::Mixed(errors) => std::fmt::Display::fmt(errors, f),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    };

    use super::*;
    use crate::validation::PropertyErrorsMap;

    #[derive(Debug)]
    struct CloneTracker(Arc<AtomicUsize>);

    impl Clone for CloneTracker {
        fn clone(&self) -> Self {
            self.0.fetch_add(1, Ordering::Relaxed);
            Self(Arc::clone(&self.0))
        }
    }

    #[test]
    fn merging_errors_does_not_clone_existing_errors() {
        let clone_count = Arc::new(AtomicUsize::new(0));
        let mut errors = Errors::Object(ObjectErrors::new(
            vec![CloneTracker(Arc::clone(&clone_count))],
            PropertyErrorsMap::new(),
        ));

        errors.merge(Errors::Object(ObjectErrors::new(
            Vec::new(),
            PropertyErrorsMap::new(),
        )));

        assert_eq!(clone_count.load(Ordering::Relaxed), 0);

        let mut errors = Errors::NewType(vec![CloneTracker(Arc::clone(&clone_count))]);
        errors.merge(Errors::Array(ArrayErrors::new(
            Vec::new(),
            ItemErrorsMap::new(),
        )));
        assert_eq!(clone_count.load(Ordering::Relaxed), 0);

        let mut errors = Errors::NewType(vec![CloneTracker(Arc::clone(&clone_count))]);
        errors.merge(Errors::Object(ObjectErrors::new(
            Vec::new(),
            PropertyErrorsMap::new(),
        )));
        assert_eq!(clone_count.load(Ordering::Relaxed), 0);
    }

    #[test]
    fn merging_errors_does_not_require_clone() {
        #[derive(Debug)]
        struct NotClone;

        let mut errors = Errors::NewType(vec![NotClone]);
        errors.merge(Errors::Object(ObjectErrors::new(
            Vec::new(),
            PropertyErrorsMap::new(),
        )));
    }
}
