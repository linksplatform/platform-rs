use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::iter::Step;

use num_traits::{AsPrimitive, FromPrimitive, PrimInt, Signed, ToPrimitive, Unsigned};

pub trait Num = PrimInt + Default + Debug + AsPrimitive<usize> + ToPrimitive;

pub trait SignNum = Num + Signed + FromPrimitive;

pub trait ToSigned {
    type Type: Num + Signed;

    fn to_signed(&self) -> Self::Type;
}

macro_rules! signed_type_impl {
    ($U:ty, $S:ty) => {
        impl ToSigned for $U {
            type Type = $S;

            fn to_signed(&self) -> Self::Type {
                *self as Self::Type
            }
        }
    };
}

signed_type_impl!(i8, i8);
signed_type_impl!(u8, i8);
signed_type_impl!(i16, i16);
signed_type_impl!(u16, i16);
signed_type_impl!(i32, i32);
signed_type_impl!(u32, i32);
signed_type_impl!(i64, i64);
signed_type_impl!(u64, i64);
signed_type_impl!(i128, i128);
signed_type_impl!(u128, i128);
signed_type_impl!(isize, isize);
signed_type_impl!(usize, isize);

pub trait MaxValue {
    const MAX: Self;
}

macro_rules! max_value_impl {
    ($T:ty) => {
        impl MaxValue for $T {
            const MAX: Self = <$T>::MAX;
        }
    };
}

// TODO: Create macro foreach
max_value_impl!(i8);
max_value_impl!(u8);
max_value_impl!(i16);
max_value_impl!(u16);
max_value_impl!(i32);
max_value_impl!(u32);
max_value_impl!(i64);
max_value_impl!(u64);
max_value_impl!(i128);
max_value_impl!(u128);
max_value_impl!(isize);
max_value_impl!(usize);

// TODO: Not use alias - IDEs does not support it
#[rustfmt::skip]
pub trait LinkType:
    Num
    + Unsigned
    + Step
    + ToSigned
    + MaxValue
    + FromPrimitive
    + Debug
    + Display
    + Hash
    + Send
    + Sync
    + 'static { }

#[rustfmt::skip]
impl<
    All: Num
        + Unsigned
        + Step
        + ToSigned
        + MaxValue
        + FromPrimitive
        + Debug
        + Display
        + Hash
        + Send
        + Sync
        + 'static,
    > LinkType for All { }
