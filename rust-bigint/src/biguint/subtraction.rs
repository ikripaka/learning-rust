use crate::biguint::BigUint;
use crate::{Digit, BASE};
use std::ops::{Sub, SubAssign};

fn sub(x: &[Digit], y: &[Digit]) -> Vec<Digit> {
    let subtract = |x: u64, y: u64| -> (u64, bool) {
        match x.overflowing_sub(y) {
            (res, true) => ((x as u128 + BASE - y as u128) as u64, true),
            (res, false) => (res, false),
        }
    };

    let (mut borrow, mut i, mut data) = (false, 0, vec![]);

    for (j, d) in y.iter().enumerate() {
        let (tmp, borrow_1) = subtract(*d, x[j]);
        let (tmp, borrow_2) = subtract(tmp, borrow as u64);
        data.push(tmp);
        borrow = borrow_1 || borrow_2;
        i += 1
    }
    for j in i..x.len() {
        let (tmp, borrow_1) = subtract(x[j], borrow as u64);
        data.push(tmp);
        borrow = borrow_1;
    }
    data
}

impl Sub<BigUint> for BigUint {
    type Output = BigUint;

    fn sub(self, rhs: BigUint) -> Self::Output {
        self - &rhs
    }
}

impl Sub<&BigUint> for BigUint {
    type Output = BigUint;

    /// **sub** -- swaps arguments and performs subtraction to minimize errors
    fn sub(self, rhs: &BigUint) -> Self::Output {
        let data = match self.data.len() > rhs.data.len() {
            true => sub(&self.data, &rhs.data),
            false => sub(&rhs.data, &self.data),
        };
        let mut n = BigUint { data };
        n.fit();
        n
    }
}

impl SubAssign<BigUint> for BigUint {
    fn sub_assign(&mut self, rhs: BigUint) {
        let res = self.clone() - rhs;
        self.data = res.data
    }
}
impl SubAssign<&BigUint> for BigUint {
    fn sub_assign(&mut self, rhs: &BigUint) {
        let res = self.clone() - rhs;
        self.data = res.data
    }
}
