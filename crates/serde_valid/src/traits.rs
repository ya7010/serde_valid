//! Low-level capabilities used by validation traits.
//!
//! Validation-rule traits such as [`crate::ValidateMinLength`] live at the crate root and in
//! [`crate::validation`]. This module holds the measurements those rules use. Implementing a
//! capability is not the same as implementing a validator: some rules are blanket-derived from a
//! capability, and others stay as validator implementations.
//!
//! ## Blanket-derived validators
//!
//! Implement the capability once. Both related validators become available through blanket
//! implementations.
//!
//! | Capability | Derived validators | Measures |
//! |------------|--------------------|----------|
//! | [`Length`] | [`crate::ValidateMinLength`], [`crate::ValidateMaxLength`] | string character length |
//! | [`Properties`] | [`crate::ValidateMinProperties`], [`crate::ValidateMaxProperties`] | object property count |
//! | [`Items`] | [`crate::ValidateMinItems`], [`crate::ValidateMaxItems`] | array item count |
//! | [`Numeric`] | [`crate::ValidateMinimum`], [`crate::ValidateMaximum`], [`crate::ValidateExclusiveMinimum`], [`crate::ValidateExclusiveMaximum`], [`crate::ValidateMultipleOf`] | numeric value |
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
//!
//!
//! ## Validators with no capability trait
//!
//! These rules are implemented directly. There is no shared measurement trait underneath them.
//!
//! | Validator | Typical implementors |
//! |-----------|----------------------|
//! | [`crate::ValidatePattern`] | strings, paths, and custom types |
//! | [`crate::ValidateUniqueItems`] | `Vec`, slices, arrays, and custom collections |
//! | [`crate::ValidateEnum`] | strings and numeric primitives |
//! | [`crate::Validate`] | `#[derive(Validate)]`, plus wrappers and standard collections |

mod is_match;
mod is_unique;
mod items;
mod length;
mod numeric;
mod properties;

#[allow(deprecated)]
pub use is_match::IsMatch;
#[allow(deprecated)]
pub use is_unique::IsUnique;
pub use items::Items;
pub use length::Length;
pub use numeric::Numeric;
pub use properties::Properties;
#[allow(deprecated)]
pub use properties::Size;
