//! here lie structs for finite fields

use std::collections::HashSet;
use std::ops::{Add, Mul, Neg, Sub};

use crate::helpers::is_prime;
use crate::traits::{AbelianGroup, Associative, Commutative, Group, Invertible, Magma, Unit};

pub struct PrimeGenerator {
    curr: u64,
    next: u64,
}

impl PrimeGenerator {
    pub fn new() -> Self {
        Self { curr: 2, next: 3 }
    }
}

impl Iterator for PrimeGenerator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        self.curr = self.next;

        //uses the fact that all primes > 3 are of form 6k \pm 1
        loop {
            self.next += match self.next % 6 {
                1 => 4,
                _ => 2,
            };

            if is_prime(self.next) {
                break;
            }
        }
        Some(self.curr)
    }
}

///wrapper around u64 for primes
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub struct Prime {
    pub val: u64,
}

impl Prime {
    fn new(candidate: u64) -> Self {
        todo!();
    }
}

impl Add for Prime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self::new(self.val + rhs.val)
    }
}

impl Mul for Prime {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self::new(self.val * rhs.val)
    }
}

impl Iterator for Prime {
    type Item = Self;

    fn next(&mut self) -> Option<Self> {
        let mut val = self.val;

        loop {
            val += 1;
            if is_prime(val) {
                break;
            }
        }

        Some(Self::new(val))
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

impl Add for IntegerModN {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            val: (self.val + rhs.val) % self.n,
            n: match self.n == rhs.n {
                true => self.n,
                false => panic!(),
            },
        }
    }
}

impl Neg for IntegerModN {
    type Output = Self;

    fn neg(self) -> Self {
        if self.val == 0 {
            self
        } else {
            Self {
                val: self.n - self.val,
                n: self.n,
            }
        }
    }
}

impl Sub for IntegerModN {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        match self.n == rhs.n {
            true => Self {
                val: (self.val + (-rhs).val) % self.n,
                n: self.n,
            },
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
            true => Self {
                val: (self.val * rhs.val) % self.n,
                n: self.n,
            },
            false => {
                panic!();
            }
        }
    }
}

struct IntegersModN {
    elements: HashSet<IntegerModN>,
    order: u64,
}

impl IntegersModN {
    pub fn new(order: u64) -> Self {
        todo!();
    }

    pub fn get_order(self) -> u64 {
        self.order
    }
}

impl Magma for IntegersModN {
    type Element = IntegerModN;
}

impl Associative for IntegersModN {}

impl Unit for IntegersModN {
    type Base = Self;

    fn unit(base: Self::Base) -> IntegerModN {
        let order = base.get_order();
        IntegerModN::new(0, order)
    }
}

impl Invertible for IntegersModN {
    type Base = Self;

    fn inverse(k: IntegerModN) -> IntegerModN {
        -k
    }
}

impl Group for IntegersModN {}

impl Commutative for IntegersModN {}

impl AbelianGroup for IntegersModN {}

///an element of the field Z/pZ, where p is prime.
#[derive(Debug, PartialEq, Clone, Copy)]
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

impl Iterator for IntegerModP {
    type Item = Self;

    fn next(&mut self) -> Option<Self> {
        todo!();
    }
}

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
    fn p_add() {
        todo!();
    }

    #[test]
    #[should_panic]
    fn p_add_invalid() {
        todo!();
    }

    #[test]
    fn p_neg() {
        todo!();
    }

    #[test]
    fn p_sub() {
        todo!();
    }

    #[test]
    #[should_panic]
    fn p_sub_invalid() {
        todo!();
    }

    #[test]
    fn p_mul() {
        todo!();
    }

    #[test]
    #[should_panic]
    fn p_mul_invalid() {
        todo!();
    }
}
