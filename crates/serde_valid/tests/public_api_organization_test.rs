use serde_valid::traits::{IsMatch, IsUnique, Items, Length, Properties};
use serde_valid::{
    ValidateMaxItems, ValidateMaxLength, ValidateMaxProperties, ValidateMinItems, ValidateMinLength,
    ValidateMinProperties, ValidatePattern,
};

#[test]
fn capability_traits_are_public_under_traits() {
    fn assert_length<T: Length + ?Sized>() {}
    fn assert_properties<T: Properties + ?Sized>() {}
    fn assert_items<T: Items + ?Sized>() {}
    fn assert_match<T: IsMatch + ?Sized>() {}
    fn assert_unique<T: IsUnique + ?Sized>() {}

    assert_length::<str>();
    assert_properties::<std::collections::HashMap<String, i32>>();
    assert_items::<Vec<i32>>();
    assert_match::<str>();
    assert_unique::<[i32]>();
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
fn length_properties_items_and_is_match_blanket_implement_their_validators() {
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
    fn from_is_match<T: IsMatch + ?Sized>() {
        fn pattern<T: ValidatePattern + ?Sized>() {}
        pattern::<T>();
    }

    from_length::<str>();
    from_properties::<std::collections::HashMap<String, i32>>();
    from_items::<Vec<i32>>();
    from_is_match::<str>();
}
