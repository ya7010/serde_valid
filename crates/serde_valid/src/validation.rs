mod array;
mod composited;
pub mod custom;
pub mod error;
mod generic;
mod numeric;
mod object;
mod recursive;
mod string;

pub use composited::Composited;
#[allow(deprecated)]
pub use recursive::ValidateCompositedEnumerate;
pub use recursive::{
    composited_path, ValidateCompositedEnum, ValidateCompositedExclusiveMaximum,
    ValidateCompositedExclusiveMinimum, ValidateCompositedMaxLength,
    ValidateCompositedMaxProperties, ValidateCompositedMaximum, ValidateCompositedMinLength,
    ValidateCompositedMinProperties, ValidateCompositedMinimum, ValidateCompositedMultipleOf,
    ValidateCompositedPattern,
};

pub use array::{ValidateMaxItems, ValidateMinItems, ValidateUniqueItems};
pub use error::{
    ArrayErrors, Error, Errors, IntoError, ItemErrorsMap, ItemVecErrorsMap, ObjectErrors,
    PropertyErrorsMap, PropertyVecErrorsMap, VecErrors,
};
pub use generic::ValidateEnum;
#[allow(deprecated)]
pub use generic::ValidateEnumerate;
pub use numeric::{
    ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaximum, ValidateMinimum,
    ValidateMultipleOf,
};
pub use object::{ValidateMaxProperties, ValidateMinProperties};
pub use serde_valid_literal::{Literal, Number, Pattern};
pub use string::{ValidateMaxLength, ValidateMinLength, ValidatePattern};
