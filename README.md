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
relation is written as '=>' below.

## Math things that will be included at some point

* Traits for groups, (commutative) rings, modules, subgroups, ideals, submodules
* Polynomials
* Quotient things
* Free things
* Localization of a commutative ring
* Tensor products of modules
* inverse limits => 

* (co)limits
* Chain complexes, homology, Tor (not *that* Tor!), Ext

## Programming things that will probably be implemented at some point

* `rand` integration to generate random elements of specific rings
* `quickcheck` for testing that algebraic structures are what they say they are

## Status

* There's a passable Z/nZ implementation that will be removed once quotient rings are implemented
* Polynomial arithmetic is in progress
* Rounding off traits, and quotient, free objects are next

## About `alga`

I've decided not to build off it for two reasons:
1. It's not being actively developed
2. Since I'm sticking in the realm of *nice* rings, groups, and modules, I don't need most of it