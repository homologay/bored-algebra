//! Modules, homomorphisms
//!
//! The main thing here is the trait `ModType<R>`. There are some notable specific cases:
//!     `ModType<Self>`        for rings, and
//!     `ModType<BigInt>`      for abelian groups.
//!
//! Note, it is possible to implement `ModType<R>` for something that isn't an `R`-module. The trait
//! can't be implemented arbitrarily, as it builds off of the `AbGroupType` and `RingType` traits,
//! which are aliases that ensure the relevant operator overloads are there as well as some
//! std traits like `Debug` to make working with them easier. These are done in stable rust thanks to
//! the `trait-set` crate (this does make the documentation around `RingType` and `AbGroupType` a bit
//! weird).  
//!
//! The user-defined type could fail one or more of the module axioms.
//! At this time it is not know what behavior that will cause when using the constructions
//! generic over `R`-modules.
//!
//! TODO: using quickcheck to check a user's type satisfies module axioms.
//!
//! TODO: making a ModType<Ring = Self> automatically implement ModType<Ring = BigInt>
//! (but does this need specialization................uuuhhhhhhh)
use core::fmt::Debug;
use core::ops::{Add, Mul, Neg, Sub};
pub use num_traits::identities::{one, zero, One, Zero};
use std::marker::PhantomData;
use std::rc::Rc;
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
pub trait ModType<R>: AbGroupType {
    /// module multiplication $r \times m$, for $r \in R$ and $m \in M$.
    fn mod_mul(r: R, m: Self) -> Self;
}

/// An R-module homomorphism $A \to B$.
pub struct Homo<R: RingType, A: ModType<R>, B: ModType<R>> {
    ring: PhantomData<R>,
    function: Rc<dyn Fn(A) -> B>,
}

impl<R: RingType, A: ModType<R>, B: ModType<R>> Homo<R, A, B> {
    /// Returns the morphism in a way that u can apply elements to it
    pub fn as_fn(self) -> Rc<dyn Fn(A) -> B> {
        self.function.clone()
    }
/*
    /// Compose morphisms `self`$:A \to B$ and `other`$:B \to C$.
    pub fn compose<C: ModType<R>>(self, other: Homo<R, B, C>) -> Homo<R, A, C> {
        Self {
            ring: self.ring,
            function: Rc::new(compose_rcs_of_functions(self.function, other.function)),
        }
    }
    */
}

/// Add morphisms `self`, `other$:A \to B$ by elements
impl<R: RingType, A: ModType<R>, B: ModType<R>> Add for Homo<R, A, B> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self 
    {
        todo!();
    }
}

fn compose_rcs_of_functions<A, B, C, G, F>(g: Rc<dyn Fn(B) -> C>, f: Rc<dyn Fn(A) -> B>) -> Rc<dyn Fn(A) -> C> {
    // this should be something else...
    Rc::new(move |x| g(f(x)))     
}
