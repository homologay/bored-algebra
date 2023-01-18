//! here lie structs for finite fields

use std::collections::HashSet;
use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

use crate::helpers::{is_prime, max, min};
use crate::integers_mod;
use crate::traits::{Field, Group, IntegerModN, One, Ring, RingType, Zero};

///wrapper around u64 for primes
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Prime {
    pub val: u64,
}

impl Prime {
    // assumes candidate has already been confirmed as prime
    fn new(candidate: u64) -> Self {
        Self { val: candidate }
    }
}

///A polynomial with coefficients in a ring T.
#[derive(Eq, PartialEq, Debug, Clone)]
struct Polynomial<T: RingType> {
    core: Box<Vec<T>>, //the coefficients in order, places core[0] is x^0, core[1] is x^1, so on.
}

impl<T: RingType> Polynomial<T> {
    fn from_vec(vec: Vec<T>) -> Self {
        Self {
            core: Box::new(vec),
        }
    }
}

impl<T: RingType> One for Polynomial<T> {
    fn one() -> Self {
        Self::from_vec(vec![T::one()])
    }

    fn is_one(&self) -> bool {
        if *self == Self::one() {
            return true;
        } else {
            return false;
        }
    }
}

impl<T: RingType> Zero for Polynomial<T> {
    fn zero() -> Self {
        Self::from_vec(vec![T::zero()])
    }

    fn is_zero(&self) -> bool {
        if *self == Self::zero() {
            return true;
        } else {
            return false;
        }
    }
}

impl<T: RingType> RingType for Polynomial<T> {}

impl<T: RingType> Add for Polynomial<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let sum = self
            .core
            .into_iter()
            .zip_longest(rhs.core.into_iter())
            .map(|elem| match elem {
                Both(a, b) => a + b,
                Left(b) => b,
                Right(a) => a,
            })
            .collect::<Vec<T>>();

        Self::from_vec(sum)
    }
}

impl<T: RingType> Neg for Polynomial<T> {
    type Output = Self;

    fn neg(self) -> Self {
        let neg_core = self.core.into_iter().map(|elem| -elem).collect::<Vec<T>>();

        Self::from_vec(neg_core)
    }
}

impl<T: RingType> Sub for Polynomial<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl<T: RingType> Mul for Polynomial<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        //degrees of the polynomials
        let n = self.core.len() - 1;
        let m = rhs.core.len() - 1;

        let lhs_vec = self.core;
        let rhs_vec = rhs.core;

        // the formula:
        //\sum_{k=0}^n a_k x^k \sum_{k=0}^m b_k x^k = \sum_{k=0}^{n+m} \sum_{i=0}^k a_i b_{k-i} x^k

        let mut result = Vec::new();

        for k in 0..=(n + m) {
            let mut kth_coeff = T::zero();

            for i in 0..=k {
                match (i > n) || ((k - i) > m) {
                    true => (),
                    false => {
                        kth_coeff = kth_coeff + (lhs_vec[i].clone() * rhs_vec[k - i].clone());
                    }
                }
            }
            result.push(kth_coeff);
        }
        Self::from_vec(result)
    }
}

impl<T: RingType + Div> Div for Polynomial<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn polynomial_arithmetic() {
        //addition and subtraction

        // over Z
        // x + 1
        let z1 = Polynomial::<i64>::from_vec(vec![1, 1]);
        // 2x^2 - 1
        let z2 = Polynomial::<i64>::from_vec(vec![-1, 0, 2]);

        // x + 1 + 2x^2 - 1 = 2x^2 + x
        assert_eq!(
            z1.clone() + z2.clone(),
            Polynomial::<i64>::from_vec(vec![0, 1, 2])
        );

        // x + 1 - (2x^2 - 1) = -2x^2 + x + 2
        assert_eq!(
            z1.clone() - z2.clone(),
            Polynomial::<i64>::from_vec(vec![2, 1, -2])
        );

        // over Z/4Z
        integers_mod!(IntegerMod4, 4);

        // 1 + 3x + 2x^3
        let mod1 = Polynomial::<IntegerMod4>::from_vec(vec![
            IntegerMod4::new(1),
            IntegerMod4::new(3),
            IntegerMod4::new(0),
            IntegerMod4::new(2),
        ]);
        // x + 3
        let mod2 =
            Polynomial::<IntegerMod4>::from_vec(vec![IntegerMod4::new(3), IntegerMod4::new(1)]);
        let mod_sum = mod1.clone() + mod2.clone();
        let mod_diff = mod1.clone() - mod2.clone();

        // 1 + 3x + 2x^3 + x + 3 = 2x^3
        assert_eq!(
            mod_sum,
            Polynomial::<IntegerMod4>::from_vec(vec![
                IntegerMod4::new(0),
                IntegerMod4::new(0),
                IntegerMod4::new(0),
                IntegerMod4::new(2)
            ])
        );

        // 1 + 3x + 2x^3 - (x + 3) = 2 + 2x + 2x^3
        assert_eq!(
            mod_diff,
            Polynomial::<IntegerMod4>::from_vec(vec![
                IntegerMod4::new(2),
                IntegerMod4::new(2),
                IntegerMod4::new(0),
                IntegerMod4::new(2)
            ])
        );

        //multiplication

        // over Z/4Z
        // 1 + 3x + 2x^3
        let mod1 = Polynomial::<IntegerMod4>::from_vec(vec![
            IntegerMod4::new(1),
            IntegerMod4::new(3),
            IntegerMod4::new(0),
            IntegerMod4::new(2),
        ]);
        // x + 3
        let mod2 =
            Polynomial::<IntegerMod4>::from_vec(vec![IntegerMod4::new(3), IntegerMod4::new(1)]);

        let mod_prod = mod1 * mod2;
        let mod_expected = Polynomial::<IntegerMod4>::from_vec(vec![
            IntegerMod4::new(3),
            IntegerMod4::new(2),
            IntegerMod4::new(3),
            IntegerMod4::new(2),
            IntegerMod4::new(2),
        ]);

        // (1 + 3x + 2x^3)(x + 3) = x + 3 + 3x^2 + 9x^2 + 2x^4 + 6x^3
        //                        = 2x^4 + 2x^3 + x + 3
        assert_eq!(mod_prod, mod_expected);
        //got:
    }

    #[test]
    fn multivariable_polynomial_arithmetic() {
        // in Z[x,y]

        let one =
            Polynomial::<Polynomial<i64>>::from_vec(vec![Polynomial::<i64>::from_vec(vec![1])]);
        let zero =
            Polynomial::<Polynomial<i64>>::from_vec(vec![Polynomial::<i64>::from_vec(vec![0])]);

        // R(x,y) = -1 + xy
        // L(x,y) =  1 - xy

        let r = Polynomial::<Polynomial<i64>>::from_vec(vec![
            Polynomial::<i64>::from_vec(vec![-1]),   //ones
            Polynomial::<i64>::from_vec(vec![0, 1]), //x's
        ]);

        let l = Polynomial::<Polynomial<i64>>::from_vec(vec![
            Polynomial::<i64>::from_vec(vec![1]),     //ones
            Polynomial::<i64>::from_vec(vec![0, -1]), //x's
        ]);

        assert_eq!(r + l, zero);

        // P(x,y) = xy + 3x^2 - 4y^2x - 8
        let p =
            Polynomial::<Polynomial<i64>>::from_vec(vec![Polynomial::<i64>::from_vec(vec![-8])]);

        // Q(x,y) = x^3 - 4y + 1

        //indexing confusion ....................................
    }
}
