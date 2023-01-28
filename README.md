# Bored Algebra

This is a library, still in its very early stages. The core focus is on implementing
the following:

  * Polynomials over a ring
  * Ideals, limiting to the finitely-generated case
  * Quotient rings
  * UFD's, PID's, Euclidean domains
  * Localization of a commutative ring

With the abstract constructions, we can get the following (and more!) mostly for free:

  * Finite fields
  * Rational function fields
  * Integers modulo a number
  * Rational numbers
  * Multivariable polynomials

## Goals

* Implement the above
* Implement iterator traits for the above
* Integrate with rand to generate random elements of a specific ring
* Follow rust API best practices
* Non-painful interaction with popular math libraries like nalgebra, num-traits, ..
* Turn this from an exercise in macros and generics into something
  actually useful. Eventually! Maybe!

## Non-goals

* Be an application
* Be fast
* Be cryptographically secure
* Get deep into the algebra traits business -- there's plenty of good
  crates for that, like alga. 

## Status

* Prime generation works and is very slow
* Polynomials are being reworked (implementation 3 I think)
* There's a passable implementation of Z/nZ
* Will probably remove it once quotient rings are implemented, though. 
* Euclidean algorithm is being worked on
* Using proc macros for quotients and localization is probably the best bet..
