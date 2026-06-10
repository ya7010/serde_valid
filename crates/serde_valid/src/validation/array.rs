mod max_items;
mod min_items;
mod unique_items;

pub use max_items::ValidateMaxItems;
pub use min_items::ValidateMinItems;
pub use unique_items::ValidateUniqueItems;

use crate::{MaxItemsError, MinItemsError};

macro_rules! impl_validate_array_length_items {
    ($ValidateTrait:ident, $validate_method:ident, $Error:ident) => {
        impl<T> $ValidateTrait for Option<T>
        where
            T: $ValidateTrait,
        {
            fn $validate_method(&self, limit: usize) -> Result<(), $Error> {
                match self {
                    Some(value) => value.$validate_method(limit),
                    None => Ok(()),
                }
            }
        }
    };
}

impl_validate_array_length_items!(ValidateMaxItems, validate_max_items, MaxItemsError);
impl_validate_array_length_items!(ValidateMinItems, validate_min_items, MinItemsError);
