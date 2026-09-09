use super::{error::Composited, path as composited_path};
use crate::traits::Sequence;
use crate::validation::{
    ValidateEnum, ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaxLength,
    ValidateMaxProperties, ValidateMaximum, ValidateMinLength, ValidateMinProperties,
    ValidateMinimum, ValidateMultipleOf, ValidatePattern,
};
use crate::{
    EnumError, ExclusiveMaximumError, ExclusiveMinimumError, MaxLengthError, MaxPropertiesError,
    MaximumError, MinLengthError, MinPropertiesError, MinimumError, MultipleOfError, PatternError,
};

fn collect_sequence<S, E, F>(sequence: &S, mut validate: F) -> Result<(), Composited<E>>
where
    S: Sequence + ?Sized,
    F: FnMut(&S::Item) -> Result<(), Composited<E>>,
{
    let mut errors = indexmap::IndexMap::new();
    let mut index = 0;
    sequence.for_each(|value| {
        if let Err(error) = validate(value) {
            errors.insert(index, error);
        }
        index += 1;
    });
    if errors.is_empty() {
        Ok(())
    } else {
        Err(Composited::Array(errors))
    }
}

fn collect_map<'a, K, V, E, I, F>(entries: I, mut validate: F) -> Result<(), Composited<E>>
where
    K: ToString + 'a,
    V: 'a,
    I: IntoIterator<Item = (&'a K, &'a V)>,
    F: FnMut(&V) -> Result<(), Composited<E>>,
{
    let mut errors = indexmap::IndexMap::new();
    for (key, value) in entries {
        if let Err(error) = validate(value) {
            errors
                .entry(std::borrow::Cow::Owned(key.to_string()))
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

macro_rules! define_copy {
    ($Trait:ident, $method:ident, $Base:ident::$base_method:ident, $name:ident: $arg:ty, $Error:ty) => {
        #[doc(hidden)]
        pub trait $Trait<P = composited_path::Scalar> {
            fn $method(&self, $name: $arg) -> Result<(), Composited<$Error>>;
        }
        impl<T: $Base + ?Sized> $Trait<composited_path::Scalar> for T {
            fn $method(&self, $name: $arg) -> Result<(), Composited<$Error>> {
                $Base::$base_method(self, $name).map_err(Composited::Single)
            }
        }
        impl<T, P> $Trait<composited_path::Sequence<P>> for T
        where
            T: Sequence + ?Sized,
            T::Item: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                collect_sequence(self, |value| $Trait::$method(value, a))
            }
        }
    };
}

macro_rules! define_copy_map {
    ($Trait:ident, $method:ident, $Base:ident::$base_method:ident, $name:ident: $arg:ty, $Error:ty) => {
        define_copy!(
            $Trait,
            $method,
            $Base::$base_method,
            $name: $arg,
            $Error
        );
        define_copy_map!(@map (std::collections::HashMap<K, V>), $Trait, $method, $arg, $Error);
        define_copy_map!(@map (std::collections::BTreeMap<K, V>), $Trait, $method, $arg, $Error);
        define_copy_map!(@map (indexmap::IndexMap<K, V>), $Trait, $method, $arg, $Error);
    };
    (@map ($Map:ty), $Trait:ident, $method:ident, $arg:ty, $Error:ty) => {
        impl<K: ToString, V, P> $Trait<composited_path::Sequence<P>> for $Map
        where
            V: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                collect_map(self.iter(), |value| $Trait::$method(value, a))
            }
        }
    };
}

macro_rules! define_clone {
    ($Trait:ident, $method:ident, $Base:ident::$base_method:ident, $Error:ty) => {
        #[doc(hidden)]
        pub trait $Trait<T, P = composited_path::Scalar> {
            fn $method(&self, argument: T) -> Result<(), Composited<$Error>>;
        }
        impl<C, T: $Base<C> + ?Sized> $Trait<C, composited_path::Scalar> for T {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Base::$base_method(self, a).map_err(Composited::Single)
            }
        }
        impl<C: Clone, T, P> $Trait<C, composited_path::Sequence<P>> for T
        where
            T: Sequence + ?Sized,
            T::Item: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                collect_sequence(self, |value| $Trait::$method(value, a.clone()))
            }
        }
    };
}

