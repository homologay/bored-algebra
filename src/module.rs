use core::fmt::Debug;
use core::ops::{Add, Mul, Neg, Sub};
pub use num_traits::identities::{one, zero, One, Zero};
use std::marker::PhantomData;
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
pub struct Homo<R: RingType, A: ModType<R>, B: ModType<R>> {
    ring: PhantomData<R>,
    function: Box<dyn Fn(A) -> B>,
}

impl<R: RingType, A: ModType<R>, B: ModType<R>> Homo<R, A, B> {}

impl<R: RingType, A: ModType<R>, B: ModType<R>> Add for Homo<R, A, B> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

fn compose<R, A, B, C>(g: Homo<R, B, C>, f: Homo<R, A, B>) -> Homo<R, A, C>
where
    R: RingType,
    A: ModType<R>,
    B: ModType<R>,
    C: ModType<R>,
{
    todo!();
}

//composing two functions. maybe this will be useful, maybe not.
fn compose_functions<A, B, C, G, F>(g: G, f: F) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x)) //'move' captures environment of closure
}
