//! All the macros defined in the library

#![allow(unused_macros)]

use crate::ring_traits::{IntegerModN, RingType};
use core::ops::{Add, Mul, Neg, Sub};
use num_traits::{One, Zero};
use std::fmt;
use std::fmt::Display;

#[doc(hidden)]
use super::*;

/// Generates a struct for the ring $\mathbb{Z}/n\mathbb{Z}$, for a given $n$.
/// ```
/// // usage:
/// integers_mod!(IntegerMod5, 5);
/// ```
/// The above generates a struct for $\mathbb{Z}/5\mathbb{Z}$.
/// # Note:
/// If $n$ is prime, use `finite_field!(GFnpow1, n, 1)` instead, to get access to
/// field-specific methods.
#[macro_export]
macro_rules! integers_mod {
    ($name:ident, $char:expr) => {
        //assert that $char is type u64?

        #[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
        pub struct $name {
            val: u64,
        }

        impl $name {
            #[inline]
            pub fn new(val: u64) -> Self {
                Self { val: val % $char }
            }
        }

        impl Add for $name {
            type Output = Self;

            #[inline]
            fn add(self, rhs: Self) -> Self {
                Self::new(self.val + rhs.val)
            }
        }

        impl Neg for $name {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                Self::new($char - self.val)
            }
        }

        impl Sub for $name {
            type Output = Self;

            #[inline]
            fn sub(self, rhs: Self) -> Self {
                self + (-rhs)
            }
        }

        impl Mul for $name {
            type Output = Self;

            #[inline]
            fn mul(self, rhs: Self) -> Self {
                Self::new(self.val * rhs.val)
            }
        }

        impl Zero for $name {
            #[inline]
            fn zero() -> Self {
                Self::new(0)
            }

            #[inline]
            fn is_zero(&self) -> bool {
                *self == Self::zero()
            }
        }

        impl One for $name {
            #[inline]
            fn one() -> Self {
                Self::new(1)
            }

            #[inline]
            fn is_one(&self) -> bool {
                *self == Self::one()
            }
        }

        impl Display for $name {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}", self.val)
            }
        }

        impl RingType for $name {}

        impl IntegerModN for $name {}
    };
}

/// Generates a struct for $GF(p, q)$, for a given prime $p$ and positive integer $q$.
/// ```
/// use bored_algebra::finite_field;
/// // usage:
/// finite_field!(GF5pow3, 5, 3);
/// ```
/// The above would generate $GF(5, 3)$, or alternately $\mathbb{F}\_{5^3}$.
#[macro_export]
macro_rules! finite_field {
    ($name:ident, $char:expr, $pow:expr) => {

        #[derive(Eq, PartialEq, Hash, Clone, Debug)]
        pub struct $name {
            todo!();
        }

        impl $name {
            #[inline]
            pub fn new() -> Self {
                todo!();
            }
        }

        impl Add for $name {
            type Output = Self;

            #[inline]
            fn add(self, Rhs: Self) -> Self {
                todo!();
            }
        }

        impl Neg for $name {
            type Output = Self;

            #[inline]
            fn neg(self) -> Self {
                todo!();
            }
        }

        impl Sub for $name {
            type Output = Self;

            #[inline]
            fn sub(self, Rhs: Self) -> Self {
                todo!();
            }
        }

        impl MulInverse for $name {
            type Output = Self;

            #[inline]
            pub fn mul_inverse(self) -> Self {
                todo!();
            }
        }

        impl One for $name {
            todo!();
        }

        impl Zero for $name {
            todo!();
        }

        impl RingType for $name {}

        impl Integral for $name {}

        impl FieldType for $name {}
};
}

//some available designators:
//
//  -block
//  -expr       expressions
//  -ident      var/fn names
//  -item
//  -literal    literal consts
//  -pat        pattern
//  -path
//  -stmt       statement
//  -tt         token tree
//  -ty         type
//  -vis        visibility qualifier
//

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn integers_mod() {
        integers_mod!(IntegerMod4, 4);

        let three = IntegerMod4::new(3);
        let one = IntegerMod4::new(1);
        let zero = IntegerMod4::zero();

        let sum = one + three;
        let difference = one - three;
        let product_1 = one * three;
        let product_2 = three * three;

        assert_eq!(one, IntegerMod4::one());
        assert_eq!(zero, IntegerMod4::new(0));
        assert_eq!(sum, zero);
        assert_eq!(difference, IntegerMod4::new(2));
        assert_eq!(product_1, three);
        assert_eq!(product_2, one);
    }
}
