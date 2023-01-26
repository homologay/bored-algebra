//! All the macros defined in the library

#![allow(unused_macros)]

use crate::traits::{IntegerModN, One, RingType, Zero};
use core::ops::{Add, Mul, Neg, Sub};
use std::fmt;
use std::fmt::Display;

//idea: a macro like finite_group!((a, b),
//                                  (abab, aB))
//
//                                  makes a group with presentation
//
//                                  < a, b | abab, ab^{-1} >
//
//      (so there's not so much annoying bs like \< and such)
//
//      i think the argument given above could be expr?
//
//      look up rust reference

macro_rules! finite_group {
    //ident -- variable/function names
    ($func_name:ident) => {
        println!("you called {:?}()", stringify!($funct_name));
    };
}

// ///given some cycles return the subgroup of S_n generated by them
// macro_rules! permutations_from {
//     //todo
// }

/*
///easy way to impl Zero
macro_rules! zero_impl {
    ($t:ty, $v:expr) => {
        impl Zero for $t {
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
*/

/// Generates a struct for the ring $\mathbb{Z}/n\mathbb{Z}$, for a given $n$. 
/// ```
/// // usage:
/// integers_mod!(IntegerMod5, 5);
/// ```
/// The above generates a struct for $\mathbb{Z}/5\mathbb{Z}$. 
/// # Note:
/// If $n$ is prime, use `finite_field!` instead, to get access to 
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
