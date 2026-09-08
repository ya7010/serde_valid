# Generic: `enum`

`#[validate(r#enum = [...])]` requires a value to equal one of the listed candidates. The `r#`
prefix is Rust raw-identifier syntax.

```rust
# extern crate serde_valid;
use serde_valid::Validate;

#[derive(Validate)]
struct Request {
    #[validate(r#enum = ["draft", "published"])]
    status: String,
}

assert!(Request { status: "draft".to_owned() }.validate().is_ok());
assert!(Request { status: "deleted".to_owned() }.validate().is_err());
```

For custom scalar types, implement `serde_valid::ValidateEnum<T>`. The deprecated
`#[validate(enumerate = ...)]` and `ValidateEnumerate` names are not available in v3.
