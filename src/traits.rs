//! Traits for rings

use core::fmt::{Debug, Display};
use core::ops::{Add, Mul, Neg, Sub};
use std::collections::HashSet;
use std::hash::Hash;

pub use num_traits::identities::{one, zero, One, Zero};

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

/// An element of a group
pub trait GroupType:
    Eq + Add<Output = Self> + Neg<Output = Self> + Sub<Output = Self> + Zero + Clone + Debug
{
}

/// A group homomorphism
pub trait GroupHomo<A: GroupType, B: GroupType>: Fn(A) -> B {}

/// A ring homomorphism
pub trait RingHomo<A: RingType, B: RingType>: Fn(A) -> B {}

impl RingType for isize {}
impl RingType for i8 {}
impl RingType for i16 {}
impl RingType for i32 {}
impl RingType for i64 {}
impl RingType for i128 {}

impl GroupType for isize {}
impl GroupType for i8 {}
impl GroupType for i16 {}
impl GroupType for i32 {}
impl GroupType for i64 {}
impl GroupType for i128 {}

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
