use std::ops::{Add, Mul, Neg, Sub};

///wrapper around u64 for primes
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Prime {
    val: u64,
}

impl Prime {
    fn new(candidate: u64) -> Option<Self> {
        if is_prime(candidate) {
            return Some(Self { val: candidate });
        } else {
            return None;
        }
    }
}

impl Add for Prime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            val: self.val + rhs.val,
        }
    }
}

impl Mul for Prime {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            val: self.val * rhs.val,
        }
    }
}

impl Iterator for Prime {
    type Item = Self;

    fn next(&mut self) -> Option<Self> {
        todo!();
    }
}

///non-optimized euler phi/totient function
pub fn euler_phi_preop(n: u64) -> u64 {
    if (n == 1) || (n == 0) {
        return 1;
    } else {
        return (1..n).map(|k| (gcd(k, n), k)).fold(
            0,
            |acc, (gcd, k)| {
                if gcd == 1 {
                    acc + 1
                } else {
                    acc
                }
            },
        );
    }
}

///returns true if n is prime, false otherwise.
pub fn is_prime(n: u64) -> bool {
    euler_phi_preop(n) == n - 1
}

///computes max of two numbers. Returns a if a == b.
pub fn max(a: u64, b: u64) -> u64 {
    if a >= b {
        return a;
    } else {
        return b;
    }
}

///computes min of two numbers. Returns a if a == b.
pub fn min(a: u64, b: u64) -> u64 {
    if a <= b {
        return a;
    } else {
        return b;
    }
}

///the greatest common divisor of two positive integers.
pub fn gcd(a: u64, b: u64) -> u64 {
    (1..=min(a, b))
        .map(|n| ((a % n == 0) && (b % n == 0), n))
        .fold(1, |acc, (divides, n)| if divides { n } else { acc })
}

///an element of the ring Z/nZ, where n may be composite.
#[derive(Debug, PartialEq, Clone, Copy)]
struct IntegerModN {
    val: u64, // a representative for the class of val mod n
    n: u64,   // the order of the group
              // NOTE: val < n always. important.
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

impl Iterator for IntegerModN {
    type Item = Self;

    fn next(&mut self) -> Option<Self> {
        todo!();
    }
}

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

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(0, 3), 1);
        assert_eq!(gcd(3, 12), 3);
        assert_eq!(gcd(12, 15), 3);
        assert_eq!(gcd(20, 3), 1);
        assert_eq!(gcd(18, 36), 18);
    }

    #[test]
    fn test_euler_phi_preop() {
        assert_eq!(euler_phi_preop(1), 1);
        assert_eq!(euler_phi_preop(0), 1);
        assert_eq!(euler_phi_preop(9), 6);
        assert_eq!(euler_phi_preop(97), 96);
        assert_eq!(euler_phi_preop(42), 12);
    }

    #[test]
    fn test_is_prime() {
        assert_eq!(is_prime(1), false);
        assert_eq!(is_prime(2), true);
        assert_eq!(is_prime(19), true);
        assert_eq!(is_prime(15), false);
    }
}
