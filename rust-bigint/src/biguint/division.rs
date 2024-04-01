use crate::biguint::BigUint;
use std::ops::{Div, DivAssign, Rem, RemAssign};

impl Div<BigUint> for BigUint {
    type Output = BigUint;

    fn div(self, rhs: BigUint) -> Self::Output {
        todo!()
    }
}
impl Div<&BigUint> for BigUint {
    type Output = BigUint;

    fn div(self, rhs: &BigUint) -> Self::Output {
        todo!()
    }
}

impl Rem<BigUint> for BigUint {
    type Output = BigUint;

    fn rem(self, rhs: BigUint) -> Self::Output {
        todo!()
    }
}
impl Rem<&BigUint> for BigUint {
    type Output = BigUint;

    fn rem(self, rhs: &BigUint) -> Self::Output {
        todo!()
    }
}

impl DivAssign<BigUint> for BigUint {
    fn div_assign(&mut self, rhs: BigUint) {
        todo!()
    }
}

impl DivAssign<&BigUint> for BigUint {
    fn div_assign(&mut self, rhs: &BigUint) {
        todo!()
    }
}

impl RemAssign<BigUint> for BigUint {
    fn rem_assign(&mut self, rhs: BigUint) {
        todo!()
    }
}

impl RemAssign<&BigUint> for BigUint {
    fn rem_assign(&mut self, rhs: &BigUint) {
        todo!()
    }
}
