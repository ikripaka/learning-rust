use crate::biguint::BigUint;
use std::ops::{Mul, MulAssign};

impl Mul<BigUint> for BigUint {
    type Output = BigUint;

    fn mul(self, rhs: BigUint) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&BigUint> for BigUint {
    type Output = BigUint;

    fn mul(self, rhs: &BigUint) -> Self::Output {
        todo!()
    }
}
impl MulAssign<&BigUint> for BigUint {
    fn mul_assign(&mut self, rhs: &BigUint) {
        let res = self.clone() * rhs;
        self.data = res.data;
    }
}

impl MulAssign<BigUint> for BigUint {
    fn mul_assign(&mut self, rhs: BigUint) {
        let res = self.clone() * &rhs;
        self.data = res.data;
    }
}

fn ordinary_mul(lhs: &BigUint, rhs: &BigUint) -> BigUint {
    todo!()
}

// < 100_000 digits
fn mul_karatsuba(lhs: &BigUint, rhs: &BigUint) -> BigUint {
    todo!()
}

// > 100_000 digits
fn mul_fast_fourier_transform(lhs: &BigUint, rhs: &BigUint) -> BigUint {
    todo!()
}

fn pow(num: &BigUint, power: u64) -> BigUint {
    todo!()
}

fn mod_pow(num: &BigUint, module: &BigUint, power: u64) -> BigUint {
    todo!()
}
