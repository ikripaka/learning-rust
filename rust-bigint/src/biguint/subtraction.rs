use crate::biguint::BigUint;
use std::ops::{Sub, SubAssign};

impl Sub<BigUint> for BigUint {
    type Output = BigUint;

    fn sub(self, rhs: BigUint) -> Self::Output {
        todo!()
    }
}

impl Sub<&BigUint> for BigUint {
    type Output = BigUint;

    fn sub(self, rhs: &BigUint) -> Self::Output {
        todo!()
    }
}

impl SubAssign<BigUint> for BigUint {
    fn sub_assign(&mut self, rhs: BigUint) {
        todo!()
    }
}
impl SubAssign<&BigUint> for BigUint {
    fn sub_assign(&mut self, rhs: &BigUint) {
        todo!()
    }
}
