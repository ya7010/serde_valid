//! Low-level capabilities used by validation traits.
//!
//! Validation-rule traits such as [`crate::ValidateMinLength`] and [`crate::ValidateEnum`] are
//! available from the crate root and [`crate::validation`]. This module contains the underlying
//! capabilities used by their implementations:
//!
//! | Capability | Validators |
//! |------------|------------|
//! | [`Length`] | [`crate::ValidateMinLength`], [`crate::ValidateMaxLength`] |
//! | [`Size`] | [`crate::ValidateMinProperties`], [`crate::ValidateMaxProperties`] |
//! | [`IsMatch`] | [`crate::ValidatePattern`] |
//! | [`IsUnique`] | [`crate::ValidateUniqueItems`] |
//! | [`Sequence`] | [`crate::Validate`] (element-wise application of scalar validators) |
//!
//! For example, implementing [`Length`] for a custom scalar type provides both minimum-length and
//! maximum-length validation.
//!
//! ```rust
//! use serde_valid::{traits::Length, ValidateMaxLength, ValidateMinLength};
//!
//! struct Identifier(String);
//!
//! impl Length for Identifier {
//!     fn length(&self) -> usize {
//!         self.0.chars().count()
//!     }
//! }
//!
//! let value = Identifier("abc".to_owned());
//! assert!(value.validate_min_length(3).is_ok());
//! assert!(value.validate_max_length(2).is_err());
//! ```

mod is_match;
mod is_unique;
mod length;
mod sequence;
mod size;

pub use is_match::IsMatch;
pub use is_unique::IsUnique;
pub use length::Length;
pub use sequence::Sequence;
pub use size::Size;
