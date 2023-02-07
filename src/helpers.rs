//! Helper functions for the other modules

use crate::module::RingType;
use core::ops::Rem;
use std::iter::once;

/// treating an (abelian) group as a Z-module with this multiplication --
/// $n\times a = a + a + ... + a$, where the addition is performed $n$ times.
pub fn mul_z_module<T: RingType>(n: i64, a: T) -> T {
    once(a)
        .cycle()
        .take(n.try_into().unwrap())
        .fold(T::zero(), |acc, a| acc + a)
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
}
