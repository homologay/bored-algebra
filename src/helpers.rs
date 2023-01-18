//! helper functions

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

/// the greatest common divisor of two positive integers.
pub fn gcd(a: u64, b: u64) -> u64 {
    let mut a_mut = a;
    let mut b_mut = b;
    euclid_alg(a_mut, b_mut)
}

/// euclidean algorithm
pub fn euclid_alg(mut a: u64, mut b: u64) -> u64 {
    if (b == 1) || (b == 0) || (a == 1) || (a == 0) {
        return 1;
    }
    while b != 0 {
        let mut t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::*;

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
