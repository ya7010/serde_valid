# Trailt

`serde_valid` consists of traits for all validations, and you can define validations for your own types as well.

Scalar validation traits return their specific error type. The corresponding
`validation::ValidateComposited*` traits preserve errors through nested containers. A scalar trait
implementation is automatically promoted to composited validation, so custom types need only the
scalar implementation.
Transparent wrappers and recursive containers use the composited traits; scalar traits describe
the terminal value itself.

See [this link](https://docs.rs/serde_valid/latest/serde_valid/#validations) for information on the trait needed to define each validation.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

struct MyType(String);

impl serde_valid::ValidateMaxLength for MyType {
    fn validate_max_length(&self, max_length: usize) -> Result<(), serde_valid::MaxLengthError> {
        self.0.validate_max_length(max_length)
    }
}

#[derive(Validate)]
struct Data (
    #[validate(max_length = 5)]
    MyType,
);

assert!(Data(MyType("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦".to_string())).validate().is_ok());
```
