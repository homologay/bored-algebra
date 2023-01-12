pub mod finite_fields;
pub mod helpers;
pub mod macros;
pub mod permutations;
pub mod prime_generation;
pub mod traits;

use crate::helpers::{euler_phi_preop, gcd};
use crate::prime_generation::PrimeGenerator;

fn main() {
    for p in PrimeGenerator::new() {
        println!("{}", p.to_string());
    }
}
