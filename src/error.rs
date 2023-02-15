//! Error types
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ArithmeticError {
    #[error("divide by zero")]
    DivideByZero,
    #[error("number not in valid range")]
    OutOfBounds,
    #[error("algorithm {alg:?} is not defined for element {el:?}")]
    AlgNotApplicable { alg: String, el: u64 },
    //future idea:
    //
    //    there are many cases where the user claims an operation satisfies a certain property, like
    //    associativity, commutativity, but their implementation may not. Make errors for inferences/
    //    computations where one or more of those properties are assumed but user implementations
    //    could fail.
}
