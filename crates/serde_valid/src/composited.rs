//! Recursive validation for values nested in wrappers and containers.
//!
//! Scalar validation traits such as [`crate::ValidateMinLength`] validate one value.
//! The traits in this module apply those scalar rules recursively and retain the path to each
//! failing value. The [`Validate`](crate::Validate) derive selects the composited path
//! automatically, so application code normally implements only the scalar validation trait and
//! uses `#[validate(...)]`.
//!
//! # Standard wrappers and containers
//!
//! Serde Valid supplies recursive implementations for:
//!
//! - transparent wrappers: `&T`, `&mut T`, `Box<T>`, `Rc<T>`, `Arc<T>`, `Cow<T>`, and `Pin<P>`;
//! - sequences: `Vec<T>`, `[T; N]`, and `[T]`;
//! - optional values: `Option<T>`;
//! - maps: `HashMap<K, V>`, `BTreeMap<K, V>`, and `IndexMap<K, V>` for rules applied to values.
//!
//! Sequence failures use [`Composited::Array`]. Map failures use [`Composited::Object`] and
//! `K::to_string()` as the property key, so map keys must implement [`ToString`]. Errors from
//! distinct keys with the same string representation are collected under the same property.
//!
//! # How paths compose
//!
//! The markers in [`path`] are nested, rather than passed as separate arguments. Each marker
//! describes one layer between the field and the scalar value:
//!
//! | Rust type | Composited path |
//! | --- | --- |
//! | `String` | `Scalar` |
//! | `Box<String>` | `Transparent<Scalar>` |
//! | `Vec<Box<String>>` | `Sequence<Transparent<Scalar>>` |
//! | `Option<Vec<Box<String>>>` | `Optional<Sequence<Transparent<Scalar>>>` |
//! | `HashMap<String, Option<Vec<String>>>` | `Map<Optional<Sequence<Scalar>>>` |
//!
//! For example, `min_length` on `Option<Vec<Box<String>>>` resolves to
//! `ValidateCompositedMinLength<Optional<Sequence<Transparent<Scalar>>>>`. Users do not normally
//! write this path; Rust infers it through trait resolution.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! struct Request {
//!     #[validate(min_length = 3)]
//!     values: Option<Vec<Box<String>>>,
//! }
//!
//! let request = Request {
//!     values: Some(vec![Box::new("Bo".to_owned())]),
//! };
//! assert!(request.validate().is_err());
//! ```
//!
//! # Custom transparent wrappers
//!
//! A custom wrapper writes one path layer explicitly and forwards the remainder `P` to its inner
//! value:
//!
//! ```rust
//! use serde_valid::{
//!     composited::{
//!         path::Transparent, Composited, ValidateCompositedMinLength,
//!     },
//!     MinLengthError, Validate,
//! };
//!
//! struct Wrapper<T>(T);
//!
//! impl<T, P> ValidateCompositedMinLength<Transparent<P>> for Wrapper<T>
//! where
//!     T: ValidateCompositedMinLength<P>,
//! {
//!     fn validate_composited_min_length(
//!         &self,
//!         min_length: usize,
//!     ) -> Result<(), Composited<MinLengthError>> {
//!         ValidateCompositedMinLength::validate_composited_min_length(&self.0, min_length)
//!     }
//! }
//!
//! #[derive(Validate)]
//! struct Request {
//!     #[validate(min_length = 3)]
//!     names: Wrapper<Vec<String>>,
//! }
//!
//! let request = Request {
//!     names: Wrapper(vec!["Alice".to_owned(), "Bo".to_owned()]),
//! };
//! assert!(request.validate().is_err());
//! ```

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
