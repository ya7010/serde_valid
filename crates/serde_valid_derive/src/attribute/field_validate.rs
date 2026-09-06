// Method syntax is used only for selecting an autoderef receiver. The generated
// helper names live in serde_valid's internal namespace, and each helper uses
// trait-qualified dispatch once the receiver has been selected.
macro_rules! quote_composited_autoderef {
    (
        generic $receiver:ident, $argument:ident,
        $ValidateCompositedTrait:ident, $validate_composited_method:ident,
        $autoderef_method:ident, $Error:ident
    ) => {
        quote::quote!({
            trait __SerdeValidCompositedAutoderef<__Argument> {
                fn $autoderef_method(
                    &self,
                    argument: __Argument,
                ) -> ::std::result::Result<
                    (),
                    ::serde_valid::validation::Composited<::serde_valid::$Error>,
                >;
            }

            impl<__Receiver, __Argument> __SerdeValidCompositedAutoderef<__Argument> for __Receiver
            where
                __Receiver: ::serde_valid::validation::$ValidateCompositedTrait<__Argument>
                    + ?::std::marker::Sized,
            {
                fn $autoderef_method(
                    &self,
                    argument: __Argument,
                ) -> ::std::result::Result<
                    (),
                    ::serde_valid::validation::Composited<::serde_valid::$Error>,
                > {
                    ::serde_valid::validation::$ValidateCompositedTrait::$validate_composited_method(
                        self,
                        argument,
                    )
                }
            }

            (#$receiver).$autoderef_method(#$argument)
        })
    };
    (
        fixed $receiver:ident, $argument:ident, $Argument:ty,
        $ValidateCompositedTrait:ident, $validate_composited_method:ident,
        $autoderef_method:ident, $Error:ident
    ) => {
        quote::quote!({
            trait __SerdeValidCompositedAutoderef {
                fn $autoderef_method(
                    &self,
                    argument: $Argument,
                ) -> ::std::result::Result<
                    (),
                    ::serde_valid::validation::Composited<::serde_valid::$Error>,
                >;
            }

            impl<__Receiver> __SerdeValidCompositedAutoderef for __Receiver
            where
                __Receiver: ::serde_valid::validation::$ValidateCompositedTrait
                    + ?::std::marker::Sized,
            {
                fn $autoderef_method(
                    &self,
                    argument: $Argument,
                ) -> ::std::result::Result<
                    (),
                    ::serde_valid::validation::Composited<::serde_valid::$Error>,
                > {
                    ::serde_valid::validation::$ValidateCompositedTrait::$validate_composited_method(
                        self,
                        argument,
                    )
                }
            }

            (#$receiver).$autoderef_method(#$argument)
        })
    };
    (
        slice $receiver:ident, $candidates:ident,
        $ValidateCompositedTrait:ident, $validate_composited_method:ident,
        $autoderef_method:ident, $Error:ident
    ) => {
        quote::quote!({
            trait __SerdeValidCompositedAutoderef<'__candidate, __Candidate> {
                fn $autoderef_method(
                    &self,
                    candidates: &'__candidate [__Candidate],
                ) -> ::std::result::Result<
                    (),
                    ::serde_valid::validation::Composited<::serde_valid::$Error>,
                >;
            }

            impl<'__candidate, __Receiver, __Candidate>
                __SerdeValidCompositedAutoderef<'__candidate, __Candidate> for __Receiver
            where
                __Candidate: '__candidate,
                __Receiver: ::serde_valid::validation::$ValidateCompositedTrait<
                        &'__candidate [__Candidate],
                    > + ?::std::marker::Sized,
            {
                fn $autoderef_method(
                    &self,
                    candidates: &'__candidate [__Candidate],
                ) -> ::std::result::Result<
                    (),
                    ::serde_valid::validation::Composited<::serde_valid::$Error>,
                > {
                    ::serde_valid::validation::$ValidateCompositedTrait::$validate_composited_method(
                        self,
                        candidates,
                    )
                }
            }

            (#$receiver).$autoderef_method(&[#$candidates])
        })
    };
}

macro_rules! quote_validation_autoderef {
    (
        zero $receiver:ident,
        $ValidateTrait:ident, $validate_method:ident,
        $autoderef_method:ident, $Error:ty
    ) => {
        quote::quote!({
            trait __SerdeValidAutoderef {
                fn $autoderef_method(&self) -> ::std::result::Result<(), $Error>;
            }

            impl<__Receiver> __SerdeValidAutoderef for __Receiver
            where
                __Receiver: ::serde_valid::$ValidateTrait + ?::std::marker::Sized,
            {
                fn $autoderef_method(&self) -> ::std::result::Result<(), $Error> {
                    ::serde_valid::$ValidateTrait::$validate_method(self)
                }
            }

            (#$receiver).$autoderef_method()
        })
    };
    (
        fixed $receiver:ident, $argument:ident, $Argument:ty,
        $ValidateTrait:ident, $validate_method:ident,
        $autoderef_method:ident, $Error:ty
    ) => {
        quote::quote!({
            trait __SerdeValidAutoderef {
                fn $autoderef_method(
                    &self,
                    argument: $Argument,
                ) -> ::std::result::Result<(), $Error>;
            }

            impl<__Receiver> __SerdeValidAutoderef for __Receiver
            where
                __Receiver: ::serde_valid::$ValidateTrait + ?::std::marker::Sized,
            {
                fn $autoderef_method(
                    &self,
                    argument: $Argument,
                ) -> ::std::result::Result<(), $Error> {
                    ::serde_valid::$ValidateTrait::$validate_method(self, argument)
                }
            }

            (#$receiver).$autoderef_method(#$argument)
        })
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
