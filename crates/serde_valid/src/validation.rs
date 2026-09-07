mod array;
mod composited;
pub mod custom;
pub mod error;
mod generic;
mod numeric;
mod object;
mod string;

use crate::{
    EnumError, ExclusiveMaximumError, ExclusiveMinimumError, MaxLengthError, MaxPropertiesError,
    MaximumError, MinLengthError, MinPropertiesError, MinimumError, MultipleOfError, PatternError,
};
pub use composited::Composited;

pub use array::{ValidateMaxItems, ValidateMinItems, ValidateUniqueItems};
pub use error::{
    ArrayErrors, Error, Errors, IntoError, ItemErrorsMap, ItemVecErrorsMap, ObjectErrors,
    PropertyErrorsMap, PropertyVecErrorsMap, VecErrors,
};
pub use generic::ValidateEnum;
#[allow(deprecated)]
pub use generic::ValidateEnumerate;
use indexmap::IndexMap;
pub use numeric::{
    ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaximum, ValidateMinimum,
    ValidateMultipleOf,
};
pub use object::{ValidateMaxProperties, ValidateMinProperties};
pub use serde_valid_literal::{Literal, Number, Pattern};
pub use string::{ValidateMaxLength, ValidateMinLength, ValidatePattern};

macro_rules! impl_composited_wrappers_1args {
    ($ValidateCompositedTrait:ident, $validate_composited_method:ident, $limit_type:ty, $Error:ty) => {
        impl<T> $ValidateCompositedTrait for &T
        where
            T: $ValidateCompositedTrait + ?Sized,
        {
            fn $validate_composited_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(*self, limit)
            }
        }

        impl<T> $ValidateCompositedTrait for Box<T>
        where
            T: $ValidateCompositedTrait + ?Sized,
        {
            fn $validate_composited_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(self.as_ref(), limit)
            }
        }

        impl<T> $ValidateCompositedTrait for std::rc::Rc<T>
        where
            T: $ValidateCompositedTrait + ?Sized,
        {
            fn $validate_composited_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(self.as_ref(), limit)
            }
        }

        impl<T> $ValidateCompositedTrait for std::sync::Arc<T>
        where
            T: $ValidateCompositedTrait + ?Sized,
        {
            fn $validate_composited_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(self.as_ref(), limit)
            }
        }

        impl<P> $ValidateCompositedTrait for std::pin::Pin<P>
        where
            P: std::ops::Deref,
            P::Target: $ValidateCompositedTrait,
        {
            fn $validate_composited_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(
                    self.as_ref().get_ref(),
                    limit,
                )
            }
        }

        impl<T> $ValidateCompositedTrait for std::borrow::Cow<'_, T>
        where
            T: std::borrow::ToOwned + $ValidateCompositedTrait + ?Sized,
        {
            fn $validate_composited_method(
                &self,
                limit: $limit_type,
            ) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(self.as_ref(), limit)
            }
        }
    };
}

