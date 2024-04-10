use crate::biguint::helpers::fit_u8_vec;
use crate::biguint::BigUint;
use crate::{Digit, DoubleDigit, BASE, BASE_BIT_MASK, BITS_IN_BASE};
use num_traits::{pow, One, Pow};
use std::mem::size_of;
use std::ops::{Mul, MulAssign};

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
        pow_4_window(&self, rhs)
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
fn pow_4_window(a: &BigUint, power: u128) -> BigUint {
    //     ArrayList<Integer> C = new ArrayList<>();
    //         C.add(0, 1);
    //         ArrayList<Integer>[] window = new ArrayList[16];
    //         window[0] = new ArrayList();
    //         window[0].add(1);
    //         window[1] = A;
    //
    //         ArrayList<Integer> powerInHex = convertBase256ToHex(power);
    //         for (int i = 2; i < 16; i++) {
    //             window[i] = longMulMod(window[i - 1], A, mod);
    //         }
    //
    //         for (int j = 0; j < powerInHex.size(); j++) {
    //             C = longMulMod(C, window[powerInHex.get(j)], mod);
    //
    //             if (j != powerInHex.size() - 1) {
    //                 for (int m = 0; m < 4; m++) {
    //                     C = longMulMod(C, C, mod);
    //                 }
    //             }
    //         }
    //         return C;

    let mut c = BigUint::one();

    let mut window = Vec::with_capacity(16);
    window.push(BigUint::one());
    window.push(a.clone());
    for i in 2..16 {
        window.push(a.clone() * &window[i - 1])
    }
    let power_hex_list = extract_hex_vec_from_u128(power);
    let power_hex_list_last = power_hex_list.len() - 1;
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

fn extract_hex_vec_from_u128(n: u128) -> Vec<u8> {
    const HEX_MASK: u128 = 0xF;
    const HEX_BITS: usize = 4;

    let mut res = Vec::with_capacity(size_of::<u128>() / HEX_BITS);
    for i in 0..res.capacity() {
        res.push(((n & (HEX_MASK << (HEX_BITS * i))) >> (HEX_BITS * i)) as u8)
    }
    fit_u8_vec(&mut res);
    res
}

fn mod_pow(num: &BigUint, module: &BigUint, power: u64) -> BigUint {
    todo!()
}
