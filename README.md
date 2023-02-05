# Bored Algebra

This is a math library about "generically making new rings from rings". 

## Status

Early, early stages. Once the goals and core features are complete I will make a `CONTRIBUTING.md`. 

**Core**

* Polynomial rings 
* Ideals (finitely-generated)
* Quotient rings
* Localization of a commutative ring
* Tensor products of rings
* Property testing (with `quickcheck` or `proptest`)
* Using traits for algebraic structures from `alga` or `num-traits` instead of bespoke ones

**Maybe sometime**

* PIDs (principal ideal domains)
* UFDs (unique factorization domains)
* Euclidean domains
* (co)limits

**Feature-gated**

* `rand` integration to generate random elements of specific rings
* (nightly) specialization to allow people to reimplement algorithms for their own types
(for example, to use an algorithm optimized for GF(2, 8) instead of the generic
  GF(p, q) algorithm)

## Goals / niche

* An API that is generic and flexible, but not necessarily fast
* Good documentation that integrates LaTeX and explains the relevant concepts

## Status

* There's a passable Z/nZ implementation that will be removed once quotient rings are implemented
* Polynomial arithmetic is in progress