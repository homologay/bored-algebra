//! bored-algebra
//!
//! Scope, so this project doesn't get way out of hand
//!
//!     1. Primarily a library, with a binary just for generation:
//!             
//!                 -primes, with different algorithms
//!                 -polynomials of special kinds: irreducible, cyclotomic, symmetric, etc.
//!     
//!     2. The library component is focused on algebra, specifically
//!
//!                 -finite fields                      -> initial stages                
//!                 -polynomials                        -> mostly work, still initial
//!                 -galois groups                      -> not implemented
//!                 -S_n, A_n                           -> ""
//!                 -D_n                                -> ""
//!                 -direct and semidirect products     -> ""
//!                 -prime/irr testing in rings         -> ""
//!                 -factoring in euclidean rings       -> ""
//!                 -elliptic curves over finite fields -> ""
//!
//!     3. The library specifically will not implement:
//!
//!                 -category theory        ->      possible future project
//!                 -homological algebra    ->      ""
//!                 -topology               ->      idk              
//!                 -matrices or tensors    ->      implemented well elsewhere    
//!                 -complex numbers        ->      ""
//!                 -quaternions            ->      ""
//!                 -rationals              ->      ""
//!                 -geometry               ->      ""
//!                 -floats/ reals          ->      ""
//!                 -probability            ->      ""
//!                 -super optimized stuff  ->      beyond my abliity
//!

#[macro_use]
pub mod macros;
pub mod error;
pub mod finite_fields;
pub mod helpers;
pub mod prime_generation;
pub mod traits;
