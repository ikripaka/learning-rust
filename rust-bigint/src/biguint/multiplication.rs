use std::ops::{Mul, MulAssign};

use num_traits::{One, Pow};

use crate::biguint::helpers::extract_hex_vec_from_biguint;
use crate::biguint::helpers::extract_hex_vec_from_u128;
use crate::biguint::BigUint;
use crate::{Digit, DoubleDigit, BASE_BIT_MASK, BITS_IN_BASE};

impl Mul<BigUint> for BigUint {
    type Output = BigUint;

    fn mul(self, rhs: BigUint) -> Self::Output {
        self * &rhs
    }
}

impl Mul<&BigUint> for BigUint {
    type Output = BigUint;

    fn mul(self, rhs: &BigUint) -> Self::Output {
        let mut res = ordinary_mul(&self.data, &rhs.data);
        res.fit();
        res
    }
}

impl MulAssign<&BigUint> for BigUint {
    fn mul_assign(&mut self, rhs: &BigUint) {
        let res = self.clone() * rhs;
        self.data = res.data;
    }
}

impl MulAssign<BigUint> for BigUint {
    fn mul_assign(&mut self, rhs: BigUint) {
        let res = self.clone() * &rhs;
        self.data = res.data;
    }
}

impl Pow<u128> for BigUint {
    type Output = BigUint;

    fn pow(self, rhs: u128) -> Self::Output {
        pow_4_window(&self, &extract_hex_vec_from_u128(rhs))
    }
}

impl Pow<&BigUint> for BigUint {
    type Output = BigUint;

    fn pow(self, rhs: &BigUint) -> Self::Output {
        pow_4_window(&self, &extract_hex_vec_from_biguint(rhs))
    }
}

impl Pow<BigUint> for BigUint {
    type Output = BigUint;

    fn pow(self, rhs: BigUint) -> Self::Output {
        self.pow(&rhs)
    }
}

fn ordinary_mul(lhs: &[Digit], rhs: &[Digit]) -> BigUint {
    let long_mul_one_digit = |slice: &[Digit], m: Digit| -> BigUint {
        let mut carry: DoubleDigit = 0;
        let mut data = Vec::with_capacity(slice.len() + 1);
        for d in slice.iter() {
            let tmp = (*d) as DoubleDigit * m as DoubleDigit + carry;
            data.push((tmp & BASE_BIT_MASK) as Digit);
            carry = tmp >> BITS_IN_BASE;
        }
        if carry != 0 {
            data.push(carry as Digit)
        }
        BigUint { data }
    };

    let mut c = BigUint {
        data: vec![0; lhs.len() * 2],
    };
    for (i, d) in rhs.iter().enumerate() {
        let mut tmp = long_mul_one_digit(lhs, *d);
        tmp <<= i as u128 * BITS_IN_BASE;
        c += tmp;
    }
    c
}

// < 100_000 digits
fn mul_karatsuba(lhs: &BigUint, rhs: &BigUint) -> BigUint {
    todo!()
}

// > 100_000 digits
fn mul_fast_fourier_transform(lhs: &BigUint, rhs: &BigUint) -> BigUint {
    todo!()
}

// implementing sliding window method of powering numbers
fn pow_4_window(a: &BigUint, power_hex_list: &Vec<u8>) -> BigUint {
    let mut c = BigUint::one();

    let mut window = Vec::with_capacity(16);
    window.push(BigUint::one());
    window.push(a.clone());
    for i in 2..16 {
        window.push(a.clone() * &window[i - 1])
    }
    for (j, h) in power_hex_list.iter().enumerate().rev() {
        c *= &window[*h as usize];
        if j != 0 {
            for _ in 0..4 {
                c = c.clone() * c.clone()
            }
        }
    }
    c
}

fn pow_mod_4_window(a: &BigUint, power: &BigUint, module: &BigUint) -> BigUint {
    let mut c = BigUint::one();

    let mut window = Vec::with_capacity(16);
    window.push(BigUint::one());
    window.push(a.clone() % module);
    for i in 2..16 {
        window.push((a.clone() * &window[i - 1]) % module);
    }
    let power_hex_list = extract_hex_vec_from_biguint(power);
    for (j, h) in power_hex_list.iter().enumerate().rev() {
        c = (c * &window[*h as usize]) % module;
        if j != 0 {
            for _ in 0..4 {
                c = (c.clone() * c.clone()) % module
            }
        }
    }
    c
}

pub(super) fn mod_pow(a: &BigUint, power: &BigUint, module: &BigUint) -> BigUint {
    pow_mod_4_window(a, power, module)
}
