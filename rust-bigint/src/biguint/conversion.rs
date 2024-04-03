use std::fmt::format;
use std::mem;
use crate::{BigUint, ParseBigUintErr};
use num_traits::Zero;

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
    for x in data.chars().collect::<Vec<char>>().chunks(HEX_SYMBOLS_IN_BYTE) {
        res.push(u8::from_str_radix(&x.iter().fold(String::new(), |mut acc, x| {
            acc += &x.to_string();
            acc
        }), 16)?)
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
        res.push(u8::from_str_radix(&x.iter().fold(String::new(), |mut acc, x| {
            acc += &x.to_string();
            acc
        }), 2)?)
    }
   parse_from_byte_slice(&res)
}
pub(crate) fn parse_from_byte_slice(data: &[u8]) -> Result<BigUint, ParseBigUintErr> {
    let data = {
        let mut data = data.to_vec();
        while data.len() % BITS_IN_BYTE != 0{
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
    let tmp = tmp.iter().map(|x| format!("{:#64b}", x)).collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
        acc += &x;
        acc
    });
    tmp.trim_start_matches("0").to_string()
}

pub(crate) fn to_lower_hex(x: &BigUint) -> String {
    let mut tmp = x.data.clone();
    tmp.reverse();
    let tmp = tmp.iter().map(|x| format!("{:16x}", x)).collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| {
        acc += &x;
        acc
    });
    tmp.trim_start_matches("0").to_string()
}

pub(crate) fn to_upper_hex(x: &BigUint) -> String {
    to_lower_hex(x).to_uppercase()
}

pub(crate) fn to_octal(x: &BigUint) -> String {
    let mut tmp = x.data.clone();
    tmp.reverse();
    let tmp = tmp.iter().map(|x| format!("{:08o}", x)).collect::<Vec<String>>();
    let tmp = tmp.iter().fold(String::new(), |mut acc, x| { acc += &x; acc });
    tmp.trim_start_matches("0").to_string()
}