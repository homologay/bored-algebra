# Bored Algebra

Rings and things.

## Status

Early stages. Not much is implemented yet, and the API changes frequently.

## Motivation

Most abstract algebra crates I have seen are either a complicated system of traits,
or a concrete implementation of one specific thing. This library aims to implement general 
constructions in commutative algebra, using only as many traits and macros as necessary. 
From this we can get a bunch of fun things nearly for free, like finite fields (from quotients, polynomials)
or p-adic numbers (from completions). This 'from' relation is written as '=>' below.

## Documentation

This library is not on `crates.io` yet, so here are the steps to view the documentation. First, clone
the repo:
```sh
git clone https://github.com/maxinebeckie/bored-algebra
cd bored-algebra
```
Then build the documentation with the following flags, so the LaTeX renders properly.
```sh
RUSTDOCFLAGS="--html-in-header src/katex-header.html" cargo doc --no-deps --open
```
This method is from the crate `rustdoc-katex-demo`.  

## Planned features / roadmap

**abstractions**

* modules => rings, abelian groups
* submodules => ideals, subgroups
* module homomorphisms => ring and group homomorphisms

! all rings are commutative in this house ! 

**constructions**

* polynomials
* quotient modules
* free modules
* localization
* tensor products
* arbitrary products => inverse limits => completions
* arbitrary disjoint unions => direct limits => some cursed stuff, apparently

**homological things**

* chain complexes of modules
* homology of a chain complex
* hom functor
* cohomology of a chain complex

**programming things**

* `rand` integration to generate random elements of specific rings
* `quickcheck` for testing that algebraic structures are what they say they are

## Contributing

Suggestions and ideas are always welcome, but I will wait until some core features are implemented 
before accepting pull requests and making a formal `CONTRIBUTING.md`. 