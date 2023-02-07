//! Polynomials
use crate::helpers::mul_z_module;
use crate::module::RingType;

use std::iter::once;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};
use std::rc::Rc;

use num_traits::identities::{One, Zero};

/// A polynomial with coefficients in `T`.
/// # Example:
/// ```
/// use bored_algebra::poly::Polynomial;
/// use std::rc::Rc;
///
/// let vec = vec![0, 1, 2];
/// // x + 2x^2
/// let p: Polynomial<i64> = Polynomial::from(vec.clone());
/// assert_eq!(p.deg(), 2_u64);
/// assert_eq!(p.coeffs(), Rc::new(vec));
/// ```
#[derive(Debug, Clone)]
pub struct Polynomial<T> {
    coeffs: Rc<Vec<T>>, // index == deg
    deg: u64,
}

impl<T: RingType> Polynomial<T> {
    pub fn deg(&self) -> u64 {
        self.deg
    }
    pub fn coeffs(&self) -> Rc<Vec<T>> {
        self.coeffs.clone()
    }

    pub fn new() -> Self {
        Self::default()
    }

    /// Takes two polynomials. If the first is higher or equal degree, return true. Otherwise, return false.
    pub fn compare_deg(&self, other: &Self) -> bool {
        let self_deg = self.deg();
        let other_deg = other.deg();

        match self_deg >= other_deg {
            true => true,
            false => false,
        }
    }

    /// multiply, assuming rhs has lower or equal degree to self.
    fn mul_pad_second(self, rhs: Self) -> Self {
        todo!();
    }

    /// add, assuming rhs has lower or equal degree to self.
    fn add_pad_second(self, rhs: Self) -> Self {
        let pad_len = &self.deg() - &rhs.deg();
        let zero = T::zero();
        let padding = once(&zero).cycle().take(pad_len.try_into().unwrap());
        if pad_len == 0 {
            Polynomial::from(
                self.coeffs()
                    .iter()
                    .zip(rhs.coeffs().iter())
                    .map(|(a, b)| (*a).clone() + (*b).clone())
                    .collect::<Vec<T>>(),
            )
        } else {
            Polynomial::from(
                self.coeffs()
                    .iter()
                    .zip(rhs.coeffs().iter().chain(padding))
                    .map(|(a, b)| (*a).clone() + (*b).clone())
                    .collect::<Vec<T>>(),
            )
        }
    }

    /// The derivative of a polynomial, given by the formula
    /// $$
    /// \frac{d}{dx} \sum\_{i=0}^{n}a\_i x^i = \sum_{i=1}^{n} i a\_i x^{i-1}
    /// $$
    /// where $i a\_i = a\_i + ... + a\_i$, with the addition being performed $n$ times.
    pub fn derivative(self) -> Self {
        Polynomial::from(
            self.coeffs()
                .iter()
                .enumerate()
                .skip(1)
                .map(|(n, a)| mul_z_module(n.try_into().unwrap(), (*a).clone()))
                .collect::<Vec<T>>(),
        )
    }
}

impl<T> Default for Polynomial<T> {
    fn default() -> Self {
        Self {
            coeffs: Rc::new(Vec::new()),
            deg: 0,
        }
    }
}

impl<T> From<Vec<T>> for Polynomial<T>
where
    T: RingType,
{
    fn from(vec: Vec<T>) -> Self {
        // if vec is empty, return 0 polynomial
        if vec.is_empty() {
            Self::default()
        } else {
            // otherwise, the degree is the maximum of the indices of nonzero coefficients
            let degree = match vec.iter().rposition(|coeff| *coeff != T::zero()) {
                Some(index) => index as u64,
                None => 0 as u64,
            };

            Self {
                coeffs: Rc::new(vec),
                deg: degree,
            }
        }
    }
}

/// Two polynomials are equal if they have the same degree and all coefficients <= deg
/// are equal.
impl<T: RingType> PartialEq for Polynomial<T> {
    fn eq(&self, other: &Self) -> bool {
        let lhs_deg = self.deg();
        let rhs_deg = other.deg();

        if lhs_deg == rhs_deg {
            //iterate over (elem, index) of each, up to lhs_deg + 1,check all nonzero ones are equal
            return self
                .coeffs()
                .iter()
                .enumerate()
                .take((lhs_deg + 1).try_into().unwrap())
                .eq(other
                    .coeffs()
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
        Self::from(vec![T::one()])
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
        Self::from(vec![T::zero()])
    }

    fn is_zero(&self) -> bool {
        if *self == Self::zero() {
            return true;
        } else {
            return false;
        }
    }
}

/// Polynomial addition. The notation in the code follows the formula
/// $$
/// \sum\_{i=0}^n a\_i x^i + \sum\_{j=0} b\_j x^j = \sum\_{i=0}^{\max(n,m)} (a\_i + b\_i) x^i
/// $$
/// where coefficients beyond the degree of the polynomial are taken to be zero.
impl<T: RingType> Add for Polynomial<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        if Self::compare_deg(&self, &rhs) {
            // pad rhs
            Self::add_pad_second(self, rhs)
        } else {
            // pad self
            Self::add_pad_second(rhs, self)
        }
    }
}

