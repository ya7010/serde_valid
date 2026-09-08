// Keep dispatch trait-qualified. Method syntax would allow an inherent method on
// the field type to shadow validation, regardless of the generated method's span.
macro_rules! quote_composited_validation {
    (
        $receiver:ident, $argument:ident,
        $ValidateCompositedTrait:ident, $validate_composited_method:ident
    ) => {
        quote::quote!(
            ::serde_valid::composited::$ValidateCompositedTrait::$validate_composited_method(
                #$receiver,
                #$argument,
            )
        )
    };
}

macro_rules! quote_validation {
    (
        zero $receiver:ident,
        $ValidateTrait:ident, $validate_method:ident
    ) => {
        quote::quote!(::serde_valid::$ValidateTrait::$validate_method(#$receiver))
    };
    (
        $receiver:ident, $argument:ident,
        $ValidateTrait:ident, $validate_method:ident
    ) => {
        quote::quote!(::serde_valid::$ValidateTrait::$validate_method(#$receiver, #$argument))
    };
}

mod array;
mod field;
mod generic;
mod meta;
mod numeric;
mod object;
mod string;

pub use field::FieldValidators;
pub use meta::extract_field_validator;
