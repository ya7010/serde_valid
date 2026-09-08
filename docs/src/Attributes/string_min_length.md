# String: "min_length" validation

The `#[validate(min_length = ???)]` attribute validates that a string has at least the given length.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Data (
    #[validate(min_length = 4)]
    String,
);

assert!(Data("tes".to_owned()).validate().is_err());
assert!(Data("test".to_owned()).validate().is_ok());
```
