pub mod finite_fields;

use crate::finite_fields::*;

fn main() {
    println!("{}", gcd(43432342, 123124));
    println!("{}", euler_phi_preop(234231));
}
