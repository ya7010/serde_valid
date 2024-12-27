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

impl<E> Errors<E>
where
    E: Clone,
{
    pub fn merge(&mut self, other: Errors<E>) {
        match self {
            Errors::Array(a) => match other {
                Errors::Array(b) => {
                    a.errors.extend(b.errors);

                    for (index, item) in b.items {
                        match a.items.get_mut(&index) {
                            Some(errors) => errors.merge(item),
                            None => {
                                a.items.insert(index, item);
                            }
                        };
                    }
                }
                Errors::Object(_) => {
                    unreachable!("conflict Array and Object in serde_valid::validation::Errors")
                }
                Errors::NewType(errors) => {
                    a.errors.extend(errors);
                }
            },
            Errors::NewType(a) => match other {
                Errors::Array(b) => {
                    a.extend(b.errors);
                    *self = Errors::Array(ArrayErrors::new(a.to_vec(), b.items));
                }
                Errors::Object(_) => {
                    unreachable!("conflict Array and Object in serde_valid::validation::Errors")
                }
                Errors::NewType(b) => {
                    a.extend(b);
                }
            },
            Errors::Object(_) => {
                unimplemented!("Object does not support yet.")
            }
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
