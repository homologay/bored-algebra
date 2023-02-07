//! Traits for groups

use core::fmt::{Debug, Display};
use core::ops::{Add, Mul, Neg, Sub};

pub use num_traits::identities::{one, zero, One, Zero};

/// An element of a group
pub trait GroupType:
    Eq + Add<Output = Self> + Neg<Output = Self> + Sub<Output = Self> + Zero + Clone + Debug
{
}

/// A group homomorphism
pub trait GroupHomo<A: GroupType, B: GroupType>: Fn(A) -> B {}

impl GroupType for isize {}
impl GroupType for i8 {}
impl GroupType for i16 {}
impl GroupType for i32 {}
impl GroupType for i64 {}
impl GroupType for i128 {}
