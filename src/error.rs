use thiserror::Error;
use anyhow;

#[derive(Error, Debug)]
pub enum ArithmeticError {
    #[error("divide by zero")]
    DivideByZero,
    #[error("number not in valid range")]
    OutOfBounds,
    #[error("algorithm {alg:?} is not defined for element {el:?}")]
    AlgNotApplicable {
        alg: String,
        el: u64,
    },
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}
