use std::ops::{Shl, ShlAssign, Shr, ShrAssign};
use crate::BigUint;

impl ShlAssign<u128> for BigUint {
    fn shl_assign(&mut self, rhs: u128) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs);
        self.data = data;
        self.fit()
    }
}
impl ShlAssign<u64> for BigUint {
    fn shl_assign(&mut self, rhs: u64) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShlAssign<u32> for BigUint {
    fn shl_assign(&mut self, rhs: u32) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShlAssign<u16> for BigUint {
    fn shl_assign(&mut self, rhs: u16) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShlAssign<u8> for BigUint {
    fn shl_assign(&mut self, rhs: u8) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}

impl ShrAssign<u128> for BigUint {
    fn shr_assign(&mut self, rhs: u128) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs);
        self.data = data;
        self.fit()
    }
}

impl ShrAssign<u64> for BigUint {
    fn shr_assign(&mut self, rhs: u64) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShrAssign<u32> for BigUint {
    fn shr_assign(&mut self, rhs: u32) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShrAssign<u16> for BigUint {
    fn shr_assign(&mut self, rhs: u16) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}
impl ShrAssign<u8> for BigUint {
    fn shr_assign(&mut self, rhs: u8) {
        let data = crate::biguint::shift::long_shift_left(&self.data, rhs as u128);
        self.data = data;
        self.fit()
    }
}

impl Shl<u128> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u128) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_left(&self.data, rhs),
        };
        n.fit();
        n
    }
}

impl Shr<u128> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u128) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_right(&self.data, rhs),
        };
        n.fit();
        n
    }
}
impl Shl<u64> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u64) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_left(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shr<u64> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u64) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_right(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shl<u32> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u32) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_left(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shr<u32> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u32) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_right(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shl<u16> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u16) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_left(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shr<u16> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u16) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_right(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shl<u8> for BigUint {
    type Output = BigUint;

    fn shl(self, rhs: u8) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_left(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}

impl Shr<u8> for BigUint {
    type Output = BigUint;

    fn shr(self, rhs: u8) -> Self::Output {
        let mut n = BigUint {
            data: crate::biguint::shift::long_shift_right(&self.data, rhs as u128),
        };
        n.fit();
        n
    }
}