//! here lie structs for finite fields

use std::collections::HashSet;
use std::ops::{Add, Mul, Neg, Sub};

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

use crate::helpers::{is_prime, max, min};
use crate::traits::{Zero, Field, Group, Ring};

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

///an element of the ring Z/nZ, where n may be composite.
#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
struct IntegerModN {
    val: u64, // a representative for the class of val mod n
    n: u64,   // the order of the group
}

impl IntegerModN {
    fn new(val: u64, n: u64) -> Self {
        Self { val: val % n, n: n }
    }
}

//to implement Zero ... change to enum struct or zero? then have cases for operators .. ugh
//or change trait, but idk how to do that to make it work with the struct as is...
//Some() optional field???

impl Add for IntegerModN {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        match self.n == rhs.n {
            true => Self::new(self.val + rhs.val, self.n),
            false => panic!(),
        }
    }
}

impl Neg for IntegerModN {
    type Output = Self;

    fn neg(self) -> Self {
        if self.val == 0 {
            self
        } else {
            Self::new(self.n - self.val, self.n)
        }
    }
}

impl Sub for IntegerModN {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match self.n == rhs.n {
            true => Self::new(self.val + (-rhs).val, self.n),
            false => {
                panic!();
            }
        }
    }
}

impl Mul for IntegerModN {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        match self.n == rhs.n {
            true => Self::new(self.val * rhs.val, self.n),
            false => {
                panic!();
            }
        }
    }
}

#[derive(Debug)]
struct IntegersModN {
    elements: HashSet<IntegerModN>,
    order: u64,
}

impl IntegersModN {
    pub fn new(order: u64) -> Self {
        let mut set = HashSet::new();
        for k in 0..order {
            let k_mod_n = IntegerModN::new(k, order);
            set.insert(k_mod_n);
        }
        Self {
            elements: set,
            order: order,
        }
    }

    pub fn get_order(self) -> u64 {
        self.order
    }
}

impl Ring for IntegersModN {
    type Element = IntegerModN;

    fn additive_unit(set: HashSet<Self::Element>) -> Self::Element {
        todo!();
    }

    fn multiplicative_unit(set: HashSet<Self::Element>) -> Self::Element {
        todo!();
    }
}

///an element of the field Z/pZ, where p is prime.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
struct IntegerModP {
    val: u64, // a representative for the class of val mod p
    p: Prime, // the order of the group
}

impl IntegerModP {
    fn new(val: u64, p: Prime) -> Self {
        todo!();
    }

    fn inverse(self) -> Self {
        todo!();
    }
}

impl Add for IntegerModP {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

impl Neg for IntegerModP {
    type Output = Self;

    fn neg(self) -> Self {
        todo!();
    }
}

impl Sub for IntegerModP {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        todo!();
    }
}

impl Mul for IntegerModP {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        todo!();
    }
}

///a polynomial with coefficients in a ring T
#[derive(Eq, PartialEq, Debug, Clone)]
struct Polynomial<T: Add + Neg + Sub + Mul> {
    core: Vec<T>, //the coefficients in order, places core[0] is x^0, core[1] is x^1, so on.
}

impl<T: Add + Neg + Sub + Mul> Polynomial<T> {
    fn from_vec(vec: Vec<T>) -> Self {
        Self { core: vec }
    }
}

impl<T: Add<Output = T> + Neg + Sub + Mul> Add for Polynomial<T> {
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

impl<T: Add + Neg<Output = T> + Sub + Mul> Neg for Polynomial<T> {
    type Output = Self;

    fn neg(self) -> Self {
        let neg_core = self.core.into_iter().map(|elem| -elem).collect::<Vec<T>>();

        Self::from_vec(neg_core)
    }
}

impl<T: Add<Output = T> + Neg<Output = T> + Sub + Mul> Sub for Polynomial<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl<T: Add<Output = T> + Neg + Sub + Mul<Output = T> + Zero> Mul for Polynomial<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let self_degree = self.core.len() - 1;
        let rhs_degree = rhs.core.len() - 1;
        let product_degree = self_degree + rhs_degree;

        //coefficients for self in increasing deg order, padded with zeros till product_degree
        let self_iter = self.core.iter().
            chain([T::zero(); rhs_degree].into_iter());

        //front padding of zeros, then coefficients for rhs in decreasing order 
        let rhs_iter = [T::zero(); self_degree].into_iter()
            .chain(rhs.core.iter().rev());