macro_rules! impl_generic_composited_wrappers_1args {
    ($ValidateCompositedTrait:ident, $validate_composited_method:ident, $Error:ty) => {
        #[allow(deprecated)]
        impl<C, T> $ValidateCompositedTrait<C> for &T
        where
            T: $ValidateCompositedTrait<C> + ?Sized,
        {
            fn $validate_composited_method(&self, limit: C) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(*self, limit)
            }
        }

        #[allow(deprecated)]
        impl<C, T> $ValidateCompositedTrait<C> for Box<T>
        where
            T: $ValidateCompositedTrait<C> + ?Sized,
        {
            fn $validate_composited_method(&self, limit: C) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(self.as_ref(), limit)
            }
        }

        #[allow(deprecated)]
        impl<C, T> $ValidateCompositedTrait<C> for std::rc::Rc<T>
        where
            T: $ValidateCompositedTrait<C> + ?Sized,
        {
            fn $validate_composited_method(&self, limit: C) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(self.as_ref(), limit)
            }
        }

        #[allow(deprecated)]
        impl<C, T> $ValidateCompositedTrait<C> for std::sync::Arc<T>
        where
            T: $ValidateCompositedTrait<C> + ?Sized,
        {
            fn $validate_composited_method(&self, limit: C) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(self.as_ref(), limit)
            }
        }

        #[allow(deprecated)]
        impl<C, P> $ValidateCompositedTrait<C> for std::pin::Pin<P>
        where
            P: std::ops::Deref,
            P::Target: $ValidateCompositedTrait<C>,
        {
            fn $validate_composited_method(&self, limit: C) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(
                    self.as_ref().get_ref(),
                    limit,
                )
            }
        }

        #[allow(deprecated)]
        impl<C, T> $ValidateCompositedTrait<C> for std::borrow::Cow<'_, T>
        where
            T: std::borrow::ToOwned + $ValidateCompositedTrait<C> + ?Sized,
        {
            fn $validate_composited_method(&self, limit: C) -> Result<(), Composited<$Error>> {
                $ValidateCompositedTrait::$validate_composited_method(self.as_ref(), limit)
            }
        }
    };
}

