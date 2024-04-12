use num_traits::ToPrimitive;

use crate::{BigUint, ParseBigUintErr, BITS_IN_BASE};

const BITS_IN_BYTE: usize = 8;
const HEX_SYMBOLS_IN_BYTE: usize = 2;

pub(crate) fn parse_from_hex_str(data: &str) -> Result<BigUint, ParseBigUintErr> {
    const HEX_SYMBOLS_IN_BYTE: usize = 2;

    let comments_regex = regex::Regex::new(r" +").unwrap();
    let mut data = comments_regex.replace_all(data, "").to_string();
    if data.len() % HEX_SYMBOLS_IN_BYTE != 0 {
        data = "0".to_string() + data.as_str()
    }
    let mut res = Vec::new();
    for x in data
        .chars()
        .collect::<Vec<char>>()
        .chunks(HEX_SYMBOLS_IN_BYTE)
    {
        res.push(u8::from_str_radix(
            &x.iter().fold(String::new(), |mut acc, x| {
                acc += &x.to_string();
                acc
            }),
            16,
        )?)
    }
    parse_from_byte_slice(&res)
}

pub(crate) fn parse_from_bit_str(data: &str) -> Result<BigUint, ParseBigUintErr> {
    let comments_regex = regex::Regex::new(r" +").unwrap();
    let mut data = comments_regex.replace_all(data, "").to_string();
    while data.len() % BITS_IN_BYTE != 0 {
        data = "0".to_string() + &data
    }
    let mut res = Vec::new();
    for x in data.chars().collect::<Vec<char>>().chunks(BITS_IN_BYTE) {
        res.push(u8::from_str_radix(
            &x.iter().fold(String::new(), |mut acc, x| {
                acc += &x.to_string();
                acc
            }),
            2,
        )?)
    }
    parse_from_byte_slice(&res)
}

pub(crate) fn parse_from_byte_slice(data: &[u8]) -> Result<BigUint, ParseBigUintErr> {
    let data = {
        let mut data = data.to_vec();
        while data.len() % BITS_IN_BYTE != 0 {
            data.insert(0, 0)
        }
        data
    };
    Ok(BigUint {
        data: {
            let mut x = data
                .chunks(BITS_IN_BYTE)
                .map(|x| u64::from_be_bytes([x[0], x[1], x[2], x[3], x[4], x[5], x[6], x[7]]))
                .collect::<Vec<u64>>();
            x.reverse();
            x
        },
    })
}

pub(crate) fn to_binary(x: &BigUint) -> String {
    let mut tmp = x.data.clone();
    tmp.reverse();
    let tmp = tmp
        .iter()
        .map(|x| format!("{:064b}", x))
        .collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
        acc += &x;
        acc
    });
    let x = tmp.trim_start_matches("0");
    if x.is_empty() {
        "0".to_string()
    } else {
        x.to_string()
    }
}

pub(crate) fn to_lower_hex(x: &BigUint) -> String {
    let mut tmp = x.data.clone();
    tmp.reverse();
    let tmp = tmp
        .iter()
        .map(|x| format!("{:016x?}", x))
        .collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
        acc += &x;
        acc
    });
    let x = tmp.trim_start_matches("0");
    if x.is_empty() {
        "0".to_string()
    } else {
        x.to_string()
    }
}

pub(crate) fn to_upper_hex(x: &BigUint) -> String {
    to_lower_hex(x).to_uppercase()
}

pub(crate) fn to_octal(x: &BigUint) -> String {
    let mut tmp = x.data.clone();
    tmp.reverse();
    let tmp = tmp
        .iter()
        .map(|x| format!("{:08o}", x))
        .collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
        acc += &x;
        acc
    });
    let x = tmp.trim_start_matches("0");
    if x.is_empty() {
        "0".to_string()
    } else {
        x.to_string()
    }
}

impl ToPrimitive for BigUint {
    fn to_i64(&self) -> Option<i64> {
        self.to_u64().and_then(|x| x.to_i64())
    }

    fn to_i128(&self) -> Option<i128> {
        self.to_u128().and_then(|x| x.to_i128())
    }

    fn to_u64(&self) -> Option<u64> {
        let (mut x, mut bits_filled) = (0, 0);
        for d in &self.data {
            if !(bits_filled < BITS_IN_BASE) {
                return None;
            }
            x = *d;
            bits_filled += BITS_IN_BASE;
        }
        Some(x)
    }

    fn to_u128(&self) -> Option<u128> {
        let (mut x, mut bits_filled) = (0, 0);
        for (i, d) in self.data.iter().enumerate() {
            if !(bits_filled < BITS_IN_BASE * 2) {
                return None;
            }
            x = x | ((*d as u128) << (i as u128 * BITS_IN_BASE));
            bits_filled += BITS_IN_BASE;
        }
        Some(x)
    }
}
