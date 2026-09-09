# Migrating from serde_valid v2 to v3

Version 3 removes APIs deprecated in v2 and separates the public validation capabilities from the
internal machinery used to apply them recursively.

## Rename `enumerate` to `enum`

Replace the deprecated enumerate names with their enum equivalents.

| v2 | v3 |
| --- | --- |
| `#[validate(enumerate = [...])]` | `#[validate(r#enum = [...])]` |
| `ValidateEnumerate` | `ValidateEnum` |
| `EnumerateError` | `EnumError` |
| `ValidateCompositedEnumerate` | `ValidateEnum` for direct validation, or `#[validate(r#enum = [...])]` for derived validation |

The `r#` prefix is Rust raw-identifier syntax because `enum` is a Rust keyword.

Internally, derived collection validation uses `ValidateCompositedEnum`, but composited-validator
traits are not part of the supported public API in v3.

```rust
use serde_valid::Validate;

#[derive(Validate)]
struct Request {
    #[validate(r#enum = ["draft", "published"])]
    status: String,
}
```

## Remove direct composited-validator usage

`Composited` and the `ValidateCompositedXXX` traits are implementation details used by
`#[derive(Validate)]`. They are no longer part of the supported public API.

For a custom scalar, implement the corresponding public validation trait. For example:

```rust
use serde_valid::{MinLengthError, ValidateMinLength};

struct Identifier(String);

impl ValidateMinLength for Identifier {
    fn validate_min_length(&self, minimum: usize) -> Result<(), MinLengthError> {
        self.0.validate_min_length(minimum)
    }
}
```

When both minimum and maximum length validation should use the same definition, implement the
lower-level `Length` capability instead:

```rust
use serde_valid::traits::Length;

struct Identifier(String);

impl Length for Identifier {
    fn length(&self) -> usize {
        self.0.length()
    }
}
```

Object property count uses `Properties`. `Size` is a deprecated alias for `Properties`. Array item
count uses `Items`. Custom numeric wrappers use `Numeric` to derive the five numeric validators.
Built-in numeric types keep those validators implemented directly. These are not interchangeable
with `Length`.

```rust
use serde_valid::traits::{Items, Numeric, Properties};

struct Labels(std::collections::HashMap<String, String>);
struct Ids(Vec<i32>);
struct Count(i32);

impl Properties for Labels {
    fn properties(&self) -> usize {
        self.0.properties()
    }
}

impl Items for Ids {
    fn items(&self) -> usize {
        self.0.items()
    }
}

impl Numeric for Count {
    type Value = i32;

    fn numeric(&self) -> Self::Value {
        self.0
    }
}
```

## Custom collections use public validators

A custom collection implements the public validator or measurement trait it needs. Element-wise
field rules such as `#[validate(min_length = 3)]` apply to `Vec`, slices, arrays, and map values;
they are not enabled by implementing `Items`.

```rust
use serde_valid::traits::Items;
use serde_valid::Validate;

struct Ids(Vec<i32>);

impl Items for Ids {
    fn items(&self) -> usize {
        self.0.items()
    }
}

#[derive(Validate)]
struct Request {
    #[validate(max_items = 2)]
    ids: Ids,
    #[validate(min_length = 3)]
    names: Vec<String>,
}
```

See [`serde_valid::traits`](https://docs.rs/serde_valid/latest/serde_valid/traits/index.html) for
the capability-to-validator map.

## Use nested `Validate` for value wrappers

Pointer and optional wrappers delegate ordinary nested validation. Validate a wrapped type with
`#[validate]`; do not implement composited-validator traits for `Box<T>` or `Option<T>`.

```rust
use serde_valid::Validate;

#[derive(Validate)]
struct Child {
    #[validate(minimum = 1)]
    value: i32,
}

#[derive(Validate)]
struct Request {
    #[validate]
    child: Option<Box<Child>>,
}
```

## Update custom map keys

Validation of `HashMap`, `BTreeMap`, and `IndexMap` now requires `K: ToString`. Implement
`Display` for a custom key to receive the standard `ToString` implementation.

Map validation errors use the key's string representation as their property path instead of a
numeric iteration position. Update assertions or consumers that depended on numeric map positions.
If distinct keys produce the same string, their validation errors are combined under that property.
