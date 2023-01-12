//! here lie traits for algebraic abstractions...

use core::ops::{Add, Mul, Neg, Sub};
use std::collections::HashSet;
use std::hash::Hash;

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
