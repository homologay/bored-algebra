//! Modules, homomorphisms
use core::fmt::Debug;
use core::ops::{Add, Mul, Neg, Sub};
pub use num_traits::identities::{one, zero, One, Zero};
use std::marker::PhantomData;
use trait_set::trait_set;

trait_set! {
    /// For rings. If the documentation gets weird here, it's because I'm using `trait-set` to have trait aliases
    /// in stable rust.
    pub trait RingType = AbGroupType + Mul<Output = Self> + One;

    /// For abelian groups. As with `RingType`, this trait is defined as an alias in the `trait_set!` macro.
    pub trait AbGroupType =
        Eq
        + Add<Output = Self>
        + Neg<Output = Self>
        + Sub<Output = Self>
        + Zero
        + Clone
        + Debug;
}

/// For R-modules. Since this library is for commutative rings, it doesn't matter whether it's a left or right
/// module, but we will say left for definitiveness.
pub trait ModType<R: RingType>: AbGroupType {
    /// module multiplication $r \times m$, for $r \in R$ and $m \in M$.
    fn mod_mul(r: R, m: Self) -> Self;
}

/// An R-module homomorphism $A \to B$.
pub struct Homo<R: RingType, A: ModType<R>, B: ModType<R>> {
    ring: PhantomData<R>,
    function: Box<dyn Fn(A) -> B>,
}

impl<R: RingType, A: ModType<R>, B: ModType<R>> Homo<R, A, B> {
    /// Returns the morphism in a way that u can apply elements to it
    pub fn as_fn(self) -> impl Fn(A) -> B {
        todo!();
    }

    /// Compose morphisms `self`$:A \to B$ and `other`$:B \to C$.
    pub fn compose<C: ModType<R>>(self, other: Homo<R, B, C>) -> Homo<R, A, C> {
        todo!();
    }
}

/// Add morphisms `self`, `other$:A \to B$ by elements
impl<R: RingType, A: ModType<R>, B: ModType<R>> Add for Homo<R, A, B> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        todo!();
    }
}

fn compose_functions<A, B, C, G, F>(g: G, f: F) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}
