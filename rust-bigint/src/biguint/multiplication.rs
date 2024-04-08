use crate::biguint::BigUint;
use crate::{Digit, DoubleDigit, BASE, BASE_BIT_MASK, BITS_IN_BASE};
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
        let mut res = ordinary_mul(&self.data, &rhs.data);
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

fn ordinary_mul(lhs: &[Digit], rhs: &[Digit]) -> BigUint {
    let long_mul_one_digit = |slice: &[Digit], m: Digit| -> BigUint {
        let mut carry: DoubleDigit = 0;
        let mut data = Vec::with_capacity(slice.len() + 1);
        for d in slice.iter() {
            let tmp = (*d) as DoubleDigit * m as DoubleDigit + carry;
            data.push((tmp & BASE_BIT_MASK) as Digit);
            carry = tmp >> BITS_IN_BASE;
        }
        if carry != 0 {
            data.push(carry as Digit)
        }
        BigUint { data }
    };

    let mut c = BigUint {
        data: vec![0; lhs.len() * 2],
    };
    for (i, d) in rhs.iter().enumerate() {
        let mut tmp = long_mul_one_digit(lhs, *d);
        tmp <<= i as u128 * BITS_IN_BASE;
        c += tmp;
    }
    c
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
