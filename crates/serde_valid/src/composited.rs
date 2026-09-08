//! Recursive validation for values nested in wrappers and containers.
//!
//! Custom transparent wrappers can preserve the validation path by forwarding
//! a composited validator to their inner value.
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
