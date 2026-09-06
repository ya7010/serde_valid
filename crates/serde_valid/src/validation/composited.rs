use crate::validation::error::IntoError;

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
pub enum Composited<Error> {
    Single(Error),
    Array(IndexMap<usize, Composited<Error>>),
    Object(IndexMap<Cow<'static, str>, Vec<Composited<Error>>>),
}

fn merge_errors(errors: Vec<crate::validation::error::Error>) -> crate::validation::error::Errors {
    let mut item_errors = vec![];
    let mut property_errors = vec![];
    let mut errors = errors
        .into_iter()
        .filter_map(|error| match error {
            crate::validation::error::Error::Items(errors) => {
                item_errors.push(errors);
                None
            }
            crate::validation::error::Error::Properties(errors) => {
                property_errors.push(errors);
                None
            }
            error => Some(error),
        })
        .collect::<Vec<_>>();

    if !property_errors.is_empty() {
        let property_errors = property_errors
            .into_iter()
            .reduce(|mut a, b| {
                a.merge(b);
                a
            })
            .unwrap();
        errors.extend(property_errors.errors);
        crate::validation::error::Errors::Object(crate::validation::error::ObjectErrors::new(
            errors,
            property_errors.properties,
        ))
    } else if !item_errors.is_empty() {
        let item_errors = item_errors.into_iter().reduce(|a, b| a.merge(b)).unwrap();
        errors.extend(item_errors.errors);
        crate::validation::error::Errors::Array(crate::validation::error::ArrayErrors::new(
            errors,
            item_errors.items,
        ))
    } else {
        crate::validation::error::Errors::NewType(errors)
    }
}

macro_rules! impl_into_error {
    ($ErrorType:ident => $Error:ident) => {
        impl IntoError<$Error> for Composited<$Error> {
            fn into_error_by(
                self,
                format: crate::validation::error::Format<$Error>,
            ) -> crate::validation::error::Error {
                match self {
                    Composited::Single(single) => {
                        crate::validation::error::Error::$ErrorType(format.into_message(single))
                    }
                    Composited::Array(array) => crate::validation::error::Error::Items(
                        crate::validation::error::ArrayErrors::new(
                            Vec::with_capacity(0),
                            array
                                .into_iter()
                                .map(|(index, params)| {
                                    (
                                        index,
                                        merge_errors(vec![params.into_error_by(format.clone())]),
                                    )
                                })
                                .collect::<IndexMap<_, _>>(),
                        ),
                    ),
                    Composited::Object(object) => crate::validation::error::Error::Properties(
                        crate::validation::error::ObjectErrors::new(
                            Vec::with_capacity(0),
                            object
                                .into_iter()
                                .map(|(property, params)| {
                                    (
                                        property,
                                        merge_errors(
                                            params
                                                .into_iter()
                                                .map(|param| param.into_error_by(format.clone()))
                                                .collect(),
                                        ),
                                    )
                                })
                                .collect::<IndexMap<_, _>>(),
                        ),
                    ),
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