macro_rules! impl_composited_validation_1args {
    (
        pub trait $ValidateCompositedTrait:ident {
            fn $validate_composited_method:ident(
                &self,
                $limit:ident: $limit_type:ty$(,)*
            ) -> Result<(), Composited<$Error:ty>>;
        }
    ) => {
        pub trait $ValidateCompositedTrait {
            fn $validate_composited_method(
                &self,
                $limit: $limit_type
            ) -> Result<(), Composited<$Error>>;
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

        impl<T> $ValidateCompositedTrait for [T]
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

        impl_composited_wrappers_1args!(
            $ValidateCompositedTrait,
            $validate_composited_method,
            $limit_type,
            $Error
        );

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

        impl<K, V> $ValidateCompositedTrait2 for std::collections::HashMap<K, V>
        where
            K: AsRef<str>,
            V: $ValidateCompositedTrait3,
        {
            fn $validate_composited_method(
                &self,
                $limit: $limit_type,
            ) -> Result<(), Composited<$Error>> {
                let mut errors = IndexMap::new();
                for (key, value) in self {
                    if let Err(error) = value.$validate_composited_method($limit) {
                        errors
                            .entry(std::borrow::Cow::Owned(key.as_ref().to_owned()))
                            .or_insert_with(Vec::new)
                            .push(error);
                    }
                }

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Composited::Object(errors))
                }
            }
        }

        impl<K, V> $ValidateCompositedTrait2 for indexmap::IndexMap<K, V>
        where
            K: AsRef<str>,
            V: $ValidateCompositedTrait3,
        {
            fn $validate_composited_method(
                &self,
                $limit: $limit_type,
            ) -> Result<(), Composited<$Error>> {
                let mut errors = IndexMap::new();
                for (key, value) in self {
                    if let Err(error) = value.$validate_composited_method($limit) {
                        errors
                            .entry(std::borrow::Cow::Owned(key.as_ref().to_owned()))
                            .or_insert_with(Vec::new)
                            .push(error);
                    }
                }

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Composited::Object(errors))
                }
            }
        }
    };
    (
        $(#[$trait_meta:meta])+
        pub trait $ValidateCompositedTrait:ident<T> {
            fn $validate_composited_method:ident(
                &self,
                $limit:ident: T$(,)*
            ) -> Result<(), Composited<$Error:ty>>;
        }
    ) => {
        impl_composited_validation_1args!(
            @generic
            [#[allow(deprecated)] $(#[$trait_meta])*]
            [#[allow(deprecated)]]
            pub trait $ValidateCompositedTrait<T> {
                fn $validate_composited_method(
                    &self,
                    $limit: T,
                ) -> Result<(), Composited<$Error>>;
            }
        );
    };
    (
        pub trait $ValidateCompositedTrait:ident<T> {
            fn $validate_composited_method:ident(
                &self,
                $limit:ident: T$(,)*
            ) -> Result<(), Composited<$Error:ty>>;
        }
    ) => {
        impl_composited_validation_1args!(
            @generic
            []
            []
            pub trait $ValidateCompositedTrait<T> {
                fn $validate_composited_method(
                    &self,
                    $limit: T,
                ) -> Result<(), Composited<$Error>>;
            }
        );
    };
    (
        @generic
        [$(#[$trait_meta:meta])*]
        [$(#[$impl_meta:meta])*]
        pub trait $ValidateCompositedTrait:ident<T> {
            fn $validate_composited_method:ident(
                &self,
                $limit:ident: T$(,)*
            ) -> Result<(), Composited<$Error:ty>>;
        }
    ) => {
        $(#[$trait_meta])*
        pub trait $ValidateCompositedTrait<T> {
            fn $validate_composited_method(
                &self,
                limit: T,
            ) -> Result<(), crate::validation::Composited<$Error>>;
        }

        $(#[$impl_meta])*
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

        $(#[$impl_meta])*
        impl<T, K, V> $ValidateCompositedTrait<T> for std::collections::HashMap<K, V>
        where
            T: Copy,
            K: AsRef<str>,
            V: $ValidateCompositedTrait<T>,
        {
            fn $validate_composited_method(&self, $limit: T) -> Result<(), Composited<$Error>> {
                let mut errors = IndexMap::new();
                for (key, value) in self {
                    if let Err(error) = value.$validate_composited_method($limit) {
                        errors
                            .entry(std::borrow::Cow::Owned(key.as_ref().to_owned()))
                            .or_insert_with(Vec::new)
                            .push(error);
                    }
                }

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Composited::Object(errors))
                }
            }
        }

        $(#[$impl_meta])*
        impl<T, K, V> $ValidateCompositedTrait<T> for indexmap::IndexMap<K, V>
        where
            T: Copy,
            K: AsRef<str>,
            V: $ValidateCompositedTrait<T>,
        {
            fn $validate_composited_method(&self, $limit: T) -> Result<(), Composited<$Error>> {
                let mut errors = IndexMap::new();
                for (key, value) in self {
                    if let Err(error) = value.$validate_composited_method($limit) {
                        errors
                            .entry(std::borrow::Cow::Owned(key.as_ref().to_owned()))
                            .or_insert_with(Vec::new)
                            .push(error);
                    }
                }

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Composited::Object(errors))
                }
            }
        }

        $(#[$impl_meta])*
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

        $(#[$impl_meta])*
        impl<T, U> $ValidateCompositedTrait<T> for [U]
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

        impl_generic_composited_wrappers_1args!(
            $ValidateCompositedTrait,
            $validate_composited_method,
            $Error
        );

        $(#[$impl_meta])*
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
        $ValidateCompositedTrait:ident,
        $validate_composited_method:ident,
        $ValidateTrait:ident,
        $validate_method:ident,
        $Error:ident,
        $type:ty
    ) => {
        impl $ValidateCompositedTrait<$type> for $type {
            fn $validate_composited_method(
                &self,
                limit: $type,
            ) -> Result<(), crate::validation::Composited<$Error>> {
                <$type as $ValidateTrait<$type>>::$validate_method(self, limit)
                    .map_err(|error| crate::validation::Composited::Single(error))
            }
        }
    };
}

pub(crate) use impl_generic_composited_validation_1args;

// Scalar leaves bridge their public validator into the recursive composited
// validator. Transparent wrappers are handled generically above.
macro_rules! impl_composited_scalar_leaves_1args {
    (
        $ValidateCompositedTrait:ident::$validate_composited_method:ident,
        $ValidateTrait:ident::$validate_method:ident,
        $limit_type:ty,
        $Error:ty,
        [$($type:ty),+ $(,)?]
    ) => {
        $(
            impl $ValidateCompositedTrait for $type {
                fn $validate_composited_method(
                    &self,
                    limit: $limit_type,
                ) -> Result<(), Composited<$Error>> {
                    <$type as $ValidateTrait>::$validate_method(self, limit)
                        .map_err(Composited::Single)
                }
            }
        )+
    };
}

macro_rules! impl_composited_object_leaves {
    (
        $ValidateCompositedTrait:ident::$validate_composited_method:ident,
        $ValidateTrait:ident::$validate_method:ident,
        $Error:ty
    ) => {
        impl<K, V> $ValidateCompositedTrait for std::collections::HashMap<K, V> {
            fn $validate_composited_method(&self, limit: usize) -> Result<(), Composited<$Error>> {
                $ValidateTrait::$validate_method(self, limit).map_err(Composited::Single)
            }
        }

        impl<K, V> $ValidateCompositedTrait for std::collections::BTreeMap<K, V> {
            fn $validate_composited_method(&self, limit: usize) -> Result<(), Composited<$Error>> {
                $ValidateTrait::$validate_method(self, limit).map_err(Composited::Single)
            }
        }

        impl<K, V> $ValidateCompositedTrait for indexmap::IndexMap<K, V> {
            fn $validate_composited_method(&self, limit: usize) -> Result<(), Composited<$Error>> {
                $ValidateTrait::$validate_method(self, limit).map_err(Composited::Single)
            }
        }

        impl $ValidateCompositedTrait for serde_json::Map<String, serde_json::Value> {
            fn $validate_composited_method(&self, limit: usize) -> Result<(), Composited<$Error>> {
                $ValidateTrait::$validate_method(self, limit).map_err(Composited::Single)
            }
        }
    };
}

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

impl_composited_scalar_leaves_1args!(
    ValidateCompositedMaxLength::validate_composited_max_length,
    ValidateMaxLength::validate_max_length,
    usize,
    MaxLengthError,
    [
        str,
        String,
        std::ffi::OsStr,
        std::ffi::OsString,
        std::path::Path,
        std::path::PathBuf
    ]
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

impl_composited_scalar_leaves_1args!(
    ValidateCompositedMinLength::validate_composited_min_length,
    ValidateMinLength::validate_min_length,
    usize,
    MinLengthError,
    [
        str,
        String,
        std::ffi::OsStr,
        std::ffi::OsString,
        std::path::Path,
        std::path::PathBuf
    ]
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

impl_composited_scalar_leaves_1args!(
    ValidateCompositedPattern::validate_composited_pattern,
    ValidatePattern::validate_pattern,
    &regex::Regex,
    PatternError,
    [
        str,
        String,
        std::ffi::OsStr,
        std::ffi::OsString,
        std::path::Path,
        std::path::PathBuf
    ]
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

impl_composited_object_leaves!(
    ValidateCompositedMaxProperties::validate_composited_max_properties,
    ValidateMaxProperties::validate_max_properties,
    MaxPropertiesError
);

impl_composited_validation_1args!(
    pub trait ValidateCompositedMinProperties {
        fn validate_composited_min_properties(
            &self,
            min_properties: usize,
        ) -> Result<(), Composited<MinPropertiesError>>;
    }
);

impl_composited_object_leaves!(
    ValidateCompositedMinProperties::validate_composited_min_properties,
    ValidateMinProperties::validate_min_properties,
    MinPropertiesError
);

// Generic
impl_composited_validation_1args!(
    pub trait ValidateCompositedEnum<T> {
        fn validate_composited_enum(&self, candidates: T) -> Result<(), Composited<EnumError>>;
    }
);

impl_composited_validation_1args!(
    #[deprecated(
        since = "2.0.2",
        note = "use `ValidateCompositedEnum` and `validate_composited_enum` instead"
    )]
    pub trait ValidateCompositedEnumerate<T> {
        fn validate_composited_enumerate(&self, enumerate: T) -> Result<(), Composited<EnumError>>;
    }
);
