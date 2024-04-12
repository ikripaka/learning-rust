use std::ops::{Add, AddAssign};
use num_traits::Zero;
use crate::{BigInt};
use crate::bigint::Sign;

impl Add<BigInt> for BigInt{
    type Output = BigInt;

    fn add(self, rhs: BigInt) -> Self::Output {
        self + &rhs
    }
}

impl Add<&BigInt> for BigInt{
    type Output = BigInt;

    fn add(self, rhs: &BigInt) -> Self::Output {

        match (&self.sign, &rhs.sign){
            (Sign::Negative, Sign::Positive) => {
                if self.data > rhs.data{
                    BigInt{ sign: Sign::Negative, data: self.data.clone() - &rhs.data }
                }else if self.data < rhs.data{
                    BigInt{ sign: Sign::Positive, data: rhs.data.clone() - &self.data }
                }else{
                    BigInt::zero()
                }
            },
            (Sign::Positive, Sign::Negative) => {
                if self.data > rhs.data{
                    BigInt{ sign: Sign::Positive, data: self.data.clone() - &rhs.data }
                }else if self.data < rhs.data{
                    BigInt{ sign: Sign::Negative, data: rhs.data.clone() - &self.data }
                }else{
                    BigInt::zero()
                }
            },
            _ => BigInt{ sign: self.sign.clone(), data: self.data + rhs.data.clone() }
        }
    }
}

impl AddAssign<&BigInt> for BigInt{
    fn add_assign(&mut self, rhs: &BigInt) {
        let res =  self.clone() + rhs;
        self.sign = res.sign;
        self.data = res.data
    }
}

impl AddAssign<BigInt> for BigInt{
    fn add_assign(&mut self, rhs: BigInt) {
        let res =  self.clone() + &rhs;
        self.sign = res.sign;
        self.data = res.data
    }
}
