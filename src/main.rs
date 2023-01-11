pub mod finite_fields;
pub mod macros;
pub mod permutations;
pub mod traits;
pub mod helpers;

use crate::helpers::{gcd, euler_phi_preop};

fn main() {
    println!("{}", gcd(43432342, 123124));
    println!("{}", euler_phi_preop(234231));
}
