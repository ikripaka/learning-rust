use std::ops::{Div, DivAssign, Rem, RemAssign};
use crate::{BigInt, BigUint};
use crate::bigint::Sign;

impl Div<BigInt> for BigInt {
    type Output = BigInt;

    fn div(self, rhs: BigInt) -> Self::Output {
        self / &rhs
    }
}

impl Div<&BigInt> for BigInt {
    type Output = BigInt;

    fn div(self, rhs: &BigInt) -> Self::Output {
        BigInt{ sign:  match (&self.sign, &rhs.sign){
            (Sign::Positive, Sign::Negative) => Sign::Negative,
            (Sign::Negative, Sign::Positive) => Sign::Negative,
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            (Sign::Negative, Sign::Negative) => Sign::Positive,
        }, data: self.data / &rhs.data }
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
        BigInt{ sign:match (&self.sign, &rhs.sign){
            (Sign::Positive, Sign::Negative) => Sign::Negative,
            (Sign::Negative, Sign::Positive) => Sign::Positive,
            (Sign::Positive, Sign::Positive) => Sign::Positive,
            (Sign::Negative, Sign::Negative) => Sign::Negative,
        }, data: self.data % &rhs.data }
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