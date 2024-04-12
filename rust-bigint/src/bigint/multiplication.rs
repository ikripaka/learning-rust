use std::ops::{Mul, MulAssign};
use num_traits::{One, Pow};
use crate::{BigInt, BigUint};
use crate::bigint::Sign;

impl Mul<BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: BigInt) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&BigInt> for BigInt {
    type Output = BigInt;

    fn mul(self, rhs: &BigInt) -> Self::Output {
        BigInt {
            sign: match (&self.sign, &rhs.sign) {
                (Sign::Positive, Sign::Negative) => Sign::Negative,
                (Sign::Negative, Sign::Positive) => Sign::Negative,
                (_, _) => Sign::Positive,
            },
            data: self.data * &rhs.data,
        }
    }
}

impl MulAssign<&BigInt> for BigInt {
    fn mul_assign(&mut self, rhs: &BigInt) {
        let res = self.clone() * rhs;
        self.sign = res.sign;
        self.data = res.data
    }
}

impl MulAssign<BigInt> for BigInt {
    fn mul_assign(&mut self, rhs: BigInt) {
        let res = self.clone() * &rhs;
        self.sign = res.sign;
        self.data = res.data
    }
}

impl Pow<u128> for BigInt {
    type Output = BigInt;

    fn pow(self, rhs: u128) -> Self::Output {
        BigInt{ sign: {
            if self.sign == Sign::Negative{
                match rhs % 2 == 0 {
                    true => Sign::Positive,
                    false => Sign::Negative,
                }
            }else{
                Sign::Positive
            }
        }, data: self.data.pow(rhs) }
    }
}

impl Pow<&BigUint> for BigInt {
    type Output = BigInt;

    fn pow(self, rhs: &BigUint) -> Self::Output {
        BigInt{ sign: {
            if let Sign::Negative = self.sign{
                match !rhs.is_odd() {
                    true => Sign::Positive,
                    false => Sign::Negative,
                }
            }else{
                Sign::Positive
            }
        }, data: self.data.pow(rhs) }
    }
}

impl Pow<BigUint> for BigInt {
    type Output = BigInt;

    fn pow(self, rhs: BigUint) -> Self::Output {
        self.pow(&rhs)
    }
}

pub(crate) fn pow_mod(a:&BigInt, power: &BigUint, module: &BigUint) -> BigInt{
    BigInt {
        sign: Sign::Positive,
        data: get_positive_number(a, module).pow_mod(power, module),
    }
}

/// **get_positive_number** -- transforms (-a) mod n = (k*n - a) mod n
fn get_positive_number(a: &BigInt, module: &BigUint) -> BigUint{
    if let Sign::Negative = a.sign{
        let k = (a.data.clone() / module) + BigUint::one();
        (k * module) - &a.data
    } else{
        a.data.clone()
    }
}