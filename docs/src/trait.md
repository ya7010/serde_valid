# Validation traits

Each validation rule has a public trait. Implement the corresponding trait to make a rule available for your own scalar type.

The derive macro uses the scalar trait for the field value and the traits in
[`serde_valid::composited`](./composited.md) when that value is nested in wrappers or containers.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

struct MyType(String);

impl serde_valid::ValidateMaxLength for MyType {
    fn validate_max_length(&self, max_length: usize) -> Result<(), serde_valid::MaxLengthError> {
        serde_valid::ValidateMaxLength::validate_max_length(&self.0, max_length)
    }
}

#[derive(Validate)]
struct Data (
    #[validate(max_length = 5)]
    MyType,
);

assert!(Data(MyType("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦".to_string())).validate().is_ok());
```

Implement only the scalar trait in the common case. For example, the implementation above also
allows `Vec<MyType>`, `Option<MyType>`, and standard transparent pointer wrappers to be used with
`#[validate(max_length = ...)]`; serde_valid supplies their composited implementations.

The public scalar traits are re-exported from the crate root and are also grouped under
`serde_valid::traits`. See the [crate API documentation](https://docs.rs/serde_valid/latest/serde_valid/)
for the complete list.
