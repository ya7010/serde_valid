#[cfg(feature = "i128")]
use std::num::{NonZeroI128, NonZeroU128};
use std::num::{
    NonZeroI16, NonZeroI32, NonZeroI64, NonZeroI8, NonZeroIsize, NonZeroU16, NonZeroU32,
    NonZeroU64, NonZeroU8, NonZeroUsize,
};

#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Number {
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    #[cfg(feature = "i128")]
    I128(i128),
    Isize(isize),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    #[cfg(feature = "i128")]
    U128(u128),
    Usize(usize),
    NonZeroI8(NonZeroI8),
    NonZeroI16(NonZeroI16),
    NonZeroI32(NonZeroI32),
    NonZeroI64(NonZeroI64),
    #[cfg(feature = "i128")]
    NonZeroI128(NonZeroI128),
    NonZeroIsize(NonZeroIsize),
    NonZeroU8(NonZeroU8),
    NonZeroU16(NonZeroU16),
    NonZeroU32(NonZeroU32),
    NonZeroU64(NonZeroU64),
    #[cfg(feature = "i128")]
    NonZeroU128(NonZeroU128),
    NonZeroUsize(NonZeroUsize),
    F32(f32),
    F64(f64),
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Number::I8(num) => write!(f, "{:?}", num),
            Number::I16(num) => write!(f, "{:?}", num),
            Number::I32(num) => write!(f, "{:?}", num),
            Number::I64(num) => write!(f, "{:?}", num),
            #[cfg(feature = "i128")]
            Number::I128(num) => write!(f, "{:?}", num),
            Number::Isize(num) => write!(f, "{:?}", num),
            Number::U8(num) => write!(f, "{:?}", num),
            Number::U16(num) => write!(f, "{:?}", num),
            Number::U32(num) => write!(f, "{:?}", num),
            Number::U64(num) => write!(f, "{:?}", num),
            #[cfg(feature = "i128")]
            Number::U128(num) => write!(f, "{:?}", num),
            Number::Usize(num) => write!(f, "{:?}", num),
            Number::NonZeroI8(num) => write!(f, "{:?}", num),
            Number::NonZeroI16(num) => write!(f, "{:?}", num),
            Number::NonZeroI32(num) => write!(f, "{:?}", num),
            Number::NonZeroI64(num) => write!(f, "{:?}", num),
            #[cfg(feature = "i128")]
            Number::NonZeroI128(num) => write!(f, "{:?}", num),
            Number::NonZeroIsize(num) => write!(f, "{:?}", num),
            Number::NonZeroU8(num) => write!(f, "{:?}", num),
            Number::NonZeroU16(num) => write!(f, "{:?}", num),
            Number::NonZeroU32(num) => write!(f, "{:?}", num),
            Number::NonZeroU64(num) => write!(f, "{:?}", num),
            #[cfg(feature = "i128")]
            Number::NonZeroU128(num) => write!(f, "{:?}", num),
            Number::NonZeroUsize(num) => write!(f, "{:?}", num),
            Number::F32(num) => write!(f, "{:?}", num),
            Number::F64(num) => write!(f, "{:?}", num),
        }
    }
}

macro_rules! impl_from_trait {
    ($type:ty) => {
        pastey::paste! {
            impl From<$type> for Number {
                fn from(item: $type) -> Self {
                    Number::[<$type:camel>](item)
                }
            }

            impl From<&$type> for Number {
                fn from(item: &$type) -> Self {
                    Number::[<$type:camel>](*item)
                }
            }
        }
    };
}

impl_from_trait!(i8);
impl_from_trait!(i16);
impl_from_trait!(i32);
impl_from_trait!(i64);
#[cfg(feature = "i128")]
impl_from_trait!(i128);
impl_from_trait!(isize);
impl_from_trait!(u8);
impl_from_trait!(u16);
impl_from_trait!(u32);
impl_from_trait!(u64);
#[cfg(feature = "i128")]
impl_from_trait!(u128);
impl_from_trait!(usize);
impl_from_trait!(NonZeroI8);
impl_from_trait!(NonZeroI16);
impl_from_trait!(NonZeroI32);
impl_from_trait!(NonZeroI64);
#[cfg(feature = "i128")]
impl_from_trait!(NonZeroI128);
impl_from_trait!(NonZeroIsize);
impl_from_trait!(NonZeroU8);
impl_from_trait!(NonZeroU16);
impl_from_trait!(NonZeroU32);
impl_from_trait!(NonZeroU64);
#[cfg(feature = "i128")]
impl_from_trait!(NonZeroU128);
impl_from_trait!(NonZeroUsize);
impl_from_trait!(f32);
impl_from_trait!(f64);
