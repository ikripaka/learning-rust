use crate::biguint::BigUint;
use std::ops::{Add, AddAssign};
use crate::Digit;

/// **add** -- adds iterates over left value and adds both values between themselves
/// DISCLAIMER: may cause errors if tight value is less than left
fn add(x: &[Digit], y: &[Digit]) -> Vec<Digit>{
    let (mut carry , mut data) = (false, vec![]);
    for (i, x) in x.iter().enumerate(){
        let (tmp, carry_1) = (*x).overflowing_add(data[i]);
        let (tmp, carry_2) = tmp.overflowing_add(carry as u64);
        data.push(tmp);
        carry = carry_1 || carry_2
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
        let data = match self.data.len() > rhs.data.len(){
            true => add(&self.data, &rhs.data),
            false => add(&rhs.data, &self.data)
        };
        let mut n = BigUint{data};
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
