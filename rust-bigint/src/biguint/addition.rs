use crate::biguint::BigUint;
use std::ops::{Add, AddAssign};

impl Add<BigUint> for BigUint {
    type Output = BigUint;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl Add<&BigUint> for BigUint {
    type Output = BigUint;

    fn add(self, rhs: &BigUint) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Self> for BigUint {
    fn add_assign(&mut self, rhs: BigUint) {
        todo!()
    }
}

impl AddAssign<&BigUint> for BigUint {
    fn add_assign(&mut self, rhs: &BigUint) {
        todo!()
    }
}
