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

macro_rules! impl_fixed_sequence {
    ($Trait:ident, $method:ident, $arg:ty, $Error:ty, maps = $maps:ident) => {
        impl<T, P> $Trait<composited_path::Sequence<P>> for Vec<T> where T: $Trait<P> { fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> { $Trait::$method(self.as_slice(), a) } }
        impl<T, P, const N: usize> $Trait<composited_path::Sequence<P>> for [T; N] where T: $Trait<P> { fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> { $Trait::$method(self.as_slice(), a) } }
        impl<T, P> $Trait<composited_path::Sequence<P>> for [T] where T: $Trait<P> {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> {
                let errors: indexmap::IndexMap<usize, Composited<$Error>> = self.iter().enumerate().filter_map(|(i, v)| $Trait::$method(v, a).err().map(|e| (i, e))).collect();
                if errors.is_empty() { Ok(()) } else { Err(Composited::Array(errors)) }
            }
        }
        impl_fixed_sequence!(@maps $maps, $Trait, $method, $arg, $Error);
    };
    (@maps yes, $Trait:ident, $method:ident, $arg:ty, $Error:ty) => {
        impl<K: ToString, V, P> $Trait<composited_path::Sequence<P>> for std::collections::HashMap<K, V> where V: $Trait<P> {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> { collect_map(self.iter(), a, |v, a| $Trait::$method(v, a)) }
        }
        impl<K: ToString, V, P> $Trait<composited_path::Sequence<P>> for std::collections::BTreeMap<K, V> where V: $Trait<P> {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> { collect_map(self.iter(), a, |v, a| $Trait::$method(v, a)) }
        }
        impl<K: ToString, V, P> $Trait<composited_path::Sequence<P>> for indexmap::IndexMap<K, V> where V: $Trait<P> {
            fn $method(&self, a: $arg) -> Result<(), Composited<$Error>> { collect_map(self.iter(), a, |v, a| $Trait::$method(v, a)) }
        }
    };
    (@maps no, $Trait:ident, $method:ident, $arg:ty, $Error:ty) => {};
}

