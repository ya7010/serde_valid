//! Type-level paths for recursive composited validation.

/// A scalar value validated directly.
pub struct Scalar;

/// A transparent wrapper around another validation path.
pub struct Transparent<P>(std::marker::PhantomData<fn() -> P>);

/// A sequence whose elements follow another validation path.
pub struct Sequence<P>(std::marker::PhantomData<fn() -> P>);

/// An optional value following another validation path when present.
pub struct Optional<P>(std::marker::PhantomData<fn() -> P>);

/// A string-keyed map whose values follow another validation path.
pub struct Map<P>(std::marker::PhantomData<fn() -> P>);
