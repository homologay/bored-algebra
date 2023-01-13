//! here lie traits for algebraic abstractions...

use core::ops::{Add, Mul, Neg, Sub};
use std::collections::HashSet;
use std::hash::Hash;

///property of having a zero, for types that don't require additional parameters
pub trait ZeroNoParams: Sized + Add<Self, Output = Self> {
    fn zero() -> Self;

    fn set_zero(&mut self) {
        *self = Zero::zero(param);
    }

    fn is_zero(&self) -> bool;
}

///property of having a zero, for types that require one additional u64 parameter
pub trait ZeroOneParam: Sized + Add<Self, Output = Self> {
    fn zero(param: u64) -> Self;

    fn set_zero(&mut self);

    fn is_zero(&self) -> bool;
}

///property of having a zero
pub trait Zero<T> {
    type OneParam: MaybeZeroOneParam;
    type NoParams: MaybeZeroNoParams;
}

pub trait MaybeZeroOneParam {
    fn behavior_maybe() {}
}

impl<T> MaybeZeroOneParam for T {}

impl<T: ZeroOneParam> MaybeZeroOneParam for T {
    fn behavior_maybe() {
        //what should this do??
    }
}

pub trait MaybeZeroNoParams {


///easy way to impl ZeroNoParams
macro_rules! zero_impl {
    ($t:ty, $v:expr) => {
        impl ZeroNoParams for $t {
            #[inline]
            fn zero() -> $t {
                $v
            }
            #[inline]
            fn is_zero(&self) -> bool {
                *self == $v
            }
        }
    };
}

zero_impl!(usize, 0);
zero_impl!(u8, 0);
zero_impl!(u16, 0);
zero_impl!(u32, 0);
zero_impl!(u64, 0);
zero_impl!(u128, 0);

zero_impl!(isize, 0);
zero_impl!(i8, 0);
zero_impl!(i16, 0);
zero_impl!(i32, 0);
zero_impl!(i64, 0);
zero_impl!(i128, 0);

zero_impl!(f32, 0.0);
zero_impl!(f64, 0.0);

pub trait One: Sized + Mul<Self, Output = Self> {
    /// Returns the multiplicative identity element of `Self`, `1`.
    ///
    /// # Purity
    ///
    /// This function should return the same result at all times regardless of
    /// external mutable state, for example values stored in TLS or in
    /// `static mut`s.
    // This cannot be an associated constant, because of bignums.
    fn one() -> Self;

    /// Sets `self` to the multiplicative identity element of `Self`, `1`.
    fn set_one(&mut self) {
        *self = One::one();
    }

    /// Returns `true` if `self` is equal to the multiplicative identity.
    ///
    /// For performance reasons, it's best to implement this manually.
    /// After a semver bump, this method will be required, and the
    /// `where Self: PartialEq` bound will be removed.
    #[inline]
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::one()
    }
}

macro_rules! one_impl {
    ($t:ty, $v:expr) => {
        impl One for $t {
            #[inline]
            fn one() -> $t {
                $v
            }
            #[inline]
            fn is_one(&self) -> bool {
                *self == $v
            }
        }
    };
}

one_impl!(usize, 1);
one_impl!(u8, 1);
one_impl!(u16, 1);
one_impl!(u32, 1);
one_impl!(u64, 1);
one_impl!(u128, 1);

one_impl!(isize, 1);
one_impl!(i8, 1);
one_impl!(i16, 1);
one_impl!(i32, 1);
one_impl!(i64, 1);
one_impl!(i128, 1);

one_impl!(f32, 1.0);
one_impl!(f64, 1.0);

pub trait Group {
    type Element: Eq + Hash + Add + Neg + Sub;

    //req'd

    ///returns the unit of the group
    fn unit(set: HashSet<Self::Element>) -> Self::Element;

    //optional

    ///returns the order of the group
    fn order(&self) -> u64 {
        panic!("order() is not implemented");
    }
}

pub trait Ring {
    type Element: Eq + Hash + Add + Neg + Sub + Mul;

    //req'd

    ///returns the 0 of the ring
    fn additive_unit(set: HashSet<Self::Element>) -> Self::Element;

    ///returns the 1 of the ring
    fn multiplicative_unit(set: HashSet<Self::Element>) -> Self::Element;

    //optional

    ///returns any zero divisors of the ring
    fn zero_divisors(&self) -> HashSet<Self::Element> {
        panic!("zero_divisors() is not implemented");
    }

    fn nilradical(&self) -> HashSet<Self::Element> {
        panic!("nilradical() is not implemented");
    }
}

pub trait Field: Ring {
    fn multiplicative_inverse(elem: Self::Element) -> Self::Element;
}

#[cfg(test)]
mod tests {
    use super::*;
}
