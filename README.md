# Bored Algebra

A commutative algebra library. This project is still in the early phases, so the API changes quite rapidly. I wouldn't recommend using it yet.

## Math things that *might* be included at some point

* Traits for rings, groups, modules
* Polynomial rings 
* Power series rings
* Finitely-generated ideals
* Quotient things
* Free things
* Hensel's Lemma
* Localization of a commutative ring
* Tensor products of modules
* PIDs (principal ideal domains)
* UFDs (unique factorization domains)
* Euclidean domains
* (co)limits
* Chain complexes, homology, Tor (not *that* Tor!), Ext

## Programming things that will probably be implemented at some point

* `rand` integration to generate random elements of specific rings
* `quickcheck` for testing that algebraic structures are what they say they are

## Goals / niche

* An API that is generic and flexible, but not necessarily fast
* Good documentation that integrates LaTeX and explains the relevant concepts

## Status

* There's a passable Z/nZ implementation that will be removed once quotient rings are implemented
* Polynomial arithmetic is in progress
* Rounding off traits, and quotient, free objects are next

## About `alga`

I've decided not to build off it for two reasons:
1. It's not being actively developed
2. Since I'm sticking in the realm of *nice* rings, groups, and modules, I don't need most of it