impl<T: RingType> Neg for Polynomial<T> {
    type Output = Self;

    fn neg(self) -> Self {
        Polynomial::from(
            self.coeffs()
                .iter()
                .map(|a| -((*a).clone()))
                .collect::<Vec<T>>(),
        )
    }
}

impl<T: RingType> Sub for Polynomial<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        self + (-rhs)
    }
}

/// Polynomial multiplication. The notation in the code follows the formula
/// $$
/// \Big(\sum\_{i=0}^n a\_i x^i\Big)\Big(\sum\_{j=0}^m b\_j x^j\Big) =
/// \sum\_{k=0}^{n+m} \sum\_{i = 0}^k a\_i b\_{k-i} x^k
/// $$
impl<T: RingType> Mul for Polynomial<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        if Self::compare_deg(&self, &rhs) {
            // pad rhs
            Self::mul_pad_second(self, rhs)
        } else {
            // pad self
            Self::mul_pad_second(rhs, self)
        }
    }
}

impl<T: RingType> Div for Polynomial<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        todo!();
    }
}

impl<T: RingType> Rem for Polynomial<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self {
        todo!();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_from() {
        let vec1 = vec![1, 2, 3, 4, 0, 0, 0, 0, 0, 0];
        let vec2 = vec![1, 2, 3, 4];
        let vec3 = vec![1, 2, 3, 4, 0, 1];

        let poly1 = Polynomial::from(vec1);
        let poly2 = Polynomial::from(vec2);
        let poly3 = Polynomial::from(vec3);

        assert_eq!(poly1, poly2);
        assert_ne!(poly1, poly3);
        assert_ne!(poly2, poly3);
    }

    #[test]
    fn test_compare_deg() {
        //x + x^2 over Z
        let a = Polynomial::<i64>::from(vec![0, 1, 1]);
        //1 + x over Z, with a bunch of extra zeros
        let b = Polynomial::<i64>::from(vec![1, 1, 0, 0, 0, 0]);
        assert_eq!(Polynomial::compare_deg(&a, &b), true);
    }

    #[test]
    fn test_neg() {
        //x + x^2 over Z
        let a = Polynomial::<i64>::from(vec![0, 1, 1]); //
        assert_eq!(-a, Polynomial::<i64>::from(vec![0, -1, -1]));
    }

    #[test]
    fn test_sub() {
        //x + x^2 over Z
        let a = Polynomial::<i64>::from(vec![0, 1, 1]);
        //1 + x over Z, with a bunch of extra zeros
        let b = Polynomial::<i64>::from(vec![1, 1, 0, 0, 0, 0]);
        assert_eq!(a - b, Polynomial::<i64>::from(vec![-1, 0, 1]));
    }

    #[test]
    fn test_add() {
        // over Z
        // x + 1
        let z1 = Polynomial::<i64>::from(vec![1, 1]);
        // 2x^2 - 1
        let z2 = Polynomial::<i64>::from(vec![-1, 0, 2]);

        // x + 1 + 2x^2 - 1 = 2x^2 + x
        assert_eq!(
            z1.clone() + z2.clone(),
            Polynomial::<i64>::from(vec![0, 1, 2])
        );

        // x + 1 - (2x^2 - 1) = -2x^2 + x + 2
        assert_eq!(
            z1.clone() - z2.clone(),
            Polynomial::<i64>::from(vec![2, 1, -2])
        );
    }

    #[test]
    fn test_derivative() {
        // over Z
        // x + 1
        let z1 = Polynomial::<i64>::from(vec![1, 1]);
        // 2x^2 - 1
        let z2 = Polynomial::<i64>::from(vec![-1, 0, 2]);

        assert_eq!(z1.derivative(), Polynomial::<i64>::from(vec![1]));
        assert_eq!(z2.derivative(), Polynomial::<i64>::from(vec![0, 4]));
    }
    /*
    #[test]
    fn test_mul() {
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
    }
    */
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
