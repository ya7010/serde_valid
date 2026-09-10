//! Type-level paths for composited validation.
//!
//! - [`Scalar`] — validate the value directly
//! - [`Transparent`] — unwrap `Box` / `&` / `Rc` / `Arc` / `Cow` / `Pin` and keep the inner path
//! - [`Sequence`] — validate each item / map value (maps share this path)
//! - [`Optional`] — skip `None`, otherwise keep the inner path

/// A scalar value validated directly.
#[doc(hidden)]
pub struct Scalar;

/// A transparent wrapper around another validation path.
#[doc(hidden)]
pub struct Transparent<P>(std::marker::PhantomData<fn() -> P>);

/// A value whose elements follow another validation path.
///
/// Indexed sequences produce array errors. Key-value iterators produce object errors, but use the
/// same path because both apply a validation rule to their values.
#[doc(hidden)]
pub struct Sequence<P>(std::marker::PhantomData<fn() -> P>);

/// An optional value that follows another validation path when present.
///
/// `Optional<P>` is distinct from [`Scalar`] so a generic `Option<T>` impl does not overlap the
/// scalar blanket that forwards through capability traits.
#[doc(hidden)]
pub struct Optional<P>(std::marker::PhantomData<fn() -> P>);
