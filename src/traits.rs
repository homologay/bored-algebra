//! here lie traits for algebraic abstractions... (based on set theory) 

use core::ops::{Add, Mul, Neg, Sub};
use std::hash::Hash;

///a trait to wrap (hash)sets with a binary operation "add"
pub trait Magma {
    type Element: Eq + Hash + Add;
}

// 'restriction' traits ...

pub trait Unit {
    type Base: Magma;

    //picks a distinguished element to act as a unit. As of now it is up to the trait implementor
    //to make sure the unit actually has the properties of the unit, maybe I can figure out how to
    //change this...
    fn unit(base: Self::Base) -> <<Self as Unit>::Base as Magma>::Element;
}

pub trait Associative: Magma {}

pub trait Invertible {
    type Base: Magma;

    fn inverse(elem: <<Self as Invertible>::Base as Magma>::Element) -> 
        <<Self as Invertible>::Base as Magma>::Element;
}

pub trait Commutative: Magma {}

// some aliases...

pub trait Monoid: Associative + Unit {}

pub trait Loop: Invertible + Unit {}

pub trait Group: Invertible + Associative + Unit {} 

pub trait AbelianGroup: Commutative + Group {}

// getting into ring stuff...


#[cfg(test)]
mod tests {
    use super::*;

}
