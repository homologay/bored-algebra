//! for probabilistic primality testing

use std::collections::HashMap;
use rand::{thread_rng, Rng};

use crate::error::ArithmeticError;

/// Miller-Rabin primality test. Requires n > 4. true = pseudoprime, false = composite
pub fn miller_rabin(n: u64) -> Result<bool, ArithmeticError::AlgNotApplicable> {
    if n < 5 {
        return Err(ArithmeticError::AlgNotApplicable {
           alg: Miller-Rabin,
            el: n,
        });
    }
    let pow_2 = split_off_power_of_two(n);
    let a = random_base(n);                 //pick 1 < a < n
    // todo: odd powers
    //
    //      for each b = a^m mod n, output true if b = +- 1 
    //
    //      - need to implement modulo power function first
    //
    // todo: even powers
    //
    //      for each r with 1 <= r < k if b^{2^r} = -1 mod n output true
    //
    Ok(false)
}

/// computes unique integers m, k such that m is odd and n - 1 = 2^k m. key = k, val = m.
fn split_off_power_of_two(n: u64) -> HashMap<u64, u64> {
    todo!();
}

fn random_base(n: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2..=n)
}


