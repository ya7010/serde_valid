# Custom validation

The `#[validate(custom = ???)]` attribute allows you to define your own validation logic.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

fn user_validation(val: &i32) -> Result<(), serde_valid::validation::Error> {
    if *val == 1 {
        return Err(
            serde_valid::validation::Error::Custom("custom error".to_string())
        )
    }
    Ok(())
}

#[derive(Validate)]
struct Data (
    #[validate(custom = user_validation)]
    i32,
    #[validate(custom = |v| user_validation(v))] // you can also use closures
    i32,
);

assert!(Data(0, 0).validate().is_ok());
assert!(Data(1, 1).validate().is_err());
```
