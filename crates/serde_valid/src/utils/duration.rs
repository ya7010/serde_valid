use std::fmt::Debug;

/// A duration supported by the duration validation helpers.
///
/// `std::time::Duration` is always supported. Enable `chrono` for
/// `chrono::TimeDelta` (also called `chrono::Duration`), or `jiff` for
/// `jiff::SignedDuration`. Both features can be enabled together.
/// Comparisons use the duration type's native ordering without conversion.
///
/// `jiff::Span` is not supported because calendar units need a relative date
/// before they can be compared.
pub trait Duration: PartialOrd + Debug {}

impl Duration for std::time::Duration {}

#[cfg(feature = "chrono")]
impl Duration for chrono::TimeDelta {}

#[cfg(feature = "jiff")]
impl Duration for jiff::SignedDuration {}

/// Validate that the duration is less than or equal to the maximum.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use serde_valid::utils::duration_maximum;
/// use serde_valid::Validate;
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(custom = duration_maximum(Duration::from_micros(5)))]
///     val: Duration,
/// }
///
/// let s = TestStruct {
///     val: Duration::from_micros(5),
/// };
///
/// assert!(s.validate().is_ok());
/// ```
#[allow(dead_code)]
pub fn duration_maximum<T: Duration>(
    maximum: T,
) -> impl FnOnce(&T) -> Result<(), crate::validation::Error> {
    move |val: &T| {
        if *val <= maximum {
            Ok(())
        } else {
            Err(crate::validation::Error::Custom(format!(
                "Duration {val:?} is greater than maximum {maximum:?}.",
            )))
        }
    }
}

/// Validate that the duration is greater than or equal to the minimum.
///
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use serde_valid::utils::duration_minimum;
/// use serde_valid::Validate;
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(custom = duration_minimum(Duration::from_micros(5)))]
///     val: Duration,
/// }
///
/// let s = TestStruct {
///     val: Duration::from_secs(5),
/// };
///
/// assert!(s.validate().is_ok());
/// ```
#[allow(dead_code)]
pub fn duration_minimum<T: Duration>(
    minimum: T,
) -> impl FnOnce(&T) -> Result<(), crate::validation::Error> {
    move |val: &T| {
        if *val >= minimum {
            Ok(())
        } else {
            Err(crate::validation::Error::Custom(format!(
                "Duration {val:?} is less than minimum {minimum:?}.",
            )))
        }
    }
}

/// Validate that the duration is less than the exclusive maximum.
///     
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use serde_valid::utils::duration_exclusive_maximum;
/// use serde_valid::Validate;
///
/// #[derive(Validate)]
/// struct TestStruct {
///     #[validate(custom = duration_exclusive_maximum(Duration::from_micros(5)))]
///     val: Duration,
/// }
///
/// let s = TestStruct {
///     val: Duration::from_micros(4),
/// };
///
/// assert!(s.validate().is_ok());
/// ```
#[allow(dead_code)]
pub fn duration_exclusive_maximum<T: Duration>(
    maximum: T,
) -> impl FnOnce(&T) -> Result<(), crate::validation::Error> {
    move |val: &T| {
        if *val < maximum {
            Ok(())
        } else {
            Err(crate::validation::Error::Custom(format!(
                "Duration {val:?} is greater than or equal to exclusive maximum {maximum:?}.",
            )))
        }
    }
}

/// Validate that the duration is greater than the exclusive minimum.
///     
/// # Example
///
/// ```rust
/// use std::time::Duration;
///
/// use serde_valid::utils::duration_exclusive_minimum;
/// use serde_valid::Validate;
///  
/// #[derive(Validate)]
/// struct TestStruct {
///      #[validate(custom = duration_exclusive_minimum(Duration::from_micros(5)))]
///      val: Duration,
/// }
///
/// let s = TestStruct {
///     val: Duration::from_micros(6),
/// };
///
/// assert!(s.validate().is_ok());
/// ```
#[allow(dead_code)]
pub fn duration_exclusive_minimum<T: Duration>(
    minimum: T,
) -> impl FnOnce(&T) -> Result<(), crate::validation::Error> {
    move |val: &T| {
        if *val > minimum {
            Ok(())
        } else {
            Err(crate::validation::Error::Custom(format!(
                "Duration {val:?} is less than or equal to exclusive minimum {minimum:?}.",
            )))
        }
    }
}
