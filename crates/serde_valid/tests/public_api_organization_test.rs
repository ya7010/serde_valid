#![allow(deprecated)]

use regex::Regex;
use serde_valid::traits::{IsMatch, IsUnique, Items, Length, Numeric, Properties};
use serde_valid::{
    ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaxItems, ValidateMaxLength,
    ValidateMaxProperties, ValidateMaximum, ValidateMinItems, ValidateMinLength,
    ValidateMinProperties, ValidateMinimum, ValidateMultipleOf, ValidatePattern,
    ValidateUniqueItems,
};

struct Count(i32);

impl Numeric for Count {
    type Value = i32;

    fn numeric(&self) -> Self::Value {
        self.0
    }
}

struct Identifier(String);

#[allow(deprecated)]
impl IsMatch for Identifier {
    fn is_match(&self, pattern: &Regex) -> bool {
        pattern.is_match(&self.0)
    }
}

struct Ids(Vec<i32>);

#[allow(deprecated)]
impl IsUnique for Ids {
    fn is_unique(&self) -> bool {
        self.0.validate_unique_items().is_ok()
    }
}

#[test]
#[allow(deprecated)]
fn capability_traits_are_public_under_traits() {
    fn assert_length<T: Length + ?Sized>() {}
    fn assert_properties<T: Properties + ?Sized>() {}
    fn assert_items<T: Items + ?Sized>() {}
    fn assert_numeric<T: Numeric>() {}
    fn assert_match<T: IsMatch + ?Sized>() {}
    fn assert_unique<T: IsUnique + ?Sized>() {}

    assert_length::<str>();
    assert_properties::<std::collections::HashMap<String, i32>>();
    assert_items::<Vec<i32>>();
    assert_numeric::<Count>();
    assert_match::<Identifier>();
    assert_unique::<Ids>();
}

#[test]
#[allow(deprecated)]
fn size_is_a_deprecated_alias_for_properties() {
    use serde_valid::traits::Size;

    fn from_properties<T: Properties + ?Sized>() {
        fn as_size<T: Size + ?Sized>() {}
        as_size::<T>();
    }
    fn from_size<T: Size + ?Sized>() {
        fn as_properties<T: Properties + ?Sized>() {}
        as_properties::<T>();
    }

    from_properties::<std::collections::HashMap<String, i32>>();
    from_size::<std::collections::HashMap<String, i32>>();
}

#[test]
fn length_properties_items_numeric_and_is_match_blanket_implement_their_validators() {
    fn from_length<T: Length + ?Sized>() {
        fn min<T: ValidateMinLength + ?Sized>() {}
        fn max<T: ValidateMaxLength + ?Sized>() {}
        min::<T>();
        max::<T>();
    }
    fn from_properties<T: Properties + ?Sized>() {
        fn min<T: ValidateMinProperties + ?Sized>() {}
        fn max<T: ValidateMaxProperties + ?Sized>() {}
        min::<T>();
        max::<T>();
    }
    fn from_items<T: Items + ?Sized>() {
        fn min<T: ValidateMinItems + ?Sized>() {}
        fn max<T: ValidateMaxItems + ?Sized>() {}
        min::<T>();
        max::<T>();
    }
    fn from_numeric<T: Numeric<Value = i32>>() {
        fn min<T: ValidateMinimum<i32>>() {}
        fn max<T: ValidateMaximum<i32>>() {}
        fn exclusive_min<T: ValidateExclusiveMinimum<i32>>() {}
        fn exclusive_max<T: ValidateExclusiveMaximum<i32>>() {}
        fn multiple_of<T: ValidateMultipleOf<i32>>() {}
        min::<T>();
        max::<T>();
        exclusive_min::<T>();
        exclusive_max::<T>();
        multiple_of::<T>();
    }
    #[allow(deprecated)]
    fn from_is_match<T: IsMatch + ?Sized>() {
        fn pattern<T: ValidatePattern + ?Sized>() {}
        pattern::<T>();
    }
    from_length::<str>();
    from_properties::<std::collections::HashMap<String, i32>>();
    from_items::<Vec<i32>>();
    from_numeric::<Count>();
    from_is_match::<Identifier>();
}
