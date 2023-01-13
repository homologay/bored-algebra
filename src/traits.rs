//! here lie traits for algebraic abstractions...


use core::fmt::{Display, Debug};
use core::ops::{Add, Mul, Neg, Sub};
use std::collections::HashSet;
use std::hash::Hash;

///property of having a 0
pub trait Zero: Sized + Add<Self, Output = Self> {
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

    #[inline]
    fn is_one(&self) -> bool
    where
        Self: PartialEq,
    {
        *self == Self::one()
    }
}

pub trait IntegerModN: Eq + Hash + Add + Neg + Sub + Mul + Clone + Copy + Display + Debug {}

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
