//! An iterator that generates prime numbers (slowly)

use crate::helpers::is_prime;

pub struct PrimeGenerator {
    curr: u64,
    next: u64,
}

impl Default for PrimeGenerator {
    fn default() -> Self {
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
