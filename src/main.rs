use bored_algebra::prime_generation::PrimeGenerator;

fn main() {
    for p in PrimeGenerator::new() {
        println!("{}", p.to_string());
    }
}