fn collect_map<'a, K, V, A: Copy, E, I, F>(
    entries: I,
    argument: A,
    mut validate: F,
) -> Result<(), Composited<E>>
where
    K: ToString + 'a,
    V: 'a,
    I: IntoIterator<Item = (&'a K, &'a V)>,
    F: FnMut(&V, A) -> Result<(), Composited<E>>,
{
    let mut errors = indexmap::IndexMap::new();
    for (key, value) in entries {
        if let Err(error) = validate(value, argument) {
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

macro_rules! define_fixed {
    ($Trait:ident, $method:ident, $Base:ident::$base_method:ident, $name:ident: $arg:ty, $Error:ty, maps = $maps:ident, example = ($values:expr, $example_arg:expr)) => {
        #[doc = concat!(
            "Applies [`", stringify!($Base), "`] to a scalar or each collection value and preserves collection error paths.\n\n",
            "Implementing the scalar trait is sufficient for scalar and sequence validation.\n\n",
            "# Examples\n\n",
            "```rust\n",
            "use serde_valid::composited::", stringify!($Trait), ";\n\n",
            "let values = ", stringify!($values), ";\n",
            "let result = ", stringify!($Trait), "::", stringify!($method),
            "(&values, ", stringify!($example_arg), ");\n",
            "assert!(result.is_err());\n",
            "```"
        )]
        pub trait $Trait<P = composited_path::Scalar> {
            fn $method(&self, $name: $arg) -> Result<(), Composited<$Error>>;
        }
        impl<T: $Base + ?Sized> $Trait<composited_path::Scalar> for T {
            fn $method(&self, $name: $arg) -> Result<(), Composited<$Error>> {
                $Base::$base_method(self, $name).map_err(Composited::Single)
            }
        }
        impl_fixed_sequence!($Trait, $method, $arg, $Error, maps = $maps);
    };
}

macro_rules! define_owned {
    ($Trait:ident, $method:ident, $Base:ident::$base_method:ident, $Error:ty) => {
        #[doc = concat!(
            "Applies [`", stringify!($Base), "`] to a scalar or each collection value and preserves collection error paths.\n\n",
            "Implementing the scalar trait is sufficient for scalar and sequence validation.\n\n",
            "# Examples\n\n",
            "```rust\n",
            "use serde_valid::composited::", stringify!($Trait), ";\n\n",
            "let values = vec![1_i32, 2, 3];\n",
            "let result = ", stringify!($Trait), "::", stringify!($method), "(&values, 2);\n",
            "assert!(result.is_err());\n",
            "```"
        )]
        pub trait $Trait<T, P = composited_path::Scalar> {
            fn $method(&self, argument: T) -> Result<(), Composited<$Error>>;
        }
        impl<C, T: $Base<C> + ?Sized> $Trait<C, composited_path::Scalar> for T {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Base::$base_method(self, a).map_err(Composited::Single)
            }
        }
        impl<C: Clone, T, P> $Trait<C, composited_path::Sequence<P>> for Vec<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_slice(), a)
            }
        }
        impl<C: Clone, T, P, const N: usize> $Trait<C, composited_path::Sequence<P>> for [T; N]
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_slice(), a)
            }
        }
        impl<C: Clone, T, P> $Trait<C, composited_path::Sequence<P>> for [T]
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                let errors: indexmap::IndexMap<usize, Composited<$Error>> = self
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| $Trait::$method(v, a.clone()).err().map(|e| (i, e)))
                    .collect();
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Composited::Array(errors))
                }
            }
        }
        impl<C: Clone, K: ToString, V, P> $Trait<C, composited_path::Sequence<P>>
            for std::collections::HashMap<K, V>
        where
            V: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                collect_map_clone(self.iter(), a, |v, a| $Trait::$method(v, a))
            }
        }
        impl<C: Clone, K: ToString, V, P> $Trait<C, composited_path::Sequence<P>>
            for std::collections::BTreeMap<K, V>
        where
            V: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                collect_map_clone(self.iter(), a, |v, a| $Trait::$method(v, a))
            }
        }
        impl<C: Clone, K: ToString, V, P> $Trait<C, composited_path::Sequence<P>>
            for indexmap::IndexMap<K, V>
        where
            V: $Trait<C, P>,
        {
            fn $method(&self, a: C) -> Result<(), Composited<$Error>> {
                collect_map_clone(self.iter(), a, |v, a| $Trait::$method(v, a))
            }
        }
    };
}

fn collect_map_clone<'a, K, V, A: Clone, E, I, F>(
    entries: I,
    argument: A,
    mut validate: F,
) -> Result<(), Composited<E>>
where
    K: ToString + 'a,
    V: 'a,
    I: IntoIterator<Item = (&'a K, &'a V)>,
    F: FnMut(&V, A) -> Result<(), Composited<E>>,
{
    let mut errors = indexmap::IndexMap::new();
    for (key, value) in entries {
        if let Err(error) = validate(value, argument.clone()) {
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

macro_rules! define_slice {
    ($(#[$meta:meta])* $Trait:ident, $method:ident, $Base:ident::$base_method:ident, $Error:ty) => {
        $(#[$meta])*
        #[doc = concat!(
            "Applies [`", stringify!($Base), "`] to a scalar or each collection value and preserves collection error paths.\n\n",
            "Implementing the scalar trait is sufficient for scalar and sequence validation.\n\n",
            "# Examples\n\n",
            "```rust\n",
            "use serde_valid::composited::", stringify!($Trait), ";\n\n",
            "let values = vec![\"red\", \"blue\"];\n",
            "let result = ", stringify!($Trait), "::", stringify!($method),
            "(&values, &[\"red\", \"green\"]);\n",
            "assert!(result.is_err());\n",
            "```"
        )]
        pub trait $Trait<T, P = composited_path::Scalar> {
            fn $method(&self, candidates: &[T]) -> Result<(), Composited<$Error>>;
        }
        impl<C, T: $Base<C> + ?Sized> $Trait<C, composited_path::Scalar> for T {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Base::$base_method(self, a).map_err(Composited::Single)
            }
        }
        impl<C, T, P> $Trait<C, composited_path::Sequence<P>> for Vec<T>
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_slice(), a)
            }
        }
        impl<C, T, P, const N: usize> $Trait<C, composited_path::Sequence<P>> for [T; N]
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                $Trait::$method(self.as_slice(), a)
            }
        }
        impl<C, T, P> $Trait<C, composited_path::Sequence<P>> for [T]
        where
            T: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                let errors: indexmap::IndexMap<usize, Composited<$Error>> = self
                    .iter()
                    .enumerate()
                    .filter_map(|(i, v)| $Trait::$method(v, a).err().map(|e| (i, e)))
                    .collect();
                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(Composited::Array(errors))
                }
            }
        }
        impl<C, K: ToString, V, P> $Trait<C, composited_path::Sequence<P>>
            for std::collections::HashMap<K, V>
        where
            V: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                collect_map(self.iter(), a, |v, a| $Trait::$method(v, a))
            }
        }
        impl<C, K: ToString, V, P> $Trait<C, composited_path::Sequence<P>>
            for std::collections::BTreeMap<K, V>
        where
            V: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                collect_map(self.iter(), a, |v, a| $Trait::$method(v, a))
            }
        }
        impl<C, K: ToString, V, P> $Trait<C, composited_path::Sequence<P>>
            for indexmap::IndexMap<K, V>
        where
            V: $Trait<C, P>,
        {
            fn $method(&self, a: &[C]) -> Result<(), Composited<$Error>> {
                collect_map(self.iter(), a, |v, a| $Trait::$method(v, a))
            }
        }
    };
}

