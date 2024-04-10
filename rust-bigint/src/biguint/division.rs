use crate::biguint::BigUint;
use crate::{Digit, BITS_IN_BASE};
use num_traits::{One, Zero};
use std::ops::{Div, DivAssign, Rem, RemAssign};

/// **divide** -- (Quotient, Remainder)
/// A = B * Q + R, 0 <= R < B
fn divide(a: &BigUint, b: &BigUint) -> (BigUint, BigUint) {
    assert_ne!(*b, BigUint::zero(), "Can't divide on zero number");
    let k = calc_bit_len(&b.data);
    let mut r = a.clone();
    let mut q = BigUint::zero();
    while r >= *b {
        let mut t = calc_bit_len(&r.data);
        let mut c = b.clone() << (t - k);
        if r < c {
            t -= 1;
            c = b.clone() << (t - k);
        }
        r -= c;
        q += BigUint::one() << (t - k)
    }
    (q, r)
}

fn calc_bit_len(a: &[Digit]) -> u128 {
    if a.len() == 0 {
        return 0;
    }
    let (mut len, mut x) = ((a.len() - 1) as u128 * BITS_IN_BASE, *a.last().unwrap());
    for i in 0..=BITS_IN_BASE {
        if x == 0 {
            len += i;
            break;
        }
        x >>= 1;
    }
    len
}

fn barret_reduction(a: &BigUint, module: &BigUint) -> BigUint{
    todo!()
}

impl Div<BigUint> for BigUint {
    type Output = BigUint;

    fn div(self, rhs: BigUint) -> Self::Output {
        self / &rhs
    }
}

impl Div<&BigUint> for BigUint {
    type Output = BigUint;

    fn div(self, rhs: &BigUint) -> Self::Output {
        let mut res = divide(&self, rhs).0;
        res.fit();
        res
    }
}

impl Rem<BigUint> for BigUint {
    type Output = BigUint;

    fn rem(self, rhs: BigUint) -> Self::Output {
        self % &rhs
    }
}

impl Rem<&BigUint> for BigUint {
    type Output = BigUint;

    fn rem(self, rhs: &BigUint) -> Self::Output {
        let (_, mut r) = divide(&self, rhs);
        r.fit();
        r
    }
}

impl DivAssign<BigUint> for BigUint {
    fn div_assign(&mut self, rhs: BigUint) {
        let res = self.clone() / &rhs;
        self.data = res.data
    }
}

impl DivAssign<&BigUint> for BigUint {
    fn div_assign(&mut self, rhs: &BigUint) {
        let res = self.clone() / rhs;
        self.data = res.data
    }
}

impl RemAssign<BigUint> for BigUint {
    fn rem_assign(&mut self, rhs: BigUint) {
        let res = self.clone() % &rhs;
        self.data = res.data
    }
}

impl RemAssign<&BigUint> for BigUint {
    fn rem_assign(&mut self, rhs: &BigUint) {
        let res = self.clone() % rhs;
        self.data = res.data
    }
}
