# Bored Algebra

Commutative algebra in rust.

This project is still in the early phases, so the API changes quite rapidly. 
I wouldn't recommend using it yet.

## Motivation

From browsing `crates.io` and `lib.rs`, it seems algebra crates are either 
a complicated system of traits, or a concrete implementation of one specific thing. 
This library aims to implement general constructions in commutative algebra, using only as
many traits and macros as necessary. From this we can get a bunch of fun things, like
finite fields (from quotients, polynomials) or p-adic numbers (from completions). This 
'from' relation is written as '=>' below.

## Planned features / roadmap

**abstractions**

* rings
* modules
* abelian groups

! all rings are commutative in this house ! 

**constructions**

* polynomials
* quotient things
* free things
* localization
* tensor products of modules
* products => inverse limits => completions
* disjoint unions => direct limits => some cursed stuff, apparently

**homological things**

* chain complexes of modules
* homology of a chain complex
* hom functor
* cohomology of a chain complex

**programming things**

* `rand` integration to generate random elements of specific rings
* `quickcheck` for testing that algebraic structures are what they say they are

## Status

* not much so far
* current work is on abstractions and polynomial arithmetic
