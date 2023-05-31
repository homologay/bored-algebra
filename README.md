# Bored Algebra

## ! very incomplete ! not for anything but educational purposes at this time! (and not even that, really)

## What's here

- Incomplete traits for modules, commutative rings, abelian groups, homomorphisms
- Incomplete polynomial ring implementation
- Some random algorithms based on `rug`'s numbers
- A lot of sketches for future ideas that may or may not happen
- Some tests of `frunk` to see if it would be a useful library to use

## What's planned

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

## Code structure

There are currently three crates:

`mod_r` is the central crate consisting of abstractions around modules over a commutative ring

`numnumnum` is number theory algorithms

`catty` is category theory stuff, mostly with `frunk`

## Documentation

This library is not on `crates.io`, so here are the steps to view the documentation. First, clone
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

## Contributing

Pretty early stages for that, but I'd take any feedback you may have :)