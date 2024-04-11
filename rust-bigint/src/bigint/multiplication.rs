use std::ops::{Mul, MulAssign};
use num_traits::Pow;
use crate::BigUint;

impl Mul<BigUint> for BigUint {
    type Output = BigUint;

    fn mul(self, rhs: BigUint) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&BigUint> for BigUint {
    type Output = BigUint;

    fn mul(self, rhs: &BigUint) -> Self::Output {
        let mut res = crate::biguint::multiplication::ordinary_mul(&self.data, &rhs.data);
        res.fit();
        res
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

impl Pow<u128> for BigUint {
    type Output = BigUint;

    fn pow(self, rhs: u128) -> Self::Output {
        crate::biguint::multiplication::pow_4_window(&self, &crate::biguint::helpers::extract_hex_vec_from_u128(rhs))
    }
}

impl Pow<&BigUint> for BigUint {
    type Output = BigUint;

    fn pow(self, rhs: &BigUint) -> Self::Output {
        crate::biguint::multiplication::pow_4_window(&self, &crate::biguint::helpers::extract_hex_vec_from_biguint(rhs))
    }
}

impl Pow<BigUint> for BigUint {
    type Output = BigUint;

    fn pow(self, rhs: BigUint) -> Self::Output {
        self.pow(&rhs)
    }
}