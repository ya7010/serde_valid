use super::{error::Composited, path as composited_path};
use crate::validation::{
    ValidateEnum, ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaxLength,
    ValidateMaxProperties, ValidateMaximum, ValidateMinLength, ValidateMinProperties,
    ValidateMinimum, ValidateMultipleOf, ValidatePattern,
};
use crate::{
    EnumError, ExclusiveMaximumError, ExclusiveMinimumError, MaxLengthError, MaxPropertiesError,
    MaximumError, MinLengthError, MinPropertiesError, MinimumError, MultipleOfError, PatternError,
};

fn collect_items<T, E, F>(
    values: impl IntoIterator<Item = T>,
    mut validate: F,
) -> Result<(), Composited<E>>
where
    F: FnMut(T) -> Result<(), Composited<E>>,
{
    let mut errors = indexmap::IndexMap::new();
    for (index, value) in values.into_iter().enumerate() {
        if let Err(error) = validate(value) {
            errors.insert(index, error);
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(Composited::Array(errors))
    }
}

fn collect_properties<'a, K, V, E, I, F>(entries: I, mut validate: F) -> Result<(), Composited<E>>
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

/// `Option` skips validation when `None` and otherwise follows the inner path `P`.
///
/// Uses [`composited_path::Optional`] so this does not overlap the scalar capability blanket.
/// `Option<Vec<_>>`, `Option<&[_]>` and `Option<HashMap<_, _>>` all reuse the inner impls.
macro_rules! impl_composited_option {
    ($Trait:ident, $method:ident, $arg:ty, $Error:ty) => {
        impl<T, P> $Trait<composited_path::Optional<P>> for Option<T>
        where
            T: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                match self {
                    Some(value) => $Trait::$method(value, a),
                    None => Ok(()),
                }
            }
        }
    };
    ($Trait:ident, $method:ident, $Error:ty) => {
        impl<C, T, P> $Trait<C, composited_path::Optional<P>> for Option<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                match self {
                    Some(value) => $Trait::$method(value, a),
                    None => Ok(()),
                }
            }
        }
    };
    (slice $Trait:ident, $method:ident, $Error:ty) => {
        impl<C, T, P> $Trait<C, composited_path::Optional<P>> for Option<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                match self {
                    Some(value) => $Trait::$method(value, a),
                    None => Ok(()),
                }
            }
        }
    };
}