        let product = (0..=product_degree).into_iter()
            .map(|k| (k, &self_iter, &rhs_iter))
            .map(|(k, &s, &r)| s.zip(r)
                  .take(k)
                  .map(|(a, b)| a * b)
                  .sum()
                  )
            .collect::<Vec<T>>();

        Self::from_vec(product)
       }
}

///a polynomial ring with coefficients in a ring T
#[derive(Debug)]
struct PolynomialRing<T: Add + Neg + Sub + Mul> {
    elements: HashSet<Polynomial<T>>,
}

//impl Ring for PolynomialRing ...

///the field GF(q) for q = p^n for some prime p and positive integer n.
#[derive(Debug)]
struct FiniteField {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn n_add() {
        let n1 = IntegerModN { val: 3, n: 7 };
        let n2 = IntegerModN { val: 2, n: 7 };
        assert_eq!(n1 + n2, IntegerModN { val: 5, n: 7 }, "valid, no wrap");
        assert_eq!(n1 + n1 + n1, IntegerModN { val: 2, n: 7 }, "valid, wrap");
    }

    #[test]
    #[should_panic]
    fn n_add_invalid() {
        let n7 = IntegerModN { val: 1, n: 7 };
        let n3 = IntegerModN { val: 1, n: 3 };
        let n_invalid = n7 + n3;
    }

    #[test]
    fn n_neg() {
        let num = IntegerModN { val: 1, n: 7 };
        let minus_num = IntegerModN { val: 6, n: 7 };
        assert_eq!(minus_num, -num);
        let zero = IntegerModN { val: 0, n: 234 };
        assert_eq!(zero, -zero);
    }

    #[test]
    fn n_sub() {
        //two nonzero, no wrap
        let num1 = IntegerModN { val: 5, n: 9 };
        let num2 = IntegerModN { val: 3, n: 9 };
        let expected1 = IntegerModN { val: 2, n: 9 };
        assert_eq!(num1 - num2, expected1);

        //two nonzero, wrap
        let num3 = IntegerModN { val: 6, n: 9 };
        let expected2 = IntegerModN { val: 8, n: 9 };
        assert_eq!(num1 - num3, expected2);

        //one zero
        let zero = IntegerModN { val: 0, n: 9 };
        assert_eq!(num1 - zero, num1);
    }

    #[test]
    #[should_panic]
    fn n_sub_invalid() {
        let num1 = IntegerModN { val: 1, n: 2 };
        let num2 = IntegerModN { val: 1, n: 3 };
        let num3 = num1 - num2;
    }

    #[test]
    fn n_mul() {
        let num1 = IntegerModN { val: 3, n: 5 };
        let num2 = IntegerModN { val: 2, n: 5 };
        let num3 = IntegerModN { val: 1, n: 5 };
        assert_eq!(num1 * num2, num3);
    }

    #[test]
    #[should_panic]
    fn n_mul_invalid() {
        let num1 = IntegerModN { val: 3, n: 5 };
        let num2 = IntegerModN { val: 3, n: 6 };
        let num3 = num1 * num2;
    }

    #[test]
    fn polynomial_sum_difference() {
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
        // 1 + 3x + 2x^3
        let mod1 = Polynomial::<IntegerModN>::from_vec(vec![
            IntegerModN::new(1, 4),
            IntegerModN::new(3, 4),
            IntegerModN::new(0, 4),
            IntegerModN::new(2, 4),
        ]);
        // x + 3
        let mod2 = Polynomial::<IntegerModN>::from_vec(vec![
            IntegerModN::new(3, 4),
            IntegerModN::new(1, 4),
        ]);
        let mod_sum = mod1.clone() + mod2.clone();
        let mod_diff = mod1.clone() - mod2.clone();

        // 1 + 3x + 2x^3 + x + 3 = 2x^3
        assert_eq!(
            mod_sum,
            Polynomial::<IntegerModN>::from_vec(vec![
                IntegerModN::new(0, 4),
                IntegerModN::new(0, 4),
                IntegerModN::new(0, 4),
                IntegerModN::new(2, 4)
            ])
        );

        // 1 + 3x + 2x^3 - (x + 3) = 2 + 2x + 2x^3
        assert_eq!(
            mod_diff,
            Polynomial::<IntegerModN>::from_vec(vec![
                IntegerModN::new(2, 4),
                IntegerModN::new(2, 4),
                IntegerModN::new(0, 4),
                IntegerModN::new(2, 4)
            ])
        );
    }
}
