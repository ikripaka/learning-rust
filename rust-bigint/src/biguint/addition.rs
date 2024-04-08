use crate::biguint::BigUint;
use crate::Digit;
use std::ops::{Add, AddAssign};

/// **add** -- adds iterates over right value (that less than left one) and adds both
///     values between themselves
fn add(x: &[Digit], y: &[Digit]) -> Vec<Digit> {
    let (mut carry, mut i, mut data) = (false, 0, Vec::with_capacity(x.len() + 1));
    for (j, d) in y.iter().enumerate() {
        let (tmp, carry_1) = (*d).overflowing_add(x[j]);
        let (tmp, carry_2) = tmp.overflowing_add(carry as u64);
        data.push(tmp);
        carry = carry_1 || carry_2;
        i += 1
    }
    for j in i..x.len() {
        let (tmp, carry_1) = x[j].overflowing_add(carry as u64);
        data.push(tmp);
        carry = carry_1;
    }
    if carry {
        data.push(carry as u64)
    }

    data
}

impl Add<BigUint> for BigUint {
    type Output = BigUint;

    fn add(self, rhs: Self) -> Self::Output {
        self + &rhs
    }
}

impl Add<&BigUint> for BigUint {
    type Output = BigUint;

    fn add(self, rhs: &BigUint) -> Self::Output {
        let data = match self.data.len() > rhs.data.len() {
            true => add(&self.data, &rhs.data),
            false => add(&rhs.data, &self.data),
        };
        let mut n = BigUint { data };
        n.fit(); //maybe this is redundant for addition
        n
    }
}

impl AddAssign<Self> for BigUint {
    fn add_assign(&mut self, rhs: BigUint) {
        let res = self.clone() + rhs;
        self.data = res.data
    }
}

impl AddAssign<&BigUint> for BigUint {
    fn add_assign(&mut self, rhs: &BigUint) {
        let res = self.clone() + rhs;
        self.data = res.data
    }
}
