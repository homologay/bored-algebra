pub mod finite_fields;
pub mod helpers;
pub mod macros;
pub mod permutations;
pub mod traits;
pub mod prime_generation;

use crate::prime_generation::PrimeGenerator;
use crate::helpers::{euler_phi_preop, gcd};

fn main() {
    for p in PrimeGenerator::new() {
        println!("{}", p.to_string());
    }
}
