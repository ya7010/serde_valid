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
    ($ErrorType:ident) => {
        paste::paste! {
            impl IntoError<[<$ErrorType Error>]> for Composited<[<$ErrorType Error>]> {
                fn into_error_by(self, format: crate::validation::error::Format<[<$ErrorType Error>]>) -> crate::validation::error::Error {
                    match self {
                        Composited::Single(single) => {
                            crate::validation::error::Error::$ErrorType(format.into_message(single))
                        },
                        Composited::Array(array) =>{
                            crate::validation::error::Error::Items(crate::validation::error::ArrayErrors::new(
                            Vec::with_capacity(0),
                            array
                                .into_iter()
                                .map(|(index, params)| {
                                    (index, crate::validation::Errors::NewType(vec![params.into_error_by(format.clone())]))
                                })
                                .collect::<IndexMap<_, _>>(),
                        ))},
                    }
                }
            }
        }
    };
}

// Global
impl_into_error!(Enum);

// Numeric
impl_into_error!(Maximum);
impl_into_error!(Minimum);
impl_into_error!(ExclusiveMaximum);
impl_into_error!(ExclusiveMinimum);
impl_into_error!(MultipleOf);

// String
impl_into_error!(MaxLength);
impl_into_error!(MinLength);
impl_into_error!(Pattern);

// Array
impl_into_error!(MaxItems);
impl_into_error!(MinItems);
impl_into_error!(UniqueItems);

// Object
impl_into_error!(MaxProperties);
impl_into_error!(MinProperties);
