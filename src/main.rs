pub mod finite_fields;
pub mod helpers;
pub mod macros;
pub mod permutations;
pub mod traits;

use crate::finite_fields::PrimeGenerator;
use crate::helpers::{euler_phi_preop, gcd};

fn main() {
    for p in PrimeGenerator::new() {
        println!("{}", p.to_string());
    }
}
