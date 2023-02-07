use core::fmt::Debug;
use core::ops::{Add, Mul, Neg, Sub};
pub use num_traits::identities::{one, zero, One, Zero};

pub trait IntegerModN = RingType;

pub trait RingType = Eq
    + Add<Output = Self>
    + Neg<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Zero
    + One
    + Clone
    + Debug;

/// the type of an element of a module
pub trait ModType<R: RingType> {
    /// module multiplication
    fn mod_mul(r: R, m: Self) -> Self;
}

// then ring is Mod<Ring=Self, Ab=Self> and abelian group is Mod<Ring=Integer, Ab=Self>

/// A module homomorphism A -> B
pub trait Homo<R: RingType, A: ModType<R>, B: ModType<R>>: Fn(A) -> B + Zero {}

//composing two functions. maybe this will be useful, maybe not.
fn compose<A, B, C, G, F>(g: G, f: F) -> (impl Fn(A) -> C)
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x)) //'move' captures environment of closure
}
