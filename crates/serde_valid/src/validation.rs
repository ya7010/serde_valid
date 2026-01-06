mod array;
mod composited;
pub mod custom;
pub mod error;
mod generic;
mod numeric;
mod object;
mod string;

use crate::{
    EnumerateError, ExclusiveMaximumError, ExclusiveMinimumError, MaxLengthError,
    MaxPropertiesError, MaximumError, MinLengthError, MinPropertiesError, MinimumError,
    MultipleOfError, PatternError,
};
pub use composited::Composited;

pub use array::{ValidateMaxItems, ValidateMinItems, ValidateUniqueItems};
pub use error::{
    ArrayErrors, Error, Errors, IntoError, ItemErrorsMap, ItemVecErrorsMap, ObjectErrors,
    PropertyErrorsMap, PropertyVecErrorsMap, VecErrors,
};
pub use generic::{ValidateEnum, ValidateEnumerate};
use indexmap::IndexMap;
pub use numeric::{
    ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaximum, ValidateMinimum,
    ValidateMultipleOf,
};
pub use object::{ValidateMaxProperties, ValidateMinProperties};
pub use serde_valid_literal::{Literal, Number, Pattern};
pub use string::{ValidateMaxLength, ValidateMinLength, ValidatePattern};

macro_rules! impl_composited_validation_1args {
    (
        pub trait $ValidateCompositedTrait:ident {
            fn $validate_composited_method:ident(
                &self,
                $limit:ident: $limit_type:ty$(,)*
            ) -> Result<(), Composited<$Error:ty>>;
        }
    ) => {
        paste::paste! {
            pub trait $ValidateCompositedTrait {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type
                ) -> Result<(), Composited<$Error>>;
            }

            impl<T> $ValidateCompositedTrait for T
            where
                T: [<Validate $limit:camel>],
            {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$Error>> {
                    self.[<validate_ $limit>]($limit)
                        .map_err(|error| Composited::Single(error))
                }
            }

            impl<T> $ValidateCompositedTrait for Vec<T>
            where
                T: $ValidateCompositedTrait,
            {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$Error>> {
                    let errors: IndexMap<usize, crate::validation::Composited<$Error>> = self
                        .iter()
                        .enumerate()
                        .filter_map(
                            |(index, item)| match item.$validate_composited_method($limit) {
                                Ok(_) => None,
                                Err(error) => Some((index, error)),
                            },
                        )
                        .collect();

                    if errors.is_empty() {
                        Ok(())
                    } else {
                        Err(Composited::Array(errors))
                    }
                }
            }

            impl<T, const N: usize> $ValidateCompositedTrait for [T; N]
            where
                T: $ValidateCompositedTrait,
            {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$Error>> {
                    let errors: IndexMap<usize, crate::validation::Composited<$Error>> = self
                        .iter()
                        .enumerate()
                        .filter_map(
                            |(index, item)| match item.$validate_composited_method($limit) {
                                Ok(_) => None,
                                Err(error) => Some((index, error)),
                            },
                        )
                        .collect();

                    if errors.is_empty() {
                        Ok(())
                    } else {
                        Err(Composited::Array(errors))
                    }
                }
            }

            impl<T> $ValidateCompositedTrait for Option<T>
            where
                T: $ValidateCompositedTrait,
            {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$Error>> {
                    match self {
                        Some(value) => value.$validate_composited_method($limit),
                        None => Ok(()),
                    }
                }
            }
        }
    };
    (
        pub trait $ValidateCompositedTrait:ident {
            fn $validate_composited_method:ident(
                &self,
                $limit:ident: $limit_type:ty$(,)*
            ) -> Result<(), Composited<$Error:ty>>;
        }

        impl<K, V> $ValidateCompositedTrait2:ident for std::collections::HashMap<K, V>
        where
            V: $ValidateCompositedTrait3:ident;
    ) => {
        impl_composited_validation_1args!(
            pub trait $ValidateCompositedTrait {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type
                ) -> Result<(), Composited<$Error>>;
            }
        );
        paste::paste! {
            impl<K, V> $ValidateCompositedTrait2 for std::collections::HashMap<K, V>
            where
                V: $ValidateCompositedTrait3,
            {
                fn $validate_composited_method(
                    &self,
                    $limit: $limit_type,
                ) -> Result<(), Composited<$Error>> {
                    let errors: IndexMap<usize, crate::validation::Composited<$Error>> = self
                        .iter()
                        .enumerate()
                        .filter_map(
                            |(index, (_key, value))| match value.$validate_composited_method($limit) {
                                Ok(_) => None,
                                Err(error) => Some((index, error)),
                            },
                        )
                        .collect();

                    if errors.is_empty() {
                        Ok(())
                    } else {
                        Err(Composited::Array(errors))
                    }
                }
            }
        }
    };
    (
        pub trait $ValidateCompositedTrait:ident<T> {
            fn $validate_composited_method:ident(
                &self,
                $limit:ident: T$(,)*
            ) -> Result<(), Composited<$Error:ty>>;
        }
    ) => {
        pub trait $ValidateCompositedTrait<T> {
            fn $validate_composited_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Composited<$Error>>;
        }

        impl<T, U> $ValidateCompositedTrait<T> for Vec<U>
        where
            T: Copy,
            U: $ValidateCompositedTrait<T>,
        {
            fn $validate_composited_method(
                &self,
                $limit: T,
            ) -> Result<(), crate::validation::Composited<$Error>> {
                let errors: IndexMap<usize, crate::validation::Composited<$Error>> = self
                    .iter()
                    .enumerate()
                    .filter_map(
                        |(index, item)| match item.$validate_composited_method($limit) {
                            Ok(_) => None,
                            Err(error) => Some((index, error)),
                        },
                    )
                    .collect();

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Composited::Array(errors))
                }
            }
        }

        impl<T, K, V> $ValidateCompositedTrait<T> for std::collections::HashMap<K, V>
        where
            T: Copy,
            V: $ValidateCompositedTrait<T>,
        {
            fn $validate_composited_method(&self, $limit: T) -> Result<(), Composited<$Error>> {
                let errors: IndexMap<usize, crate::validation::Composited<$Error>> = self
                    .iter()
                    .enumerate()
                    .filter_map(|(index, (_key, value))| {
                        match value.$validate_composited_method($limit) {
                            Ok(_) => None,
                            Err(error) => Some((index, error)),
                        }
                    })
                    .collect();

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Composited::Array(errors))
                }
            }
        }

        impl<T, U, const N: usize> $ValidateCompositedTrait<T> for [U; N]
        where
            T: Copy,
            U: $ValidateCompositedTrait<T>,
        {
            fn $validate_composited_method(
                &self,
                $limit: T,
            ) -> Result<(), crate::validation::Composited<$Error>> {
                let errors: IndexMap<usize, crate::validation::Composited<$Error>> = self
                    .iter()
                    .enumerate()
                    .filter_map(
                        |(index, item)| match item.$validate_composited_method($limit) {
                            Ok(_) => None,
                            Err(error) => Some((index, error)),
                        },
                    )
                    .collect();

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Composited::Array(errors))
                }
            }
        }

        impl<T, U> $ValidateCompositedTrait<T> for Option<U>
        where
            T: Copy,
            U: $ValidateCompositedTrait<T>,
        {
            fn $validate_composited_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Composited<$Error>> {
                match self {
                    Some(value) => value.$validate_composited_method(limit),
                    None => Ok(()),
                }
            }
        }
    };
}

