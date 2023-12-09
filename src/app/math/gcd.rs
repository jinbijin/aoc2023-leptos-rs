use std::ops::{AddAssign, Div, Mul, RemAssign, Sub};
use super::traits::{zero::Zero, one::One};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct GcdResult<T> {
    pub lhs_inverse: T,
    pub rhs_inverse: T,
    pub gcd: T,
}

pub trait Gcd: Sized {
    type Scalar;

    fn gcd(self, rhs: Self) -> GcdResult<Self>;
}

impl<T, Scalar> Gcd for T
where
    T: Zero,
    T: One,
    T: AddAssign<T>,
    T: Sub<T, Output = T>,
    for<'a, 'b> &'a Scalar: Mul<&'b T, Output = T>,
    for<'a, 'b> &'a T: Div<&'b T, Output = Scalar>,
    T: for<'a> RemAssign<&'a T>,
{
    type Scalar = Scalar;

    fn gcd(self, rhs: T) -> GcdResult<T> {
        let mut pos_inv_1 = T::one();
        let mut pos_inv_2 = T::zero();
        let mut pos = self;

        let mut neg_inv_1 = T::zero();
        let mut neg_inv_2 = T::one();
        let mut neg = rhs;

        loop {
            // then pos >= neg
            if neg.is_zero() {
                return GcdResult {
                    lhs_inverse: pos_inv_1,
                    rhs_inverse: if pos_inv_2.is_zero() { pos_inv_2 } else { neg_inv_2 - pos_inv_2 },
                    gcd: pos
                };
            }
            let factor: Scalar = &pos / &neg;

            pos_inv_1 += &factor * &neg_inv_1;
            pos_inv_2 += &factor * &neg_inv_2;
            pos %= &neg;

            // then neg >= pos
            if pos.is_zero() {
                return GcdResult {
                    lhs_inverse: if neg_inv_1.is_zero() { neg_inv_1 } else { pos_inv_1 - neg_inv_1 },
                    rhs_inverse: neg_inv_2,
                    gcd: neg
                };
            }
            let factor: Scalar = &neg / &pos;

            neg_inv_1 += &factor * &pos_inv_1;
            neg_inv_2 += &factor * &pos_inv_2;
            neg %= &pos;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_return() {
        let result = 3.gcd(24);

        assert_eq!(result, GcdResult { lhs_inverse: 1, rhs_inverse: 0, gcd: 3 });
    }
}