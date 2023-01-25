//! Polynomials
use crate::integers_mod;
use crate::traits::{IntegerModN, One, RingType, Zero};

use std::fmt;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Neg, Sub};

use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;

///A polynomial with coefficients in a ring T, represented as Box<Vec<T>>. The coefficient for
///degree n is the element at index n in the vector. 
#[derive(Debug, Clone)]
struct Polynomial<T: RingType> {
    coeffs: Box<Vec<T>>, //the coefficients in order, places core[0] is x^0, core[1] is x^1, so on.
    deg: u64,
}

//is the "from_" naming convention followed properly here?
impl<T: RingType> Polynomial<T> {
    fn from_vec(vec: Vec<T>) -> Self {
        
        // the degree is the maximum of the indices of nonzero coefficients
        let degree = match vec.iter().rposition(|coeff| *coeff != T::zero()) {
            Some(index) => index as u64,
            None => 0 as u64 ,
        };

        Self {
            coeffs: Box::new(vec),
            deg: degree,
        }
    }

    fn degree(&self) -> u64 {
        todo!(); 
    }

    fn coeffs(&self) -> Box<Vec<T>> {
        todo!();
    }
}

impl<T: RingType> From<Vec<T>> for Polynomial<T> {
    fn from(vec: Vec<T>) -> Self {
        todo!();
    }
}

impl<T: RingType> Into<Vec<T>> for Polynomial<T> {
    fn into(self) -> Vec<T> {
        todo!();
    }
}

/// Two polynomials are considered equal if they have the same degree, and for each index <=
/// degree, the coeffients at that index are equal. 
impl<T: RingType> PartialEq for Polynomial<T> {
    fn eq(&self, other: &Self) -> bool {
        let lhs_deg = self.degree();
        let rhs_deg = other.degree();

        if lhs_deg == rhs_deg {
            //iterate over (elem, index) of each, up to lhs_deg + 1,check all nonzero ones are equal
            return self.coeffs().iter().enumerate().take((lhs_deg + 1).try_into().unwrap()).eq(other
                .coeffs
                .iter()
                .enumerate()
                .take((lhs_deg + 1).try_into().unwrap()));
        } else {
            return false; 
        }
    }

    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }
}

impl<T: RingType> Eq for Polynomial<T> {}

/// The 1 of the polynomial ring is the 1 of its coeffient ring. 
impl<T: RingType> One for Polynomial<T> {
    fn one() -> Self {
        Self::from_vec(vec![T::one()])
    }

    fn is_one(&self) -> bool {
        if *self == Self::one() {
            return true;
        } else {
            return false;
        }
    }
}

/// The 0 of the polynomial ring is the 0 of its coefficient ring. 
impl<T: RingType> Zero for Polynomial<T> {
    fn zero() -> Self {
        Self::from_vec(vec![T::zero()])
    }

    fn is_zero(&self) -> bool {
        if *self == Self::zero() {
            return true;
        } else {
            return false;
        }
    }
}

impl<T: RingType> RingType for Polynomial<T> {}

impl<T: RingType> Add for Polynomial<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        let sum = (*self
            .coeffs())       //*Rc<Vec<T>>
            .iter()         
            .zip_longest((*rhs.coeffs()).iter())
            .map(|elem| match elem {
                Both(&a, &b) => a + b,
                Left(&b) => b,
                Right(&a) => a,
            })
            .collect();

        Self::from_vec(sum)
    }
}

impl<T: RingType> Neg for Polynomial<T> {
    type Output = Self;

    fn neg(self) -> Self {
        let neg_coeffs = self.coeffs().into_iter().map(|elem| -elem).collect::<Vec<T>>();

        Self::from_vec(neg_coeffs)
    }
}

impl<T: RingType> Sub for Polynomial<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

impl<T: RingType> Mul for Polynomial<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        
        //degrees of the polynomials lhs = n, rhs = m
        let lhs_vec = self.coeffs();
        let rhs_vec = rhs.coeffs();
        let n = &lhs_vec.len() - 1;
        let m = &rhs_vec.len() - 1;

        // the formula:
        //\sum_{k=0}^n a_k x^k \sum_{k=0}^m b_k x^k = \sum_{k=0}^{n+m} \sum_{i=0}^k a_i b_{k-i} x^k

        let mut result = Vec::new();

        for k in 0..=(n + m) {
            let mut kth_coeff = T::zero();

            for i in 0..=k {
                match (i > n) || ((k - i) > m) {
                    true => (),
                    false => {
                        kth_coeff = kth_coeff + (lhs_vec[i].clone() * rhs_vec[k - i].clone());
                    }
                }
            }
            result.push(kth_coeff);
        }
        Self::from_vec(result)
    }
}

