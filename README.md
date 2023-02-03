# Bored Algebra

This is a library, still in its very early stages. The core focus is on implementing
the following:

  * Polynomials over a ring
  * Finitely-generated ideals
  * Quotient rings
  * Principal ideal domains (PIDs)
  * Unique factorization domains (UFDs)
  * Euclidean domains
  * Localization of a commutative ring

With these abstract constructions, we can get the following (and more!) mostly for free.

  * (maybe) field of fractions 
  * Z/nZ
  * Finite fields, ie. GF(p, q)
  * Multivariable polynomials
  * Factorization algorithms that work on many things, like polynomials, integers, ...

## Goals

* Implement the above
* Implement iterator traits for the above
* Integrate with rand to generate random elements of a specific ring
* Follow rust API best practices
* Non-painful interaction with popular math libraries like nalgebra, num-traits, alga, ...
* Turn this from an exercise in macros and generics into something
  actually useful. Eventually! Maybe!

## Non-goals

* Be an application
* Be fast
* Be cryptographically secure
* Get deep into the algebra traits business - ideally traits should be hardcoded unless
  trait crates like num-traits, alga, are a dependency on the user's project (is this possible?) 

## Status

* Prime generation works and is very slow
* Polynomials are being reworked (implementation 3)
* There's a passable implementation of Z/nZ that will be removed once quotient rings are implemented
* Euclidean algorithm is being worked on
