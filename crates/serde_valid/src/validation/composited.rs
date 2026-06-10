use crate::validation::error::IntoError;

use crate::error::{
    EnumError, ExclusiveMaximumError, ExclusiveMinimumError, MaxItemsError, MaxLengthError,
    MaxPropertiesError, MaximumError, MinItemsError, MinLengthError, MinPropertiesError,
    MinimumError, MultipleOfError, PatternError, UniqueItemsError,
};
use indexmap::IndexMap;

/// Composited use Vec or Map error.
///
/// Composited elevates field validation errors to per-element error in the array.
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
                                        crate::validation::Errors::NewType(vec![
                                            params.into_error_by(format.clone())
                                        ]),
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
