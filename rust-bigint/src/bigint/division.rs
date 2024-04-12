use num_traits::Zero;
use std::ops::{Div, DivAssign, Rem, RemAssign};

use crate::bigint::Sign;
use crate::BigInt;

impl Div<BigInt> for BigInt {
    type Output = BigInt;

    fn div(self, rhs: BigInt) -> Self::Output {
        self / &rhs
    }
}

impl Div<&BigInt> for BigInt {
    type Output = BigInt;

    fn div(self, rhs: &BigInt) -> Self::Output {
        let x = BigInt {
            sign: match (&self.sign, &rhs.sign) {
                (Sign::Positive, Sign::Negative) => Sign::Negative,
                (Sign::Negative, Sign::Positive) => Sign::Negative,
                (Sign::Positive, Sign::Positive) => Sign::Positive,
                (Sign::Negative, Sign::Negative) => Sign::Positive,
            },
            data: self.data / &rhs.data,
        };
        if x.sign == Sign::Negative && x.data.is_zero() {
            return BigInt::zero();
        }
        x
    }
}

impl Rem<BigInt> for BigInt {
    type Output = BigInt;

    fn rem(self, rhs: BigInt) -> Self::Output {
        self % &rhs
    }
}

impl Rem<&BigInt> for BigInt {
    type Output = BigInt;

    fn rem(self, rhs: &BigInt) -> Self::Output {
        let x = BigInt {
            sign: match (&self.sign, &rhs.sign) {
                (Sign::Positive, Sign::Negative) => Sign::Negative,
                (Sign::Negative, Sign::Positive) => Sign::Positive,
                (Sign::Positive, Sign::Positive) => Sign::Positive,
                (Sign::Negative, Sign::Negative) => Sign::Negative,
            },
            data: self.data % &rhs.data,
        };
        if x.sign == Sign::Negative && x.data.is_zero() {
            return BigInt::zero();
        }
        x
    }
}

impl DivAssign<BigInt> for BigInt {
    fn div_assign(&mut self, rhs: BigInt) {
        let res = self.clone() / &rhs;
        self.data = res.data
    }
}

impl DivAssign<&BigInt> for BigInt {
    fn div_assign(&mut self, rhs: &BigInt) {
        let res = self.clone() / rhs;
        self.data = res.data
    }
}

impl RemAssign<BigInt> for BigInt {
    fn rem_assign(&mut self, rhs: BigInt) {
        let res = self.clone() % &rhs;
        self.data = res.data
    }
}

impl RemAssign<&BigInt> for BigInt {
    fn rem_assign(&mut self, rhs: &BigInt) {
        let res = self.clone() % rhs;
        self.data = res.data
    }
}
