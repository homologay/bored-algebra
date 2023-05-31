mod hmm;

use frunk;
use frunk::prelude::*;
use rug;
use rug::{Assign, Integer};

pub struct Natural {
    n: Integer,
}


/// wrapper type for positive `Integer`s. 
impl Natural {
    /// Takes an integer and returns an error if it's not positive
    pub fn new(i: Integer) -> Result<Self, String> {
        let one = Integer::from(1);
        let zero = Integer::from(0);
        let minus_one = Integer::from(-1);
        match i.clone().signum() {
            one  => Ok(Self { n: i }),
            zero => Ok(Self::default()),
            minus_one => Err("error: needs to be positive to be a natrual number".to_owned()),
            _ => Err("unknown error".to_owned()),
        }
    }

    pub fn private_to_integer(self) -> Integer {
        self.n
    }
}

impl From<Natural> for Integer {
    fn from(n: Natural) -> Self {
        n.private_to_integer()
    }
}

impl Default for Natural {
    fn default() -> Self {
        Self {n: Integer::from(0) }
    }
}

/// `n`-fold product of `a`, returns an `HList`
pub fn n_prod<T>(a: T, n: Natural) {
    // the problem here is: can Integer become iterable for Hlist??? can we make
    // Integer into a Vec<u64> s.t. sums to itself?
}
