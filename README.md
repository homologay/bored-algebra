# Bored Algebra

This is a math library, broadly about "generically making new rings from rings". 

## Status

Early, early stages. Once the goals and core features are complete I will make a `CONTRIBUTING.md`. 

## Features in progress

* Polynomials over a ring
* Z/nZ (to be subsumed by quotient rings once those are completed)

## Planned features

*Core* 
* Using traits for algebraic structures from `alga` or `num-traits` instead of bespoke ones
* Finitely generated (f.g.) ideals
* Quotient rings
* PIDs (principal ideal domains)
* UFDs (unique factorization domains)
* Euclidean domains
* Localization of a commutative ring at a f.g. ideal
* (co)limits of groups/rings
* (co)products of groups/rings
* Modules
* Tensor products
* Gr'\0xd6'bner bases
* Iterator implementations for the above
*Feature-gated*
* `rand` integration to generate random elements of specific rings
* (nightly) specialization to allow people to reimplement algorithms for their own types
(for example, to use an algorithm optimized for GF(2, 8) instead of the generic
  GF(p, q) algorithm)

## Goals / niche

* An API that is generic and flexible, but not necessarily fast
* Mathematicians should use the big tools, but maybe some of them like rust
* Good documentation that integrates LaTeX and explains the relevant concepts

## FAQ

* Q. Will you consider changing the license?
  A. No.

* Q. Why use an Rc for Polynomial<T>? 
  A. It's taking a performance hit to avoid the spaghetti code that I've seen in order to
  do generic things with references. If you think there's a better way, please let me know!
  The way it is now, to add polynomials and still use them later, just clone them, since 
  it just clones the Rc inside, so is pretty cheap (I think??)  

* Q. Will you make this a crate?
  A. Yes, if it gets useful enough that other people want to work on it