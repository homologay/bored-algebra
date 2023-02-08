use core::fmt::Debug;
use core::ops::{Add, Mul, Neg, Sub};
pub use num_traits::identities::{one, zero, One, Zero};
use trait_set::trait_set;

trait_set! {
    pub trait IntegerModN = RingType;

    pub trait RingType = AbGroupType + Mul<Output = Self> + One;

    pub trait AbGroupType =
        Eq
        + Add<Output = Self>
        + Neg<Output = Self>
        + Sub<Output = Self>
        + Zero
        + Clone
        + Debug;
}

/// For a type that is a module. A ring is ModType<Self>, and a group is ModType<Z>.
pub trait ModType<R: RingType>: AbGroupType {
    /// module multiplication
    fn mod_mul(r: R, m: Self) -> Self;
}

/// A module homomorphism A -> B
pub trait Homo<R: RingType, A: ModType<R>, B: ModType<R>>: Fn(A) -> B + AbGroupType {}

//composing two functions. maybe this will be useful, maybe not.
fn compose<A, B, C, G, F>(g: G, f: F) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x)) //'move' captures environment of closure
}