macro_rules! impl_generic_composited_validation_1args {
    (
        $ErrorType:ident,
        $type:ty
    ) => {
        paste::paste! {
            impl<T> [<ValidateComposited $ErrorType >]<$type> for T
            where
                T: [<Validate $ErrorType >]<$type>,
            {
                fn [< validate_composited_ $ErrorType:snake>](
                    &self,
                    limit: $type,
                ) -> Result<(), crate::validation::Composited<[<$ErrorType Error>]>> {
                    self.[< validate_ $ErrorType:snake>](limit)
                        .map_err(|error| crate::validation::Composited::Single(error))
                }
            }
        }
    };
}

pub(crate) use impl_generic_composited_validation_1args;

// Number
impl_composited_validation_1args!(
    pub trait ValidateCompositedMaximum<T> {
        fn validate_composited_maximum(&self, maximum: T) -> Result<(), Composited<MaximumError>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedMinimum<T> {
        fn validate_composited_minimum(&self, minimum: T) -> Result<(), Composited<MinimumError>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedExclusiveMaximum<T> {
        fn validate_composited_exclusive_maximum(
            &self,
            exclusive_maximum: T,
        ) -> Result<(), Composited<ExclusiveMaximumError>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedExclusiveMinimum<T> {
        fn validate_composited_exclusive_minimum(
            &self,
            exclusive_minimum: T,
        ) -> Result<(), Composited<ExclusiveMinimumError>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedMultipleOf<T> {
        fn validate_composited_multiple_of(
            &self,
            exclusive_minimum: T,
        ) -> Result<(), Composited<MultipleOfError>>;
    }
);

// String
impl_composited_validation_1args!(
    pub trait ValidateCompositedMaxLength {
        fn validate_composited_max_length(
            &self,
            max_length: usize,
        ) -> Result<(), Composited<MaxLengthError>>;
    }

    impl<K, V> ValidateCompositedMaxLength for std::collections::HashMap<K, V>
    where
        V: ValidateCompositedMaxLength;
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedMinLength {
        fn validate_composited_min_length(
            &self,
            min_length: usize,
        ) -> Result<(), Composited<MinLengthError>>;
    }

    impl<K, V> ValidateCompositedMinLength for std::collections::HashMap<K, V>
    where
        V: ValidateCompositedMinLength;
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedPattern {
        fn validate_composited_pattern(
            &self,
            pattern: &regex::Regex,
        ) -> Result<(), Composited<PatternError>>;
    }

    impl<K, V> ValidateCompositedPattern for std::collections::HashMap<K, V>
    where
        V: ValidateCompositedPattern;
);

// Object
impl_composited_validation_1args!(
    pub trait ValidateCompositedMaxProperties {
        fn validate_composited_max_properties(
            &self,
            max_properties: usize,
        ) -> Result<(), Composited<MaxPropertiesError>>;
    }
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedMinProperties {
        fn validate_composited_min_properties(
            &self,
            min_properties: usize,
        ) -> Result<(), Composited<MinPropertiesError>>;
    }
);

// Generic
impl_composited_validation_1args!(
    pub trait ValidateCompositedEnum<T> {
        fn validate_composited_enum(&self, enumerate: T) -> Result<(), Composited<EnumerateError>>;
    }
);
