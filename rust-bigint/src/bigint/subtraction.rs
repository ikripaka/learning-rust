use num_traits::Zero;
use std::ops::{Sub, SubAssign};

use crate::bigint::Sign;
use crate::BigInt;

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
        let x = match (self.sign, &rhs.sign) {
            (Sign::Positive, Sign::Negative) => BigInt {
                sign: Sign::Positive,
                data: rhs.data.clone() + &self.data,
            },
            (Sign::Negative, Sign::Positive) => BigInt {
                sign: Sign::Negative,
                data: self.data + &rhs.data,
            },
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
        };
        if x.sign == Sign::Negative && x.data.is_zero() {
            return BigInt::zero();
        }
        x
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
