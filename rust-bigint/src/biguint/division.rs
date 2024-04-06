use crate::biguint::BigUint;
use std::ops::{Div, DivAssign, Rem, RemAssign};

impl Div<BigUint> for BigUint {
    type Output = BigUint;

    fn div(self, rhs: BigUint) -> Self::Output {
        self / &rhs
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
        self % &rhs
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
        let res = self.clone() / &rhs;
        self.data = res.data
    }
}

impl DivAssign<&BigUint> for BigUint {
    fn div_assign(&mut self, rhs: &BigUint) {
        let res = self.clone() / rhs;
        self.data = res.data
    }
}

impl RemAssign<BigUint> for BigUint {
    fn rem_assign(&mut self, rhs: BigUint) {
        let res = self.clone() % &rhs;
        self.data = res.data
    }
}

impl RemAssign<&BigUint> for BigUint {
    fn rem_assign(&mut self, rhs: &BigUint) {
        let res = self.clone() % rhs;
        self.data = res.data
    }
}
