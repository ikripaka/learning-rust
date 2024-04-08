use crate::{BigUint, Digit, BASE_BIT_MASK, BITS_IN_BASE};
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

pub fn long_shift_left(slice: &[Digit], shift: u128) -> Vec<Digit> {
    let digit_offset = shift % BITS_IN_BASE;

    let mut res = vec![0; ((shift - digit_offset) / BITS_IN_BASE) as usize];
    let mut tmp = 0;
    // shift bits to certain positions
    for digit in slice.iter() {
        let n = ((*digit as u128) << digit_offset) | tmp;
        tmp = n >> BITS_IN_BASE;
        res.push((n & BASE_BIT_MASK) as u64);
    }
    // flush tmp value
    if tmp != 0 {
        res.push(tmp as u64)
    }
    res
}

pub fn long_shift_right(slice: &[Digit], shift: u128) -> Vec<Digit> {
    let (digit_offset, mut tmp) = (shift % BITS_IN_BASE, 0);
    let digits_to_skip = (shift - digit_offset) / BITS_IN_BASE;
    let mut res = Vec::with_capacity(slice.len() - digits_to_skip as usize);

    for (i, d) in slice[digits_to_skip as usize..].iter().enumerate() {
        let mut n = 0;
        if i == 0 {
            tmp = (*d as u128) >> digit_offset;
            continue;
        }
        n = (*d as u128) << (BITS_IN_BASE - digit_offset);
        tmp = n | tmp;
        res.push((tmp & BASE_BIT_MASK) as u64);
        tmp >>= BITS_IN_BASE;
    }
    res.push(tmp as u64);

    res
}

impl ShlAssign<u128> for BigUint {
    fn shl_assign(&mut self, rhs: u128) {
        let data = long_shift_left(&self.data, rhs);
        self.data = data;
        self.fit()
    }
}
impl ShlAssign<u64> for BigUint {
    fn shl_assign(&mut self, rhs: u64) {
        let data = long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShlAssign<u32> for BigUint {
    fn shl_assign(&mut self, rhs: u32) {
        let data = long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShlAssign<u16> for BigUint {
    fn shl_assign(&mut self, rhs: u16) {
        let data = long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShlAssign<u8> for BigUint {
    fn shl_assign(&mut self, rhs: u8) {
        let data = long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}

impl ShrAssign<u128> for BigUint {
    fn shr_assign(&mut self, rhs: u128) {
        let data = long_shift_left(&self.data, rhs);
        self.data = data;
        self.fit()
    }
}

impl ShrAssign<u64> for BigUint {
    fn shr_assign(&mut self, rhs: u64) {
        let data = long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShrAssign<u32> for BigUint {
    fn shr_assign(&mut self, rhs: u32) {
        let data = long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShrAssign<u16> for BigUint {
    fn shr_assign(&mut self, rhs: u16) {
        let data = long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShrAssign<u8> for BigUint {
    fn shr_assign(&mut self, rhs: u8) {
        let data = long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}

impl Shl<u128> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u128) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_left(&self.data, rhs),
        };
        n.fit();
        n
    }
}

impl Shr<u128> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u128) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_right(&self.data, rhs),
        };
        n.fit();
        n
    }
}
impl Shl<u64> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u64) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_left(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shr<u64> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u64) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_right(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shl<u32> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u32) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_left(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shr<u32> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u32) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_right(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shl<u16> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u16) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_left(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shr<u16> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u16) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_right(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shl<u8> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u8) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_left(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shr<u8> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u8) -> Self::Output {
        let mut n = BigUint {
            data: long_shift_right(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}
