//! Traits for rings

use core::fmt::{Debug, Display};
use core::ops::{Add, Mul, Neg, Sub};
use std::collections::HashSet;
use std::hash::Hash;

pub use num_traits::identities::{one, zero, One, Zero};

/*
///property of having a 0
pub trait Zero: Sized + Add<Output = Self> {
    fn zero() -> Self;

    fn set_zero(&mut self) {
        *self = Zero::zero();
    }

    fn is_zero(&self) -> bool;
}

///property of having a 1
pub trait One: Sized + Mul<Self, Output = Self> {
    fn one() -> Self;

    fn set_one(&mut self) {
        *self = One::one();
    }

    fn is_one(&self) -> bool;
}
*/

///encompasses all the structs made by integers_mod!, such as IntegersMod5
pub trait IntegerModN: RingType + Copy + Display + Debug {}

///all the things for a ring
pub trait RingType:
    Eq
    + Add<Output = Self>
    + Neg<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Zero
    + One
    + Clone
    + Debug
{
}

impl RingType for isize {}
impl RingType for i8 {}
impl RingType for i16 {}
impl RingType for i32 {}
impl RingType for i64 {}
impl RingType for i128 {}

///for an element of an integral domain
pub trait Integral: RingType {}

///for an element of a division ring
pub trait MulInverse: RingType {
    fn mul_inverse(self) -> Self;
}

///for an element of a (finite, since we're not doing any others) field
pub trait FieldType: RingType + Integral + MulInverse {}

#[cfg(test)]
mod tests {
    use super::*;
}
