//! Polynomials
use crate::helpers::mul_z_module; //TODO: move this functionality to ModType implementation
use crate::module::{ModType, RingType};
use std::fmt::Debug;
use std::iter::once;
use std::ops::{Add, Div, Mul, Neg, Rem, Sub};

use num_traits::identities::{One, Zero};

/// A polynomial with coefficients in $R$. To do much with `Polynomial<R>`, `R` should implement `RingType`.
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
#[derive(Debug, Clone)] //TODO: just implement these by hand
pub struct Polynomial<T: Debug + Clone> {
    coeffs: Vec<T>, // index == deg
    deg: u64,
}

impl<R: RingType> Polynomial<R> {
    pub fn deg(&self) -> u64 {
        self.deg
    }
    pub fn coeffs(&self) -> Vec<R> {
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

    /// Returns the first k coefficients of a polynomial.
    pub fn coeffs_take(&self, k: usize) -> Vec<R> {
        self.coeffs.clone().into_iter().take(k).collect()
    }

    /// Evaluation function $ev_a: R[x] \to R$, where $a \in R$.
    pub fn eval(&self, val: R) -> R {
        todo!();
        
        /*
        if val == R::zero() {
            return (*self).coeffs[0].clone();
        }
        let ret = (*self).coeffs.iter().enumerate().collect::<Vec<(R, usize)>>();
        dbg!(&ret);
        ret.into_iter().fold(R::zero(), |acc, (coeff, i)| {
            acc + int_pow((*coeff).clone(), i)
        })
        */
    }

    /// add, assuming rhs has lower or equal degree to self.
    fn add_pad_second(self, rhs: Self) -> Self {
        let pad_len = &self.deg() - &rhs.deg();
        let zero = R::zero();
        let padding = once(&zero).cycle().take(pad_len.try_into().unwrap());
        if pad_len == 0 {
            Polynomial::from(
                self.coeffs()
                    .iter()
                    .zip(rhs.coeffs().iter())
                    .map(|(a, b)| (*a).clone() + (*b).clone())
                    .collect::<Vec<R>>(),
            )
        } else {
            Polynomial::from(
                self.coeffs()
                    .iter()
                    .zip(rhs.coeffs().iter().chain(padding))
                    .map(|(a, b)| (*a).clone() + (*b).clone())
                    .collect::<Vec<R>>(),
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
                .collect::<Vec<R>>(),
        )
    }
}

impl<R: RingType> IntoIterator for Polynomial<R> {
    type Item = R;
    type IntoIter = std::vec::IntoIter<Self::Item>; //i do not understand this

    fn into_iter(self) -> Self::IntoIter {
        self.coeffs().into_iter()
    }
}

/// The default is 0
impl<T: RingType> Default for Polynomial<T> {
    fn default() -> Self {
        Self {
            coeffs: vec![T::zero()],
            deg: 0,
        }
    }
}

impl<T: RingType> ModType<Polynomial<T>> for Polynomial<T> {
    fn mod_mul(lhs: Self, rhs: Self) -> Self {
        todo!();
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
                coeffs: vec,
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
        let self_deg: usize = (&self).deg().try_into().unwrap();
        let rhs_deg: usize = (&rhs).deg().try_into().unwrap();

        match ((&self).is_zero(), (&rhs).is_zero()) {
            (true, _) => Self::zero(),
            (_, true) => Self::zero(),
            _ => {
                match ((&self).is_one(), (&rhs).is_one()) {
                    (true, _) => rhs,
                    (_, true) => self,
                    _ => {
                        // pad both with zeros to index n + m
                        let zero = T::zero();
                        let self_padding = once(&zero).cycle().take(rhs_deg);
                        let rhs_padding = once(&zero).cycle().take(self_deg);

                        let prod_max_index: usize = (self_deg + rhs_deg).try_into().unwrap();

                        // iterate over coefficients, using formula for kth coefficient
                        let prod = (0..=prod_max_index)
                            .map(|k| {
                                self.clone()
                                    .coeffs()
                                    .iter()
                                    .chain(self_padding.clone())
                                    .take(k + 1)
                                    .zip(
                                        rhs.clone()
                                            .coeffs_take(k + 1)
                                            .iter()
                                            .rev()
                                            .chain(rhs_padding.clone()),
                                    )
                                    .map(|(self_i, rhs_k_minus_i)| {
                                        (*self_i).clone() * (*rhs_k_minus_i).clone()
                                    })
                                    .fold(zero.clone(), |acc, elem| acc + elem)
                            })
                            .collect::<Vec<T>>();

                        Self::from(prod)
                    }
                }
            }
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

/// ring element taken to integer power. This should probably be moved elsewhere
pub fn int_pow<R: RingType>(r: R, n: usize) -> R {
    if n == 0 {
        return R::one();
    } else if n == 1 {
        return r;
    }
    let mut ret = r.clone();
    for _ in 0..(n - 1) {
        ret = ret * r.clone();
    }
    ret
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
    #[test]
    fn test_multivariable_add() {
        // in Z[x,y]

        let one = Polynomial::<Polynomial<i64>>::from(vec![Polynomial::<i64>::from(vec![1])]);
        let zero = Polynomial::<Polynomial<i64>>::from(vec![Polynomial::<i64>::from(vec![0])]);

        // R(x,y) = -1 + xy
        // L(x,y) =  1 - xy

        let r = Polynomial::<Polynomial<i64>>::from(vec![
            Polynomial::<i64>::from(vec![-1]),   //ones
            Polynomial::<i64>::from(vec![0, 1]), //x's
        ]);

        let l = Polynomial::<Polynomial<i64>>::from(vec![
            Polynomial::<i64>::from(vec![1]),     //ones
            Polynomial::<i64>::from(vec![0, -1]), //x's
        ]);

        assert_eq!(r + l, zero);

        // P(x,y) = xy + 3x^2 - 4y^2x - 8
        let p = Polynomial::<Polynomial<i64>>::from(vec![Polynomial::<i64>::from(vec![-8])]);

        // Q(x,y) = x^3 - 4y + 1

        //indexing confusion ....................................
    }

    #[test]
    fn test_mul() {
        // over Z

        let one = Polynomial::<i64>::one();
        let zero = Polynomial::<i64>::zero();

        // identities
        assert_eq!(one.clone() * one.clone(), one.clone());
        assert_eq!(zero.clone() * one.clone(), zero.clone());
        assert_eq!(zero.clone() * zero.clone(), zero.clone());
        assert_eq!(one.clone() * zero.clone(), zero.clone());

        let x = Polynomial::<i64>::from(vec![0, 1]);
        assert_eq!(
            Polynomial::<i64>::from(vec![2]) * Polynomial::<i64>::from(vec![3]),
            Polynomial::<i64>::from(vec![6])
        );
        assert_eq!(
            Polynomial::<i64>::from(vec![2]) * x.clone(),
            Polynomial::<i64>::from(vec![0, 2])
        );
        assert_eq!(x.clone() * one.clone(), x.clone());
        let x_squared = Polynomial::<i64>::from(vec![0, 0, 1]);
        let x_cubed = Polynomial::<i64>::from(vec![0, 0, 0, 1]);

        assert_eq!(
            x_cubed.clone() * x_cubed.clone(),
            Polynomial::<i64>::from(vec![0, 0, 0, 0, 0, 0, 1])
        );

        assert_eq!(
            x_squared.clone() * x_squared.clone(),
            Polynomial::<i64>::from(vec![0, 0, 0, 0, 1])
        );

        assert_eq!(
            x.clone() * x_squared.clone(),
            Polynomial::<i64>::from(vec![0, 0, 0, 1])
        );

        //assert_eq!(x.clone() * x.clone(), x_squared.clone());

        // 1 + 3x + 2x^3
        let a = Polynomial::<i64>::from(vec![1, 3, 2, 0, 0]);
        // x + 3
        let b = Polynomial::<i64>::from(vec![3, 1]);

        assert_eq!(Polynomial::<i64>::from(vec![12, 1, 3, 6, 2]), a * b);
    }

    #[test]
    fn test_eval_1() {
        // 1 + 3x + 2x^3
        let a = Polynomial::<i64>::from(vec![1, 3, 0, 2]);
        let t = 4;

        assert_eq!((&a).eval(4), 141);
    }

    #[test]
    fn test_eval_2() {
        // 1 - x + x^2
        let a = Polynomial::<i64>::from(vec![1, -1, 1]);
        let t = 1;
        // should have a(t) = 1 - 1 + 1 = 1
        assert_eq!((&a).eval(1), 1 as i64);
    }

    #[test]
    fn test_eval_3() {
        // 1 + x^3
        let a = Polynomial::<i64>::from(vec![1, 0, 0, 1]);
        assert_eq!((&a).eval(2), 1 + int_pow(2, 3));
    }

    #[test]
    fn test_eval_zero() {
        // 1 + x
        let a = Polynomial::<i64>::from(vec![1, 1]);
        // 1 - x + 2x^2 - 69x^3 + 420x^4
        let b = Polynomial::<i64>::from(vec![1, -1, 2, -69, 420]);
        // a(0) = b(0) = 1
        assert_eq!((&a).eval(0), (&b).eval(0));
        assert_eq!((&a).eval(0), 1);
    }

    #[test]
    ///todo: move this elsewhere
    fn test_int_pow() {
        assert_eq!(int_pow(3, 2), 9);
        assert_eq!(int_pow(1, 32342), 1);
        assert_eq!(int_pow(0, 12313), 0);
        assert_eq!(int_pow(452, 1), 452);
        assert_eq!(int_pow(34213241, 0), 1);
    }
}
