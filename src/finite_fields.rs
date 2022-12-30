use core::ops::Add;

#[derive(Debug, PartialEq, Clone, Copy)]
struct IntegerModN {
    val: usize, // a representative for the class of val mod n
    n: usize,   // the order of the group
}

impl Add for IntegerModN {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            val: (self.val + rhs.val) % self.n,
            n: match self.n == rhs.n {
                true => self.n,
                false => panic!(),
            },
        }
    }
}

// returns the (additive) order of an element of Z/nZ
fn order(num: IntegerModN) -> usize {
    //todo
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add() {
        let n1 = IntegerModN { val: 3, n: 7 };
        let n2 = IntegerModN { val: 2, n: 7 };
        assert_eq!(n1 + n2, IntegerModN { val: 5, n: 7 }, "valid, no wrap");
        assert_eq!(n1 + n1 + n1, IntegerModN { val: 2, n: 7 }, "valid, wrap");
    }

    #[test]
    #[should_panic]
    fn add_invalid() {
        let n7 = IntegerModN { val: 1, n: 7 };
        let n3 = IntegerModN { val: 1, n: 3 };
        let n_invalid = n7 + n3;
    }
}
