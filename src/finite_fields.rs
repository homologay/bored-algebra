use std::ops::{Add, Neg, Sub};

///wrapper around u64 for primes
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Prime {
    val: u64,
}

impl Add for Prime {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            val: self.val + rhs.val,
        }
    }
}

///non-optimized euler phi/totient function
fn euler_phi_preop(n: u64) -> u64 {
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

///computes max of two numbers. Returns a if a == b.
fn max(a: u64, b: u64) -> u64 {
    if a >= b {
        return a;
    } else {
        return b;
    }
}

///computes min of two numbers. Returns a if a == b.
fn min(a: u64, b: u64) -> u64 {
    if a <= b {
        return a;
    } else {
        return b;
    }
}

fn gcd(a: u64, b: u64) -> u64 {
    (1..=min(a, b))
        .map(|n| ((a % n == 0) && (b % n == 0), n))
        .fold(1, |acc, (divides, n)| if divides { n } else { acc })
}

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

// returns the (additive) order of an element of Z/nZ
fn order(num: IntegerModN) -> usize {
    //todo
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let n1 = IntegerModN { val: 3, n: 7 };
        let n2 = IntegerModN { val: 2, n: 7 };
        assert_eq!(n1 + n2, IntegerModN { val: 5, n: 7 }, "valid, no wrap");
        assert_eq!(n1 + n1 + n1, IntegerModN { val: 2, n: 7 }, "valid, wrap");
    }

    #[test]
    #[should_panic]
    fn add_invalid() {
        let n7 = IntegerModN { val: 1, n: 7 };
        let n3 = IntegerModN { val: 1, n: 3 };
        let n_invalid = n7 + n3;
    }

    #[test]
    fn neg() {
        let num = IntegerModN { val: 1, n: 7 };
        let minus_num = IntegerModN { val: 6, n: 7 };
        assert_eq!(minus_num, -num);
        let zero = IntegerModN { val: 0, n: 234 };
        assert_eq!(zero, -zero);
    }

    #[test]
    fn sub() {
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
    fn sub_invalid() {
        let num1 = IntegerModN { val: 1, n: 2 };
        let num2 = IntegerModN { val: 1, n: 3 };
        let num3 = num1 - num2;
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
}
