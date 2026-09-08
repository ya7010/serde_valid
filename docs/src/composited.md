# Composited validation

Scalar validation and recursive validation have separate responsibilities:

- `ValidateMinLength`, `ValidateMinimum`, `ValidateEnum`, and the other crate-root validation
  traits validate one scalar value.
- `serde_valid::composited::ValidateComposited*` applies a scalar rule recursively and records the
  path to each failing value.

`#[derive(Validate)]` selects the composited path automatically. Application code normally only
implements a scalar trait and uses `#[validate(...)]`.

## Standard wrappers and containers

Serde Valid supplies recursive implementations for:

- transparent wrappers: `&T`, `&mut T`, `Box<T>`, `Rc<T>`, `Arc<T>`, `Cow<T>`, and `Pin<P>`;
- sequences: `Vec<T>`, `[T; N]`, and `[T]`;
- optional values: `Option<T>`;
- maps: `HashMap<K, V>`, `BTreeMap<K, V>`, and `IndexMap<K, V>` where the rule applies to map
  values.

Sequence failures are represented by `Composited::Array`. Map failures are represented by
`Composited::Object`, using `K::to_string()` as the property key. A map key must therefore implement
`ToString` (normally through `Display`). If distinct keys produce the same string, their validation
errors are collected under the same property.

## Custom transparent wrappers

A wrapper that should preserve its inner validation path can forward the composited validator with
`path::Transparent`:

```rust
# extern crate serde_valid;
use serde_valid::{
    composited::{
        path::Transparent, Composited, ValidateCompositedMinLength,
    },
    MinLengthError, Validate,
};

struct Wrapper<T>(T);

impl<T, P> ValidateCompositedMinLength<Transparent<P>> for Wrapper<T>
where
    T: ValidateCompositedMinLength<P>,
{
    fn validate_composited_min_length(
        &self,
        min_length: usize,
    ) -> Result<(), Composited<MinLengthError>> {
        ValidateCompositedMinLength::validate_composited_min_length(&self.0, min_length)
    }
}

#[derive(Validate)]
struct Request {
    #[validate(min_length = 3)]
    names: Wrapper<Vec<String>>,
}

let request = Request {
    names: Wrapper(vec!["Alice".to_owned(), "Bo".to_owned()]),
};

assert!(request.validate().is_err());
```

The path parameter is structural. `Scalar` identifies the value that directly implements the
scalar validation trait; `Transparent<P>`, `Sequence<P>`, `Optional<P>`, and `Map<P>` describe each
recursive layer. Keeping this path in the type system allows scalar blanket implementations and
container implementations to coexist without overlapping.
