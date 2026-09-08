# Numeric: "exclusive_maximum" validation

The `#[validate(exclusive_maximum = ???)]` attribute validates that a number is less than a given value.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Data (
    #[validate(exclusive_maximum = 6)]
    i32
);

assert!(Data(5).validate().is_ok());
assert!(Data(6).validate().is_err());
assert!(Data(7).validate().is_err());
```
