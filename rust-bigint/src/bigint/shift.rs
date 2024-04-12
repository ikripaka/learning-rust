use std::ops::{Shl, ShlAssign, Shr, ShrAssign};
use crate::{BigInt};

impl ShlAssign<u128> for BigInt {
    fn shl_assign(&mut self, rhs: u128) {
        self.data = self.data.clone() << rhs;
    }
}
impl ShlAssign<u64> for BigInt {
    fn shl_assign(&mut self, rhs: u64) {
        self.data = self.data.clone() << rhs;
    }
}
impl ShlAssign<u32> for BigInt {
    fn shl_assign(&mut self, rhs: u32) {
        self.data = self.data.clone() << rhs;
    }
}
impl ShlAssign<u16> for BigInt {
    fn shl_assign(&mut self, rhs: u16) {
        self.data = self.data.clone() << rhs;
    }
}
impl ShlAssign<u8> for BigInt {
    fn shl_assign(&mut self, rhs: u8) {
        self.data = self.data.clone() << rhs;
    }
}

impl ShrAssign<u128> for BigInt {
    fn shr_assign(&mut self, rhs: u128) {
        self.data = self.data.clone() >> rhs;
    }
}

impl ShrAssign<u64> for BigInt {
    fn shr_assign(&mut self, rhs: u64) {
        self.data = self.data.clone() >> rhs;
    }
}
impl ShrAssign<u32> for BigInt {
    fn shr_assign(&mut self, rhs: u32) {
        self.data = self.data.clone() >> rhs;
    }
}
impl ShrAssign<u16> for BigInt {
    fn shr_assign(&mut self, rhs: u16) {
        self.data = self.data.clone() >> rhs;
    }
}
impl ShrAssign<u8> for BigInt {
    fn shr_assign(&mut self, rhs: u8) {
        self.data = self.data.clone() >> rhs;
    }
}

impl Shl<u128> for BigInt {
    type Output = BigInt;

    fn shl(self, rhs: u128) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data << rhs }
    }
}

impl Shr<u128> for BigInt {
    type Output = BigInt;

    fn shr(self, rhs: u128) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data >> rhs }
    }
}
impl Shl<u64> for BigInt {
    type Output = BigInt;

    fn shl(self, rhs: u64) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data << rhs }
    }
}

impl Shr<u64> for BigInt {
    type Output = BigInt;

    fn shr(self, rhs: u64) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data >> rhs }

    }
}

impl Shl<u32> for BigInt {
    type Output = BigInt;

    fn shl(self, rhs: u32) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data << rhs }
    }
}

impl Shr<u32> for BigInt {
    type Output = BigInt;

    fn shr(self, rhs: u32) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data >> rhs }
    }
}

impl Shl<u16> for BigInt {
    type Output = BigInt;

    fn shl(self, rhs: u16) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data << rhs }
    }
}

impl Shr<u16> for BigInt {
    type Output = BigInt;

    fn shr(self, rhs: u16) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data >> rhs }
    }
}

impl Shl<u8> for BigInt {
    type Output = BigInt;

    fn shl(self, rhs: u8) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data << rhs }
    }
}

impl Shr<u8> for BigInt {
    type Output = BigInt;

    fn shr(self, rhs: u8) -> Self::Output {
        BigInt{ sign: self.sign.clone(), data: self.data >> rhs }
    }
}