define_owned!(
    ValidateCompositedMaximum,
    validate_composited_maximum,
    ValidateMaximum::validate_maximum,
    MaximumError
);
define_owned!(
    ValidateCompositedMinimum,
    validate_composited_minimum,
    ValidateMinimum::validate_minimum,
    MinimumError
);
define_owned!(
    ValidateCompositedExclusiveMaximum,
    validate_composited_exclusive_maximum,
    ValidateExclusiveMaximum::validate_exclusive_maximum,
    ExclusiveMaximumError
);
define_owned!(
    ValidateCompositedExclusiveMinimum,
    validate_composited_exclusive_minimum,
    ValidateExclusiveMinimum::validate_exclusive_minimum,
    ExclusiveMinimumError
);
define_owned!(
    ValidateCompositedMultipleOf,
    validate_composited_multiple_of,
    ValidateMultipleOf::validate_multiple_of,
    MultipleOfError
);

define_fixed!(ValidateCompositedMaxLength, validate_composited_max_length, ValidateMaxLength::validate_max_length, max_length: usize, MaxLengthError, maps = yes, example = (vec!["a", "long"], 2));
define_fixed!(ValidateCompositedMinLength, validate_composited_min_length, ValidateMinLength::validate_min_length, min_length: usize, MinLengthError, maps = yes, example = (vec!["a", "long"], 2));
define_fixed!(ValidateCompositedPattern, validate_composited_pattern, ValidatePattern::validate_pattern, pattern: &regex::Regex, PatternError, maps = yes, example = (vec!["red", "blue"], &regex::Regex::new("^red$").unwrap()));
define_fixed!(ValidateCompositedMaxProperties, validate_composited_max_properties, ValidateMaxProperties::validate_max_properties, max_properties: usize, MaxPropertiesError, maps = no, example = (vec![std::collections::BTreeMap::from([("a", 1)]), std::collections::BTreeMap::from([("a", 1), ("b", 2), ("c", 3)])], 2));
define_fixed!(ValidateCompositedMinProperties, validate_composited_min_properties, ValidateMinProperties::validate_min_properties, min_properties: usize, MinPropertiesError, maps = no, example = (vec![std::collections::BTreeMap::from([("a", 1)]), std::collections::BTreeMap::from([("a", 1), ("b", 2), ("c", 3)])], 2));

define_slice!(
    ValidateCompositedEnum,
    validate_composited_enum,
    ValidateEnum::validate_enum,
    EnumError
);
