use serde_valid::traits::{IsMatch, IsUnique, Length, Size};

#[test]
fn capability_traits_are_public_under_traits() {
    fn assert_length<T: Length + ?Sized>() {}
    fn assert_size<T: Size + ?Sized>() {}
    fn assert_match<T: IsMatch + ?Sized>() {}
    fn assert_unique<T: IsUnique + ?Sized>() {}

    assert_length::<str>();
    assert_size::<std::collections::HashMap<String, i32>>();
    assert_match::<str>();
    assert_unique::<[i32]>();
}
