//! # Serde Valid
//!
//! [![Latest Version](https://img.shields.io/crates/v/serde_valid.svg?color=green&style=flat-square)](https://crates.io/crates/serde_valid)
//! [![crate docs](https://docs.rs/serde_valid/badge.svg)](https://docs.rs/serde_valid/latest/serde_valid/)
//! [![GitHub license](https://badgen.net/github/license/Naereen/Strapdown.js?style=flat-square)](https://github.com/Naereen/StrapDown.js/blob/master/LICENSE)
//!
//! This is [JSON Schema](https://json-schema.org/) based validation tool using [serde](https://github.com/serde-rs/serde).
//!
//! ## Usage
//!
//! You derive `Validate` trait, and write validations.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! struct Data {
//!     #[validate(minimum = 0)]
//!     #[validate(maximum = 10)]
//!     val: i32,
//! }
//!
//! #[derive(Validate)]
//! enum DataEnum {
//!     Named {
//!         #[validate]
//!         a: Data,
//!     },
//! }
//!
//! let s = DataEnum::Named {
//!     a: Data { val: 5 },
//! };
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! ## Feature Flags
//!
//! - `toml` - provide serialization/deserialization in `toml` format.
//! - `yaml` - provide serialization/deserialization in `yaml` format.
//! - `i128` - support `i128`/`u128` type (default).
//! - `fluent` - provide localization using [fluent](https://projectfluent.org/).
//!
//! ## Validations
//!
//! Serde Valid support standard validation based JSON Schema.
//!
//! | Type    | Serde Valid (validate derive)          | Serde Valid (validate trait) | JSON Schema                                                                                   |
//! | :-----: | :------------------------------------- | :--------------------------- | :-------------------------------------------------------------------------------------------- |
//! | String  | `#[validate(max_length = 5)]`          | [`ValidateMaxLength`]        | [maxLength](https://json-schema.org/understanding-json-schema/reference/string#length)        |
//! | String  | `#[validate(min_length = 5)]`          | [`ValidateMinLength`]        | [minLength](https://json-schema.org/understanding-json-schema/reference/string#length)        |
//! | String  | `#[validate(pattern = r"^\d{5}$")]`    | [`ValidatePattern`]          | [pattern](https://json-schema.org/understanding-json-schema/reference/string#regexp)          |
//! | Numeric | `#[validate(maximum = 5)]`             | [`ValidateMaximum`]          | [maximum](https://json-schema.org/understanding-json-schema/reference/numeric#range)          |
//! | Numeric | `#[validate(minimum = 5)]`             | [`ValidateMinimum`]          | [minimum](https://json-schema.org/understanding-json-schema/reference/numeric#range)          |
//! | Numeric | `#[validate(exclusive_maximum = 5)]`   | [`ValidateExclusiveMaximum`] | [exclusiveMaximum](https://json-schema.org/understanding-json-schema/reference/numeric#range) |
//! | Numeric | `#[validate(exclusive_minimum = 5)]`   | [`ValidateExclusiveMinimum`] | [exclusiveMinimum](https://json-schema.org/understanding-json-schema/reference/numeric#range) |
//! | Numeric | `#[validate(multiple_of = 5)]`         | [`ValidateMultipleOf`]       | [multipleOf](https://json-schema.org/understanding-json-schema/reference/numeric#multiples)   |
//! | Object  | `#[validate(max_properties = 5)]`      | [`ValidateMaxProperties`]    | [maxProperties](https://json-schema.org/understanding-json-schema/reference/object#size)      |
//! | Object  | `#[validate(min_properties = 5)]`      | [`ValidateMinProperties`]    | [minProperties](https://json-schema.org/understanding-json-schema/reference/object#size)      |
//! | Array   | `#[validate(max_items = 5)]`           | [`ValidateMaxItems`]         | [maxItems](https://json-schema.org/understanding-json-schema/reference/array#length)          |
//! | Array   | `#[validate(min_items = 5)]`           | [`ValidateMinItems`]         | [minItems](https://json-schema.org/understanding-json-schema/reference/array#length)          |
//! | Array   | `#[validate(unique_items)]`            | [`ValidateUniqueItems`]      | [uniqueItems](https://json-schema.org/understanding-json-schema/reference/array#uniqueItems)  |
//! | Generic | `#[validate(r#enum = [5, 10, 15])]`    | [`ValidateEnum`]             | [enum](https://json-schema.org/understanding-json-schema/reference/enum)                      |
//!
//! In addition, [serde_valid::utils][module@crate::utils] provides a type of validation not described in the JSON schema specification.
//!
//! | Type                                                                 | Serde Valid (validate derive)                              | Serde Valid (validation function)                                        |
//! | :------------------------------------------------------------------: | :--------------------------------------------------------- | :----------------------------------------------------------------------- |
//! | [Duration](https://doc.rust-lang.org/core/time/struct.Duration.html) | `#[validate(custom = duration_maximum(SECOND))]`           | [duration_maximum][`crate::utils::duration_maximum`]                     |
//! | [Duration](https://doc.rust-lang.org/core/time/struct.Duration.html) | `#[validate(custom = duration_minimum(ZERO))]`             | [duration_minimum][`crate::utils::duration_minimum`]                     |
//! | [Duration](https://doc.rust-lang.org/core/time/struct.Duration.html) | `#[validate(custom = duration_exclusive_maximum(SECOND))]` | [duration_exclusive_maximum][`crate::utils::duration_exclusive_maximum`] |
//! | [Duration](https://doc.rust-lang.org/core/time/struct.Duration.html) | `#[validate(custom = duration_exclusive_minimum(ZERO))]`   | [duration_exclusive_minimum][`crate::utils::duration_exclusive_minimum`] |
//!
//! ## Complete Constructor (Deserialization)
//!
//! Serde Valid support complete constructor method using by
//! [`serde_valid::json::FromJsonValue`](json::FromJsonValue) trait.
//!
//! ```rust
//! use serde::Deserialize;
//! use serde_valid::Validate;
//! use serde_valid::json::{json, FromJsonValue};
//!
//! #[derive(Debug, Deserialize, Validate)]
//! struct Data {
//!     #[validate(maximum = 100)]
//!     val: i32,
//! }
//!
//! // Deserialization and Validation!! 🚀
//! let err = Data::from_json_value(json!({ "val": 123 })).unwrap_err();
//!
//! assert_eq!(
//!     err.to_string(),
//!     json!({
//!         "errors": [],
//!         "properties": {
//!             "val": {
//!                 "errors": ["The number must be `<= 100`."]
//!             }
//!         }
//!     })
//!     .to_string()
//! );
//! ```
//!
//! You can force validation by only deserialization through `serde_valid`, and removing
//! `serde_json` from `Cargo.toml` of your project.
//!
//! ## Serialization
//!
//! For serialization, provides [`serde_valid::json::ToJsonString`](json::ToJsonString) trait.
//!
//! ```rust
//! use serde::Serialize;
//! use serde_valid::Validate;
//! use serde_valid::json::{json, ToJsonString};
//!
//! #[derive(Debug, Serialize, Validate)]
//! struct Data {
//!     #[validate(maximum = 100)]
//!     val: i32,
//! }
//!
//! assert_eq!(
//!     Data{ val: 12i32 }.to_json_string().unwrap(),
//!     json!({ "val": 12i32 }).to_json_string().unwrap()
//! );
//! ```
//!
//! ## Custom Message
//!
//! For user custom message, Serde Valid provides `message_fn` or `message`.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! #[inline]
//! fn min_error_message(_params: &serde_valid::MinItemsError) -> String {
//!     "this is custom message_fn.".to_string()
//! }
//!
//! #[derive(Validate)]
//! struct Data {
//!     #[validate(min_items = 4, message_fn = min_error_message)]
//!     #[validate(max_items = 2, message = "this is custom message.")]
//!     val: Vec<i32>,
//! }
//!
//! let s = Data { val: vec![1, 2, 3] };
//!
//! assert_eq!(
//!     s.validate().unwrap_err().to_string(),
//!     json!({
//!         "errors": [],
//!         "properties": {
//!             "val": {
//!                 "errors": [
//!                     "this is custom message_fn.",
//!                     "this is custom message."
//!                 ]
//!             }
//!         }
//!     })
//!     .to_string()
//! );
//! ```
//!
//! ### Fluent localization
//!
//! <section class="warning">
//! <code>fluent</code> feature is required.
//! </section>
//!
//! You can also use [fluent](https://projectfluent.org/) localization by using `fluent` feature.
//!
//! Allow the following attributes:
//! - `#[validate(..., fluent("message-id", key1 = value1, ...))]`
//! - `#[validate(..., message_l10n = fluent("message-id", key1 = value1, ...))]`
//!
//! ```rust
//! # #[cfg(feature = "fluent")] {
//! # use fluent::{FluentBundle, FluentResource};
//! use unic_langid::LanguageIdentifier;
//! use serde_json::json;
//! use serde_valid::{fluent::Localize, Validate};
//!
//! # fn get_bundle(source: impl Into<String>) -> FluentBundle<FluentResource> {
//! #     let res = FluentResource::try_new(source.into()).expect("Failed to parse an FTL string.");
//! #     let langid_en: LanguageIdentifier = "ja_JP".parse().expect("Parsing failed");
//! #     let mut bundle = FluentBundle::new(vec![langid_en]);
//! #     bundle.add_resource(res).unwrap();
//! #     bundle
//! # }
//!
//! #[derive(Validate)]
//! struct Data (
//!     #[validate(min_length = 3, fluent("name-min-length", min_length = 3))]
//!     String,
//! );
//!
//! assert_eq!(
//!     Data("田中".to_string()).validate()
//!         .unwrap_err()
//!         .localize(&get_bundle("name-min-length = 名前の長さは { $min_length } 文字以上でないといけません。"))
//!         .to_string(),
//!     json!({
//!         "errors": ["名前の長さは \u{2068}3\u{2069} 文字以上でないといけません。"]
//!     })
//!     .to_string()
//! );
//! # }
//! ```
//!
//! ## Custom Validation
//! ### Single Error Validation
//! You can use your custom validation using by `#[validate(custom = ...)]`.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! fn user_validation(_val: &i32) -> Result<(), serde_valid::validation::Error> {
//!     Ok(())
//! }
//!
//! #[derive(Validate)]
//! struct Data {
//!     #[validate(custom = user_validation)]
//!     val: i32,
//! }
//!
//! let s = Data { val: 1 };
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! And you can also use closure.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! fn user_validation(_val: &i32, param1: bool) -> Result<(), serde_valid::validation::Error> {
//!     Ok(())
//! }
//!
//! #[derive(Validate)]
//! struct Data {
//!     #[validate(custom = |v| user_validation(v, true))]
//!     val: i32,
//! }
//!
//! let s = Data { val: 1 };
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! You can use **closure**, so you can access other fields of the struct by using the `self` keyword.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! struct Data {
//!     val1: i32,
//!     #[validate(custom = |val2: &i32| {
//!         if self.val1 < *val2 {
//!             Ok(())
//!         } else {
//!             Err(serde_valid::validation::Error::Custom("val2 must be greater than val1".to_owned()))
//!         }
//!     })]
//!     val2: i32,
//! }
//!
//! let s = Data { val1: 2, val2: 1 };
//!
//! assert_eq!(
//!     s.validate().unwrap_err().to_string(),
//!     serde_json::json!({
//!         "errors": [],
//!         "properties": {
//!             "val2": {
//!                 "errors": ["val2 must be greater than val1"]
//!             }
//!         }
//!     })
//!     .to_string()
//! );
//! ```
//!
//! Custom validation is suitable for handling convenience validations not defined in JSON Schema.
//! `serde_valid::utils::*` provides convenience functions for specific types.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//! use serde_valid::utils::{duration_maximum, duration_minimum};
//!
//!
//! #[derive(Validate)]
//! struct Data {
//!     #[validate(custom = duration_maximum(std::time::Duration::from_micros(5)))]
//!     #[validate(custom = duration_minimum(std::time::Duration::from_micros(0)))]
//!     val1: std::time::Duration,
//! }
//!
//! let s = Data {
//!     val1: std::time::Duration::from_micros(1),
//! };
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! ### Multi Errors Validation
//! If you want to return multiple errors in the use custom validation method, you can use `#[validate(custom = ...)]` same as single error.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! // 🚀 Just change the return type from `Result<(), Error>` to `Result<(), Vec<Error>>` !!
//! fn user_validation(_val: &i32) -> Result<(), Vec<serde_valid::validation::Error>> {
//!     Ok(())
//! }
//!
//! #[derive(Validate)]
//! struct Data {
//!     #[validate(custom = user_validation)]
//!     val: i32,
//! }
//!
//! let s = Data { val: 1 };
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! ### Multi Fields Validation
//! Now, you can use `#[validate(custom = ...)]` for multi fields validation.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! fn sample_validation(val1: i32, val2: &str) -> Result<(), serde_valid::validation::Error> {
//!     Ok(())
//! }
//!
//! #[derive(Validate)]
//! #[validate(custom = |s| sample_validation(s.val1, &s.val2))]
//! struct Data {
//!     val1: i32,
//!     val2: String,
//! }
//!
//! let s = Data {
//!     val1: 1,
//!     val2: "val2".to_owned(),
//! };
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! ## Validate Traits
//!
//! By implementing the validation trait, Your original type can uses Serde Valid validations.
//!
//! ```rust
//! use serde_valid::Validate;
//!
//! struct MyType(String);
//!
//! impl serde_valid::ValidateMaxLength for MyType {
//!     fn validate_max_length(&self, max_length: usize) -> Result<(), serde_valid::MaxLengthError> {
//!         self.0.validate_max_length(max_length)
//!     }
//! }
//!
//! #[derive(Validate)]
//! struct Data {
//!     #[validate(max_length = 5)]
//!     val: MyType,
//! }
//!
//! let s = Data {
//!     val: MyType(String::from("😍👺🙋🏽👨‍🎤👨‍👩‍👧‍👦")),
//! };
//!
//! assert!(s.validate().is_ok());
//! ```
//!
//! ## Validation Errors Format
//! ### Named Struct
//! Field errors are output to `properties`.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! struct Data {
//!     #[validate(maximum = 4)]
//!     val: u32,
//! }
//!
//! let s = Data { val: 5 };
//!
//! assert_eq!(
//!     s.validate().unwrap_err().to_string(),
//!     json!({
//!         "errors": [],
//!         "properties": {
//!             "val": {
//!                 "errors": ["The number must be `<= 4`."]
//!             }
//!         }
//!     })
//!     .to_string()
//! );
//! ```
//!
//! ### Unnamed Struct
//! Field errors are output to `items`. The key for `items` is guaranteed to be a string of positive
//! numbers.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! struct Data (
//!     #[validate(maximum = 4)] u32,
//!     #[validate(maximum = 3)] u32,
//! );
//!
//! let s = Data ( 5, 4 );
//!
//! assert_eq!(
//!     s.validate().unwrap_err().to_string(),
//!     json!({
//!         "errors": [],
//!         "items": {
//!             "0": {
//!                 "errors": ["The number must be `<= 4`."]
//!             },
//!             "1": {
//!                 "errors": ["The number must be `<= 3`."]
//!             }
//!         }
//!     })
//!     .to_string()
//! );
//! ```
//!
//! ### New Type
//! Field errors are output to `errors`.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! struct Data (
//!     #[validate(maximum = 4)] u32
//! );
//!
//! let s = Data (5);
//!
//! assert_eq!(
//!     s.validate().unwrap_err().to_string(),
//!     json!({
//!         "errors": ["The number must be `<= 4`."]
//!     })
//!     .to_string()
//! );
//! ```
//!
//! ### Named Enum
//! Variant errors are output to `properties`.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! enum Data {
//!     Named {
//!         #[validate(maximum = 5)]
//!         a: i32,
//!         #[validate(maximum = 5)]
//!         b: i32,
//!     },
//! }
//!
//! let s = Data::Named { a: 6, b: 6 };
//!
//! assert_eq!(
//!     s.validate().unwrap_err().to_string(),
//!     json!({
//!         "errors": [],
//!         "properties": {
//!             "a": {
//!                 "errors": ["The number must be `<= 5`."]
//!             },
//!             "b": {
//!                 "errors": ["The number must be `<= 5`."]
//!             }
//!         }
//!     })
//!     .to_string()
//! );
//! ```
//!
//! ### Unnamed Enum
//! Variant errors are output to `items`. The key for `items` is guaranteed to be a string of
//! positive numbers.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! enum Data {
//!     Unnamed (
//!         #[validate(maximum = 5)] i32,
//!         #[validate(maximum = 5)] i32,
//!     ),
//! }
//!
//! let s = Data::Unnamed ( 6, 6 );
//!
//! assert_eq!(
//!     s.validate().unwrap_err().to_string(),
//!     json!({
//!         "errors": [],
//!         "items": {
//!             "0": {
//!                 "errors": ["The number must be `<= 5`."]
//!             },
//!             "1": {
//!                 "errors": ["The number must be `<= 5`."]
//!             }
//!         }
//!     })
//!     .to_string()
//! );
//! ```
//!
//! ### New Type Enum
//! Variant errors are output to `errors`.
//!
//! ```rust
//! use serde_json::json;
//! use serde_valid::Validate;
//!
//! #[derive(Validate)]
//! enum Data {
//!     NewType (
//!         #[validate(maximum = 5)] i32,
//!     ),
//! }
//!
//! let s = Data::NewType ( 6 );
//!
//! assert_eq!(
//!     s.validate().unwrap_err().to_string(),
//!     json!({
//!         "errors": ["The number must be `<= 5`."]
//!     })
//!     .to_string()
//! );
//! ```

pub mod error;
mod features;
pub mod json;
mod traits;
pub mod utils;
pub mod validation;

pub use error::{
    EnumError, Error, ExclusiveMaximumError, ExclusiveMinimumError, MaxItemsError, MaxLengthError,
    MaxPropertiesError, MaximumError, MinItemsError, MinLengthError, MinPropertiesError,
    MinimumError, MultipleOfError, PatternError, UniqueItemsError,
};
#[allow(unused_imports)]
pub use features::*;
use indexmap::IndexMap;
use std::{borrow::Cow, collections::HashMap};
pub use validation::{
    ValidateEnum, ValidateExclusiveMaximum, ValidateExclusiveMinimum, ValidateMaxItems,
    ValidateMaxLength, ValidateMaxProperties, ValidateMaximum, ValidateMinItems, ValidateMinLength,
    ValidateMinProperties, ValidateMinimum, ValidateMultipleOf, ValidatePattern,
    ValidateUniqueItems,
};

pub mod export {
    #[cfg(feature = "fluent")]
    pub use fluent;
    pub use once_cell;
    pub use regex;
}

pub trait Validate {
    fn validate(&self) -> std::result::Result<(), self::validation::Errors>;
}

impl<T> Validate for Vec<T>
where
    T: Validate,
{
    fn validate(&self) -> std::result::Result<(), self::validation::Errors> {
        let mut items = IndexMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(errors) = item.validate() {
                items.insert(index, errors);
            }
        }

        if items.is_empty() {
            Ok(())
        } else {
            Err(self::validation::Errors::Array(
                validation::error::ArrayErrors::new(vec![], items),
            ))
        }
    }
}

impl<T, const N: usize> Validate for [T; N]
where
    T: Validate,
{
    fn validate(&self) -> std::result::Result<(), self::validation::Errors> {
        let mut items = IndexMap::new();

        for (index, item) in self.iter().enumerate() {
            if let Err(errors) = item.validate() {
                items.insert(index, errors);
            }
        }

        if items.is_empty() {
            Ok(())
        } else {
            Err(self::validation::Errors::Array(
                validation::error::ArrayErrors::new(vec![], items),
            ))
        }
    }
}

impl<K, V> Validate for HashMap<K, V>
where
    V: Validate,
    for<'a> &'a K: Into<String>,
{
    fn validate(&self) -> std::result::Result<(), self::validation::Errors> {
        let mut items = IndexMap::new();

        for (key, value) in self.iter() {
            if let Err(errors) = value.validate() {
                items.insert(Cow::from(key.into()), errors);
            }
        }

        if items.is_empty() {
            Ok(())
        } else {
            Err(self::validation::Errors::Object(
                validation::error::ObjectErrors::new(vec![], items),
            ))
        }
    }
}

impl<K, V> Validate for IndexMap<K, V>
where
    V: Validate,
    for<'a> &'a K: Into<String>,
{
    fn validate(&self) -> std::result::Result<(), self::validation::Errors> {
        let mut items = IndexMap::new();

        for (key, value) in self.iter() {
            if let Err(errors) = value.validate() {
                items.insert(Cow::from(key.into()), errors);
            }
        }

        if items.is_empty() {
            Ok(())
        } else {
            Err(self::validation::Errors::Object(
                validation::ObjectErrors::new(vec![], items),
            ))
        }
    }
}

impl<T> Validate for Option<T>
where
    T: Validate,
{
    fn validate(&self) -> std::result::Result<(), self::validation::Errors> {
        match self {
            Some(value) => value.validate(),
            None => Ok(()),
        }
    }
}

pub use serde_valid_derive::Validate;

#[cfg(test)]
pub mod tests {
    pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
}
