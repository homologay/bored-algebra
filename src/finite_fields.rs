//! here lie structs for finite fields

use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

use crate::integers_mod;
use crate::traits::{IntegerModN, One, RingType, Zero};

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

#[cfg(test)]
mod test {
    use super::*;
}
