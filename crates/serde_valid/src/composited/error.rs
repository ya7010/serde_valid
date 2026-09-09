use crate::validation::error::{ArrayErrors, Errors, Format, IntoError, ObjectErrors};

use std::borrow::Cow;

use crate::error::{
    EnumError, ExclusiveMaximumError, ExclusiveMinimumError, MaxItemsError, MaxLengthError,
    MaxPropertiesError, MaximumError, MinItemsError, MinLengthError, MinPropertiesError,
    MinimumError, MultipleOfError, PatternError, UniqueItemsError,
};
use indexmap::IndexMap;

/// Represents a validation error for a scalar value, sequence, or map.
///
/// Composited elevates field validation errors to per-element or per-property errors.
///
/// # Examples
/// ```rust
/// use serde_valid::Validate;
///
/// #[derive(Validate)]
/// pub struct Data {
///     #[validate(minimum = 0)]
///     #[validate(maximum = 10)]
///     pub val: Vec<i32>, // <-- Here
/// }
/// ```
#[derive(Debug)]
#[doc(hidden)]
pub enum Composited<Error> {
    Single(Error),
    Array(IndexMap<usize, Composited<Error>>),
    Object(IndexMap<Cow<'static, str>, Vec<Composited<Error>>>),
}

type IntoSingle<E> = fn(E, Format<E>) -> crate::validation::error::Error;

impl<E> Composited<E> {
    fn into_errors(self, format: &Format<E>, into_single: IntoSingle<E>) -> Errors {
        match self {
            Composited::Single(single) => {
                Errors::NewType(vec![into_single(single, format.clone())])
            }
            Composited::Array(array) => Errors::Array(ArrayErrors::new(
                Vec::new(),
                array
                    .into_iter()
                    .map(|(index, params)| (index, params.into_errors(format, into_single)))
                    .collect(),
            )),
            Composited::Object(object) => Errors::Object(ObjectErrors::new(
                Vec::new(),
                object
                    .into_iter()
                    .map(|(property, params)| {
                        (
                            property,
                            merge_composited_errors(params, format, into_single),
                        )
                    })
                    .collect(),
            )),
        }
    }
}

fn merge_composited_errors<E>(
    params: Vec<Composited<E>>,
    format: &Format<E>,
    into_single: IntoSingle<E>,
) -> Errors {
    let mut params = params.into_iter();
    let Some(first) = params.next() else {
        return Errors::NewType(Vec::new());
    };
    let mut errors = first.into_errors(format, into_single);
    for param in params {
        errors.merge(param.into_errors(format, into_single));
    }
    errors
}

macro_rules! impl_into_error {
    ($ErrorType:ident => $Error:ident) => {
        impl IntoError<$Error> for Composited<$Error> {
            fn into_error_by(self, format: Format<$Error>) -> crate::validation::error::Error {
                let into_single: IntoSingle<$Error> = |error, format| {
                    crate::validation::error::Error::$ErrorType(format.into_message(error))
                };
                match self {
                    Composited::Single(single) => into_single(single, format),
                    composited => match composited.into_errors(&format, into_single) {
                        Errors::Array(array) => crate::validation::error::Error::Items(array),
                        Errors::Object(object) => {
                            crate::validation::error::Error::Properties(object)
                        }
                        Errors::NewType(_) => {
                            unreachable!("array and object composited errors stay nested")
                        }
                    },
                }
            }
        }
    };
}

// Global
impl_into_error!(Enum => EnumError);

// Numeric
impl_into_error!(Maximum => MaximumError);
impl_into_error!(Minimum => MinimumError);
impl_into_error!(ExclusiveMaximum => ExclusiveMaximumError);
impl_into_error!(ExclusiveMinimum => ExclusiveMinimumError);
impl_into_error!(MultipleOf => MultipleOfError);

// String
impl_into_error!(MaxLength => MaxLengthError);
impl_into_error!(MinLength => MinLengthError);
impl_into_error!(Pattern => PatternError);

// Array
impl_into_error!(MaxItems => MaxItemsError);
impl_into_error!(MinItems => MinItemsError);
impl_into_error!(UniqueItems => UniqueItemsError);

// Object
impl_into_error!(MaxProperties => MaxPropertiesError);
impl_into_error!(MinProperties => MinPropertiesError);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::validation::error::{Error, Errors, IntoError};

    #[test]
    fn array_of_singles_becomes_item_newtype_errors() {
        let composited = Composited::Array(IndexMap::from([(
            0,
            Composited::Single(MinimumError::new(1)),
        )]));

        let Error::Items(array) = composited.into_error() else {
            panic!("array composited must become item errors");
        };
        assert!(array.errors.is_empty());
        let Errors::NewType(errors) = &array.items[&0] else {
            panic!("scalar items must become newtype errors");
        };
        assert_eq!(errors.len(), 1);
    }

    #[test]
    fn nested_array_stays_array_errors() {
        let composited = Composited::Array(IndexMap::from([(
            0,
            Composited::Array(IndexMap::from([(
                1,
                Composited::Single(MinimumError::new(1)),
            )])),
        )]));

        let Error::Items(array) = composited.into_error() else {
            panic!("array composited must become item errors");
        };
        let Errors::Array(inner) = &array.items[&0] else {
            panic!("nested array must remain array errors");
        };
        assert!(matches!(&inner.items[&1], Errors::NewType(_)));
    }

    #[test]
    fn object_values_merge_without_wrapping_through_error() {
        let composited = Composited::Object(IndexMap::from([(
            "k".into(),
            vec![
                Composited::Single(MinimumError::new(1)),
                Composited::Single(MinimumError::new(2)),
            ],
        )]));

        let Error::Properties(object) = composited.into_error() else {
            panic!("object composited must become property errors");
        };
        let Errors::NewType(errors) = &object.properties["k"] else {
            panic!("scalar properties must become newtype errors");
        };
        assert_eq!(errors.len(), 2);
    }

    #[test]
    fn mixed_object_values_keep_object_precedence() {
        let composited = Composited::Object(IndexMap::from([(
            "k".into(),
            vec![
                Composited::Array(IndexMap::from([(
                    0,
                    Composited::Single(MinimumError::new(1)),
                )])),
                Composited::Object(IndexMap::from([(
                    "property".into(),
                    vec![Composited::Single(MinimumError::new(2))],
                )])),
            ],
        )]));

        let Error::Properties(object) = composited.into_error() else {
            panic!("object composited must become property errors");
        };
        let Errors::Object(inner) = &object.properties["k"] else {
            panic!("mixed array and object values must keep object precedence");
        };
        assert!(inner.errors.is_empty());
        assert!(matches!(&inner.properties["property"], Errors::NewType(_)));
    }
}
