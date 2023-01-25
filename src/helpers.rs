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
        false
    } else {
        for k in 2..n {
            if (n % k) == 0 {
                return false;
            }
        }

        true
    }
}

///computes maximum. Returns a if a == b.
pub fn max<T: Ord>(a: T, b: T) -> T {
    if a >= b {
        a
    } else {
        b
    }
}

///computes min of two numbers. Returns a if a == b.
pub fn min<T: Ord>(a: T, b: T) -> T {
    if a <= b {
        a
    } else {
        b
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max() {
        assert_eq!(max(0, 1), 1);
        assert_eq!(max(4, 4), 4);
        assert_eq!(max(-4234, -232442), -4234);
    }

    #[test]
    fn test_min() {
        assert_eq!(min(0, 1), 0);
        assert_eq!(min(4, 4), 4);
        assert_eq!(min(-4234, -232442), -232442);
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
