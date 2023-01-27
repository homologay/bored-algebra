//! Euclidean domains.


/// (extended) Euclidean Algorithm: returns gcd of two elements of a ring.
pub fn euclid_alg<T: RingType + Rem<Output = T>>(first: T, second: T) -> T {
   todo!(); 
}

/// (extended) Euclidean algorithm that borrows.
pub fn euclid_alg_by_ref<'a, T: RingType + Rem<Output = T> + 'a>(first: &'a T, second: &'a T) -> T
where
    &'a T: Rem<Output = T>,
{
    todo!();
}