impl<T: RingType + Div> Div for Polynomial<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: RingType + Rem> Rem for Polynomial<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;
    
    /*
    #[test]
    fn polynomial_arithmetic() {
        //addition and subtraction

        // over Z
        // x + 1
        let z1 = Polynomial::<i64>::from_vec(vec![1, 1]);
        // 2x^2 - 1
        let z2 = Polynomial::<i64>::from_vec(vec![-1, 0, 2]);

        // x + 1 + 2x^2 - 1 = 2x^2 + x
        assert_eq!(
            z1.clone() + z2.clone(),
            Polynomial::<i64>::from_vec(vec![0, 1, 2])
        );

        // x + 1 - (2x^2 - 1) = -2x^2 + x + 2
        assert_eq!(
            z1.clone() - z2.clone(),
            Polynomial::<i64>::from_vec(vec![2, 1, -2])
        );

        // over Z/4Z
        integers_mod!(IntegerMod4, 4);

        // 1 + 3x + 2x^3
        let mod1 = Polynomial::<IntegerMod4>::from_vec(vec![
            IntegerMod4::new(1),
            IntegerMod4::new(3),
            IntegerMod4::new(0),
            IntegerMod4::new(2),
        ]);
        // x + 3
        let mod2 =
            Polynomial::<IntegerMod4>::from_vec(vec![IntegerMod4::new(3), IntegerMod4::new(1)]);
        let mod_sum = mod1.clone() + mod2.clone();
        let mod_diff = mod1.clone() - mod2.clone();

        // 1 + 3x + 2x^3 + x + 3 = 2x^3
        assert_eq!(
            mod_sum,
            Polynomial::<IntegerMod4>::from_vec(vec![
                IntegerMod4::new(0),
                IntegerMod4::new(0),
                IntegerMod4::new(0),
                IntegerMod4::new(2)
            ])
        );

        // 1 + 3x + 2x^3 - (x + 3) = 2 + 2x + 2x^3
        assert_eq!(
            mod_diff,
            Polynomial::<IntegerMod4>::from_vec(vec![
                IntegerMod4::new(2),
                IntegerMod4::new(2),
                IntegerMod4::new(0),
                IntegerMod4::new(2)
            ])
        );

        //multiplication

        // over Z/4Z
        // 1 + 3x + 2x^3
        let mod1 = Polynomial::<IntegerMod4>::from_vec(vec![
            IntegerMod4::new(1),
            IntegerMod4::new(3),
            IntegerMod4::new(0),
            IntegerMod4::new(2),
        ]);
        // x + 3
        let mod2 =
            Polynomial::<IntegerMod4>::from_vec(vec![IntegerMod4::new(3), IntegerMod4::new(1)]);

        let mod_prod = mod1 * mod2;
        let mod_expected = Polynomial::<IntegerMod4>::from_vec(vec![
            IntegerMod4::new(3),
            IntegerMod4::new(2),
            IntegerMod4::new(3),
            IntegerMod4::new(2),
            IntegerMod4::new(2),
        ]);

        // (1 + 3x + 2x^3)(x + 3) = x + 3 + 3x^2 + 9x^2 + 2x^4 + 6x^3
        //                        = 2x^4 + 2x^3 + x + 3
        assert_eq!(mod_prod, mod_expected);
        //got:
    }*/

    /*
    #[test]
    fn multivariable_polynomial_arithmetic() {
        // in Z[x,y]

        let one =
            Polynomial::<Polynomial<i64>>::from_vec(vec![Polynomial::<i64>::from_vec(vec![1])]);
        let zero =
            Polynomial::<Polynomial<i64>>::from_vec(vec![Polynomial::<i64>::from_vec(vec![0])]);

        // R(x,y) = -1 + xy
        // L(x,y) =  1 - xy

        let r = Polynomial::<Polynomial<i64>>::from_vec(vec![
            Polynomial::<i64>::from_vec(vec![-1]),   //ones
            Polynomial::<i64>::from_vec(vec![0, 1]), //x's
        ]);

        let l = Polynomial::<Polynomial<i64>>::from_vec(vec![
            Polynomial::<i64>::from_vec(vec![1]),     //ones
            Polynomial::<i64>::from_vec(vec![0, -1]), //x's
        ]);

        assert_eq!(r + l, zero);

        // P(x,y) = xy + 3x^2 - 4y^2x - 8
        let p =
            Polynomial::<Polynomial<i64>>::from_vec(vec![Polynomial::<i64>::from_vec(vec![-8])]);

        // Q(x,y) = x^3 - 4y + 1

        //indexing confusion ....................................
    }*/

}
