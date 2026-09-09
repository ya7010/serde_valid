//! Type-level paths for composited validation.

/// A scalar value validated directly.
#[doc(hidden)]
pub struct Scalar;

/// A value whose elements follow another validation path.
///
/// Indexed sequences produce array errors. Key-value iterators produce object errors, but use the
/// same path because both apply a validation rule to their values.
#[doc(hidden)]
pub struct Sequence<P>(std::marker::PhantomData<fn() -> P>);
