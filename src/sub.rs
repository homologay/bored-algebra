//! Submodules

use crate::module::{ModType, RingType};

/// An $R$-submodule of $M\in \text{Mod}(R)$.
pub trait Sub<R: RingType, M: ModType<R>>: ModType<R> {
    todo!();

    // it's possible a more general implementation could extend Into<ModType<R>> instead of ModType<R>,
    // to account for situations where mathematicians say things like "consider the image of this
    // homomorphism as a submodule of blah ...."

    // It's also possible this could be too permissive. Instead, could make ways to consider
    // the image of a homomorphism as a submodule or things like that ..... idk
}
