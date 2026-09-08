//! Validation rules applied to scalar values or to the elements of collections.
//!
//! Scalar validation traits such as [`crate::ValidateMinLength`] validate one value.
//! The traits in this module also apply those rules to sequence and map values while retaining the
//! index or key of each failing value. The [`Validate`](crate::Validate) derive selects the path
//! automatically.
//!
//! # Values and collections
//!
//! [`path::Scalar`] applies a rule directly. [`path::Sequence`] applies a rule to every value
//! yielded by a collection:
//!
//! | Rust type | Composited path | Error shape |
//! | --- | --- | --- |
//! | `String` | `Scalar` | one error |
//! | `Vec<String>` | `Sequence<Scalar>` | errors by numeric index |
//! | `Vec<Vec<String>>` | `Sequence<Sequence<Scalar>>` | errors by nested numeric index |
//! | `HashMap<String, String>` | `Sequence<Scalar>` | errors by map key |
//! | `HashMap<String, Vec<String>>` | `Sequence<Sequence<Scalar>>` | key, then numeric index |
//!
//! Sequence failures use [`Composited::Array`]. Map failures use [`Composited::Object`] and
//! `K::to_string()` as the property key. Map implementations consume an iterator of key-value
//! pairs, so the path represents value iteration rather than a particular map type. Map keys must
//! implement [`ToString`]. Errors from distinct keys with the same string representation are
//! collected under the same property.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! struct Request {
//!     #[validate(min_length = 3)]
//!     values: Vec<String>,
//! }
//!
//! let request = Request {
//!     values: vec!["Alice".to_owned(), "Bo".to_owned()],
//! };
//! assert!(request.validate().is_err());
//! ```
//!
mod error;
pub mod path;
mod validator;

pub use error::Composited;
pub use validator::{
    ValidateCompositedEnum, ValidateCompositedExclusiveMaximum, ValidateCompositedExclusiveMinimum,
    ValidateCompositedMaxLength, ValidateCompositedMaxProperties, ValidateCompositedMaximum,
    ValidateCompositedMinLength, ValidateCompositedMinProperties, ValidateCompositedMinimum,
    ValidateCompositedMultipleOf, ValidateCompositedPattern,
};
