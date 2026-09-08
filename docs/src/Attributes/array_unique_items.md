# Array: `unique_items`

`#[validate(unique_items)]` requires every item in an array or slice to be unique.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Request {
    #[validate(unique_items)]
    values: Vec<i32>,
}

assert!(Request { values: vec![1, 2, 3] }.validate().is_ok());
assert!(Request { values: vec![1, 2, 1] }.validate().is_err());
```

The item type must implement `Eq`, `Hash`, and `Debug`.
