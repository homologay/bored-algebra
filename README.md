# Bored Algebra

A library for rings, fields, polynomials, and such, built off `alga`'s algebraic traits. 
This project is still in the early phases. Please let me know if you have ideas, as the
feature list is not set in stone.

## Planned Features

**Core**

* Polynomial rings 
* Finitely-generated ideals
* Quotient rings
* Hensel's Lemma
* Localization of a commutative ring
* Tensor products of rings

**Maybe sometime**

* PIDs (principal ideal domains)
* UFDs (unique factorization domains)
* Euclidean domains
* (co)limits

**Feature-gated/Dev**

* `rand` integration to generate random elements of specific rings
* (nightly) specialization to allow people to reimplement algorithms for their own types
(for example, to use an algorithm optimized for GF(2, 8) instead of the generic
  GF(p, q) algorithm)
* `quickcheck` for testing that algebraic structures are what they say they are

## Goals / niche

* An API that is generic and flexible, but not necessarily fast
* Good documentation that integrates LaTeX and explains the relevant concepts

## Status

* Haven't moved to `alga` yet
* There's a passable Z/nZ implementation that will be removed once quotient rings are implemented
* Polynomial arithmetic is in progress