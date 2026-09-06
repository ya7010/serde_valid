use super::{ArrayErrors, ObjectErrors, VecErrors};

#[derive(Debug, Clone, thiserror::Error)]
pub enum Errors<E = crate::validation::Error> {
    Array(ArrayErrors<E>),
    Object(ObjectErrors<E>),
    NewType(VecErrors<E>),
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
        }
    }
}

impl<E> Errors<E> {
    pub fn merge(&mut self, other: Errors<E>) {
        match self {
            Errors::Array(current) => match other {
                Errors::Array(other) => {
                    current.errors.extend(other.errors);
                    for (index, errors) in other.items {
                        match current.items.get_mut(&index) {
                            Some(current) => current.merge(errors),
                            None => {
                                current.items.insert(index, errors);
                            }
                        }
                    }
                }
                Errors::Object(mut other) => {
                    // Errors cannot represent items and properties at the same level. Keep the
                    // established object precedence while retaining both top-level error lists.
                    let mut errors = std::mem::take(&mut current.errors);
                    errors.extend(other.errors);
                    other.errors = errors;
                    *self = Errors::Object(other);
                }
                Errors::NewType(errors) => current.errors.extend(errors),
            },
            Errors::Object(current) => match other {
                // Keep object structure, but retain top-level errors from the array.
                Errors::Array(other) => current.errors.extend(other.errors),
                Errors::Object(other) => current.merge(other),
                Errors::NewType(errors) => current.errors.extend(errors),
            },
            Errors::NewType(current) => match other {
                Errors::Array(other) => {
                    let mut errors = std::mem::take(current);
                    errors.extend(other.errors);
                    *self = Errors::Array(ArrayErrors::new(errors, other.items));
                }
                Errors::Object(mut other) => {
                    let mut errors = std::mem::take(current);
                    errors.extend(other.errors);
                    other.errors = errors;
                    *self = Errors::Object(other);
                }
                Errors::NewType(errors) => current.extend(errors),
            },
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
    use crate::validation::{ItemErrorsMap, PropertyErrorsMap};

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

    #[test]
    fn object_errors_take_precedence_over_array_errors_without_losing_top_level_errors() {
        let properties =
            PropertyErrorsMap::from([("property".into(), Errors::NewType(vec!["property error"]))]);
        let items = ItemErrorsMap::from([(0, Errors::NewType(vec!["item error"]))]);

        let mut array_then_object =
            Errors::Array(ArrayErrors::new(vec!["array error"], items.clone()));
        array_then_object.merge(Errors::Object(ObjectErrors::new(
            vec!["object error"],
            properties.clone(),
        )));
        let Errors::Object(errors) = array_then_object else {
            panic!("object errors must take precedence")
        };
        assert_eq!(errors.errors, ["array error", "object error"]);
        assert_eq!(errors.properties.len(), 1);

        let mut object_then_array =
            Errors::Object(ObjectErrors::new(vec!["object error"], properties));
        object_then_array.merge(Errors::Array(ArrayErrors::new(vec!["array error"], items)));
        let Errors::Object(errors) = object_then_array else {
            panic!("object errors must take precedence")
        };
        assert_eq!(errors.errors, ["object error", "array error"]);
        assert_eq!(errors.properties.len(), 1);
    }

    #[test]
    fn array_errors_at_the_same_index_are_merged_recursively() {
        let mut errors = Errors::Array(ArrayErrors::new(
            Vec::new(),
            ItemErrorsMap::from([(0, Errors::NewType(vec!["first"]))]),
        ));
        errors.merge(Errors::Array(ArrayErrors::new(
            Vec::new(),
            ItemErrorsMap::from([(0, Errors::NewType(vec!["second"]))]),
        )));

        let Errors::Array(errors) = errors else {
            panic!("array structure must be retained")
        };
        let Errors::NewType(errors) = &errors.items[&0] else {
            panic!("item errors must remain newtype errors")
        };
        assert_eq!(errors, &["first", "second"]);
    }
}
