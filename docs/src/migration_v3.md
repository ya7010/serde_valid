# Migrating from v2 to v3

Version 3 separates scalar validation from recursive, path-aware validation and removes APIs that
were deprecated in v2.

## Replace `enumerate` with `enum`

The deprecated enumerate names have been removed:

| v2 | v3 |
| --- | --- |
| `#[validate(enumerate = [...])]` | `#[validate(r#enum = [...])]` |
| `ValidateEnumerate` | `ValidateEnum` |
| `ValidateCompositedEnumerate` | `ValidateCompositedEnum` |
| `EnumerateError` | `EnumError` |

`r#enum` uses Rust raw-identifier syntax because `enum` is a Rust keyword.

## Import composited APIs from `serde_valid::composited`

Composited APIs are no longer exported from `serde_valid::validation`:

```text
use serde_valid::validation::{Composited, ValidateCompositedMinLength};
```

```rust
use serde_valid::composited::{Composited, ValidateCompositedMinLength};
# let _: Option<Composited<serde_valid::MinLengthError>> = None;
# fn accepts<T: ValidateCompositedMinLength>(_: &T) {}
```

`serde_valid::validation` remains public for validation error types and scalar validation-related
types. Only the recursive composited API moved.

## Account for composited paths

Every composited validator now has a path type parameter. It defaults to `path::Scalar`, so direct
scalar use normally needs no annotation. Standard wrappers and containers compose the path
automatically.

Custom transparent wrappers must forward the relevant composited validator explicitly. See
[Composited validation](./composited.md#custom-transparent-wrappers) for a complete example.

`Composited` also has an `Object` variant so map validation errors can retain their actual property
keys. Code that exhaustively matches `Composited` must handle `Composited::Object`.

## Keep scalar and composited validation separate

In v2, some standard wrappers implemented string and enum capability traits directly. In v3,
recursion for those scalar rules is expressed through composited validation. Implement the scalar
trait for the underlying value, then use derive or the corresponding `ValidateComposited*` trait
when recursively validating a wrapper.

For example, a `Cow<str>` inside a derived type needs no custom implementation:

```rust
# extern crate serde_valid;
use std::borrow::Cow;
use serde_valid::Validate;

#[derive(Validate)]
struct Request<'a> {
    #[validate(min_length = 3)]
    name: Cow<'a, str>,
}

assert!(Request { name: Cow::Borrowed("Bo") }.validate().is_err());
```

## Update custom map keys

Recursive validation for `HashMap`, `BTreeMap`, and `IndexMap` requires `K: ToString`. Implement
`Display` for a custom key when its validation error should be reported under a property name:

```rust
# extern crate serde_valid;
use std::{collections::BTreeMap, fmt};
use serde_valid::Validate;

#[derive(Ord, PartialOrd, Eq, PartialEq)]
struct Key(u32);

impl fmt::Display for Key {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}", self.0)
    }
}

#[derive(Validate)]
struct Request {
    #[validate(min_length = 2)]
    values: BTreeMap<Key, String>,
}

let request = Request {
    values: BTreeMap::from([(Key(7), "x".to_owned())]),
};

assert!(request.validate().is_err());
```

The property name in the resulting error is `"7"`. Ensure different keys do not stringify to the
same value unless their errors should be combined.
