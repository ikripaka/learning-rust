use std::cmp::Ordering;
use std::ops::{Sub, SubAssign};
use crate::{BigInt, BigUint};
use crate::bigint::Sign;

impl Sub<BigInt> for BigInt {
    type Output = BigInt;

    fn sub(self, rhs: BigInt) -> Self::Output {
        self - &rhs
    }
}

impl Sub<&BigInt> for BigInt {
    type Output = BigInt;

    /// **sub** -- swaps arguments and performs subtraction to minimize errors
    fn sub(self, rhs: &BigInt) -> Self::Output {

        match (self.sign, &rhs.sign) {
            (Sign::Positive, Sign::Negative) => {
                if self.data >= rhs.data {
                    BigInt {
                        sign: Sign::Positive,
                        data: self.data - &rhs.data,
                    }
                } else {
                    BigInt {
                        sign: Sign::Negative,
                        data: rhs.data.clone() + &self.data,
                    }
                }
            }
            (Sign::Negative, Sign::Positive) => {
                    BigInt {
                        sign: Sign::Negative,
                        data: self.data + &rhs.data,
                    }
            }
            (Sign::Positive, Sign::Positive) => {
                if self.data >= rhs.data {
                    BigInt {
                        sign: Sign::Positive,
                        data: self.data - &rhs.data,
                    }
                } else {
                    BigInt {
                        sign: Sign::Negative,
                        data: rhs.data.clone() - &self.data,
                    }
                }
            }
            (Sign::Negative, Sign::Negative) => {
                if self.data >= rhs.data {
                    BigInt {
                        sign: Sign::Negative,
                        data: self.data - &rhs.data,
                    }
                } else {
                    BigInt {
                        sign: Sign::Positive,
                        data: rhs.data.clone() - &self.data,
                    }
                }
            }
        }
    }
}

impl SubAssign<BigInt> for BigInt {
    fn sub_assign(&mut self, rhs: BigInt) {
        let res = self.clone() - &rhs;
        self.data = res.data
    }
}

impl SubAssign<&BigInt> for BigInt {
    fn sub_assign(&mut self, rhs: &BigInt) {
        let res = self.clone() - rhs;
        self.data = res.data
    }
}