/// Forward standard pointer wrappers through [`composited_path::Transparent`].
///
/// Uses `Transparent<P>` (not a generic `Box<T>: Trait<P>`) so this does not overlap the scalar
/// capability blanket for types like `Box<str>`.
macro_rules! impl_composited_wrappers {
    ($Trait:ident, $method:ident, $arg:ty, $Error:ty) => {
        impl<T: ?Sized, P> $Trait<composited_path::Transparent<P>> for &T
        where
            T: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                $Trait::$method(*self, a)
            }
        }
        impl<T: ?Sized, P> $Trait<composited_path::Transparent<P>> for &mut T
        where
            T: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<T: ?Sized, P> $Trait<composited_path::Transparent<P>> for Box<T>
        where
            T: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<T: ?Sized, P> $Trait<composited_path::Transparent<P>> for std::rc::Rc<T>
        where
            T: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<T: ?Sized, P> $Trait<composited_path::Transparent<P>> for std::sync::Arc<T>
        where
            T: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<T: ?Sized, P> $Trait<composited_path::Transparent<P>> for std::borrow::Cow<'_, T>
        where
            T: std::borrow::ToOwned + $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_ref(), a)
            }
        }
        impl<W, P> $Trait<composited_path::Transparent<P>> for std::pin::Pin<W>
        where
            W: std::ops::Deref,
            W::Target: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_ref().get_ref(), a)
            }
        }
    };
    ($Trait:ident, $method:ident, $Error:ty) => {
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for &T
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Trait::$method(*self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for &mut T
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for Box<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for std::rc::Rc<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for std::sync::Arc<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for std::borrow::Cow<'_, T>
        where
            T: std::borrow::ToOwned + $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_ref(), a)
            }
        }
        impl<C, W, P> $Trait<C, composited_path::Transparent<P>> for std::pin::Pin<W>
        where
            W: std::ops::Deref,
            W::Target: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_ref().get_ref(), a)
            }
        }
    };
    (slice $Trait:ident, $method:ident, $Error:ty) => {
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for &T
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Trait::$method(*self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for &mut T
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for Box<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for std::rc::Rc<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for std::sync::Arc<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Trait::$method(&**self, a)
            }
        }
        impl<C, T: ?Sized, P> $Trait<C, composited_path::Transparent<P>> for std::borrow::Cow<'_, T>
        where
            T: std::borrow::ToOwned + $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_ref(), a)
            }
        }
        impl<C, W, P> $Trait<C, composited_path::Transparent<P>> for std::pin::Pin<W>
        where
            W: std::ops::Deref,
            W::Target: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_ref().get_ref(), a)
            }
        }
    };
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
        impl_composited_option!($Trait, $method, $arg, $Error);
        define_copy!(@iter Vec<T>, $Trait, $method, $arg, $Error);
        define_copy!(@iter [T], $Trait, $method, $arg, $Error);
        define_copy!(@iter [T; N], $Trait, $method, $arg, $Error);
        impl_composited_wrappers!($Trait, $method, $arg, $Error);
    };
    (@iter Vec<T>, $Trait:ident, $method:ident, $arg:ty, $Error:ty) => {
        impl<T, P> $Trait<composited_path::Sequence<P>> for Vec<T>
        where
            T: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                collect_items(self.iter(), |value| $Trait::$method(value, a))
            }
        }
    };
    (@iter [T], $Trait:ident, $method:ident, $arg:ty, $Error:ty) => {
        impl<T, P> $Trait<composited_path::Sequence<P>> for [T]
        where
            T: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                collect_items(self.iter(), |value| $Trait::$method(value, a))
            }
        }
    };
    (@iter [T; N], $Trait:ident, $method:ident, $arg:ty, $Error:ty) => {
        impl<T, const N: usize, P> $Trait<composited_path::Sequence<P>> for [T; N]
        where
            T: $Trait<P>,
        {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                collect_items(self.iter(), |value| $Trait::$method(value, a))
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
                collect_properties(self.iter(), |value| $Trait::$method(value, a))
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
        impl_composited_option!($Trait, $method, $Error);
        define_clone!(@iter Vec<T>, $Trait, $method, $Error);
        define_clone!(@iter [T], $Trait, $method, $Error);
        define_clone!(@iter [T; N], $Trait, $method, $Error);
        impl_composited_wrappers!($Trait, $method, $Error);
    };
    (@iter Vec<T>, $Trait:ident, $method:ident, $Error:ty) => {
        impl<C: Clone, T, P> $Trait<C, composited_path::Sequence<P>> for Vec<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                collect_items(self.iter(), |value| $Trait::$method(value, a.clone()))
            }
        }
    };
    (@iter [T], $Trait:ident, $method:ident, $Error:ty) => {
        impl<C: Clone, T, P> $Trait<C, composited_path::Sequence<P>> for [T]
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                collect_items(self.iter(), |value| $Trait::$method(value, a.clone()))
            }
        }
    };
    (@iter [T; N], $Trait:ident, $method:ident, $Error:ty) => {
        impl<C: Clone, T, const N: usize, P> $Trait<C, composited_path::Sequence<P>> for [T; N]
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                collect_items(self.iter(), |value| $Trait::$method(value, a.clone()))
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
                collect_properties(self.iter(), |value| $Trait::$method(value, a.clone()))
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
                collect_properties(self.iter(), |value| $Trait::$method(value, a))
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
        impl_composited_option!(slice $Trait, $method, $Error);
        define_enum!(@iter Vec<T>, $Trait, $method, $Error);
        define_enum!(@iter [T], $Trait, $method, $Error);
        define_enum!(@iter [T; N], $Trait, $method, $Error);
        define_enum!(@map (std::collections::HashMap<K, V>), $Trait, $method, $Error);
        define_enum!(@map (std::collections::BTreeMap<K, V>), $Trait, $method, $Error);
        define_enum!(@map (indexmap::IndexMap<K, V>), $Trait, $method, $Error);
        impl_composited_wrappers!(slice $Trait, $method, $Error);
    };
    (@iter Vec<T>, $Trait:ident, $method:ident, $Error:ty) => {
        impl<C, T, P> $Trait<C, composited_path::Sequence<P>> for Vec<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                collect_items(self.iter(), |value| $Trait::$method(value, a))
            }
        }
    };
    (@iter [T], $Trait:ident, $method:ident, $Error:ty) => {
        impl<C, T, P> $Trait<C, composited_path::Sequence<P>> for [T]
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                collect_items(self.iter(), |value| $Trait::$method(value, a))
            }
        }
    };
    (@iter [T; N], $Trait:ident, $method:ident, $Error:ty) => {
        impl<C, T, const N: usize, P> $Trait<C, composited_path::Sequence<P>> for [T; N]
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                collect_items(self.iter(), |value| $Trait::$method(value, a))
            }
        }
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
