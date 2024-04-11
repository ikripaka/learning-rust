use std::ops::{Sub, SubAssign};
use crate::BigUint;

impl Sub<BigUint> for BigUint {
    type Output = BigUint;

    fn sub(self, rhs: BigUint) -> Self::Output {
        self - &rhs
    }
}

impl Sub<&BigUint> for BigUint {
    type Output = BigUint;

    /// **sub** -- swaps arguments and performs subtraction to minimize errors
    fn sub(self, rhs: &BigUint) -> Self::Output {
        assert!(
            !(self < *rhs),
            "Subtraction overflow, lhs is bigger than rhs"
        );
        let data = if self > *rhs {
            crate::biguint::subtraction::sub(&self.data, &rhs.data)
        } else {
            vec![0]
        };
        let mut n = BigUint { data };
        n.fit();
        n
    }
}

impl SubAssign<BigUint> for BigUint {
    fn sub_assign(&mut self, rhs: BigUint) {
        let res = self.clone() - rhs;
        self.data = res.data
    }
}
impl SubAssign<&BigUint> for BigUint {
    fn sub_assign(&mut self, rhs: &BigUint) {
        let res = self.clone() - rhs;
        self.data = res.data
    }
}