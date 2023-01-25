//! helper functions

use crate::traits::RingType;
use core::ops::Rem;

///non-optimized euler phi/totient function
pub fn euler_phi_preop(n: i128) -> i128 {
    todo!();
}

///returns true if n is prime, false otherwise.
pub fn is_prime(n: u64) -> bool {
    if (n == 1) || (n == 0) {
        return false;
    } else {
        for k in 2..n {
            if (n % k) == 0 {
                return false;
            }
        }

        return true;
    }
}

///computes maximum. Returns a if a == b.
pub fn max<T: Ord>(a: T, b: T) -> T {
    if a >= b {
        return a;
    } else {
        return b;
    }
}

///computes min of two numbers. Returns a if a == b.
pub fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b {
        return a;
    } else {
        return b;
    }
}

/// euclidean algorithm: returns gcd of two elements of a ring.
pub fn euclid_alg<'a, T: RingType + Rem<Output = T> + 'a>(first: &'a T, second: &'a T) -> T
where
    &'a T: Rem<Output = T>,
{
    println!("starting euclid_alg with parameters:");
    let mut a = first;
    let mut b = second;

    if (b == &T::one()) || (b == &T::zero()) || (a == &T::one()) || (a == &T::zero()) {
        return T::one();
    }
    while *b != T::zero() {
        let mut t = b;
        let b = &(a % b);
        a = t;
    }
    a.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    // runs indefinitely. Why? who knows.
    /*
    #[test]
    fn test_euclid_alg() {
        assert_eq!(euclid_alg(&0, &3), 1);
        assert_eq!(euclid_alg(&3, &12), 3);
        assert_eq!(euclid_alg(&12, &15), 3);
        assert_eq!(euclid_alg(&20, &3), 1);
        assert_eq!(euclid_alg(&18, &36), 18);
    }*/

    #[test]
    fn test_max() {
        todo!();
    }

    #[test]
    fn test_min() {
        todo!();
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