macro_rules! define_clone_map {
    ($Trait:ident, $method:ident, $Base:ident::$base_method:ident, $Error:ty) => {
        define_clone!($Trait, $method, $Base::$base_method, $Error);
        define_clone_map!(@map (std::collections::HashMap<K, V>), $Trait, $method, $Error);
        define_clone_map!(@map (std::collections::BTreeMap<K, V>), $Trait, $method, $Error);
        define_clone_map!(@map (indexmap::IndexMap<K, V>), $Trait, $method, $Error);
    };
    (@map ($Map:ty), $Trait:ident, $method:ident, $Error:ty) => {
        impl<C: Clone, K: ToString, V, P> $Trait<C, composited_path::Sequence<P>> for $Map
        where
            V: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                collect_map(self.iter(), |value| $Trait::$method(value, a.clone()))
            }
        }
    };
}

macro_rules! define_enum {
    (@map ($Map:ty), $Trait:ident, $method:ident, $Error:ty) => {
        impl<C, K: ToString, V, P> $Trait<C, composited_path::Sequence<P>> for $Map
        where
            V: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                collect_map(self.iter(), |value| $Trait::$method(value, a))
            }
        }
    };
    ($Trait:ident, $method:ident, $Base:ident::$base_method:ident, $Error:ty) => {
        #[doc(hidden)]
        pub trait $Trait<T, P = composited_path::Scalar> {
            fn $method(&self, candidates: &[T]) -> Result<(), Composited<$Error>>;
        }
        impl<C, T: $Base<C> + ?Sized> $Trait<C, composited_path::Scalar> for T {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Base::$base_method(self, a).map_err(Composited::Single)
            }
        }
        impl<C, T, P> $Trait<C, composited_path::Sequence<P>> for T
        where
            T: Sequence + ?Sized,
            T::Item: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                collect_sequence(self, |value| $Trait::$method(value, a))
            }
        }
        define_enum!(@map (std::collections::HashMap<K, V>), $Trait, $method, $Error);
        define_enum!(@map (std::collections::BTreeMap<K, V>), $Trait, $method, $Error);
        define_enum!(@map (indexmap::IndexMap<K, V>), $Trait, $method, $Error);
    };
}

define_clone_map!(
    ValidateCompositedMaximum,
    validate_composited_maximum,
    ValidateMaximum::validate_maximum,
    MaximumError
);

define_clone_map!(
    ValidateCompositedMinimum,
    validate_composited_minimum,
    ValidateMinimum::validate_minimum,
    MinimumError
);

define_clone_map!(
    ValidateCompositedExclusiveMaximum,
    validate_composited_exclusive_maximum,
    ValidateExclusiveMaximum::validate_exclusive_maximum,
    ExclusiveMaximumError
);

define_clone_map!(
    ValidateCompositedExclusiveMinimum,
    validate_composited_exclusive_minimum,
    ValidateExclusiveMinimum::validate_exclusive_minimum,
    ExclusiveMinimumError
);

define_clone_map!(
    ValidateCompositedMultipleOf,
    validate_composited_multiple_of,
    ValidateMultipleOf::validate_multiple_of,
    MultipleOfError
);

define_copy_map!(
    ValidateCompositedMaxLength,
    validate_composited_max_length,
    ValidateMaxLength::validate_max_length,
    max_length: usize,
    MaxLengthError
);

define_copy_map!(
    ValidateCompositedMinLength,
    validate_composited_min_length,
    ValidateMinLength::validate_min_length,
    min_length: usize,
    MinLengthError
);

define_copy_map!(
    ValidateCompositedPattern,
    validate_composited_pattern,
    ValidatePattern::validate_pattern,
    pattern: &regex::Regex,
    PatternError
);

// Maps already implement the scalar Size validators. Recursing into map values
// would make HashMap<K, HashMap<..>> match both Scalar and Sequence, so rustc
// cannot infer the path for `#[validate(max_properties)]` / `min_properties`.
define_copy!(
    ValidateCompositedMaxProperties,
    validate_composited_max_properties,
    ValidateMaxProperties::validate_max_properties,
    max_properties: usize,
    MaxPropertiesError
);

define_copy!(
    ValidateCompositedMinProperties,
    validate_composited_min_properties,
    ValidateMinProperties::validate_min_properties,
    min_properties: usize,
    MinPropertiesError
);

define_enum!(
    ValidateCompositedEnum,
    validate_composited_enum,
    ValidateEnum::validate_enum,
    EnumError
);
