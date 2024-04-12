extern crate core;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

pub use bigint::BigInt;
pub use biguint::BigUint;

mod bigint;
mod biguint;
mod helpers;

pub(crate) type Digit = u64;
pub(crate) type DoubleDigit = u128;

const BASE: u128 = 1 << 64;
const BITS_IN_BASE: u128 = 64;

/// **BASE_BIT_MASK** -- 64 bit mask for extracting certain amount of num
const BASE_BIT_MASK: u128 = 0xFFFF_FFFF_FFFF_FFFF;

#[derive(Debug)]
pub enum ParseBigIntErr {
    CantParseSign(String),
    UnhandledRadix(u32),
    IncorrectSymbol((bool, String)),
    UnableToParseInt(String),
}

impl From<ParseBigUintErr> for ParseBigIntErr {
    fn from(value: ParseBigUintErr) -> Self {
        match value {
            ParseBigUintErr::UnhandledRadix(x) => ParseBigIntErr::UnhandledRadix(x),
            ParseBigUintErr::IncorrectSymbol(x) => ParseBigIntErr::IncorrectSymbol(x),
            ParseBigUintErr::UnableToParseInt(x) => ParseBigIntErr::UnableToParseInt(x),
        }
    }
}

impl Display for ParseBigIntErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseBigIntErr::CantParseSign(sign) => format!("Can't parse sing: {sign}"),
            ParseBigIntErr::UnhandledRadix(radix) =>
                format!("Cannot handle such radix in conversion, got: {radix}, can handle only radix 2 or 16"),
            ParseBigIntErr::IncorrectSymbol((is_binary, symbol)) =>
                format!("Can't handle such symbol: \"{symbol}\" in {} conversion",
                        if *is_binary {
                            "binary"
                        } else {
                            "hex"
                        }),
            ParseBigIntErr::UnableToParseInt(msg) => format!("Unable to parse int from str, message: {msg}")
        })
    }
}

impl Error for ParseBigIntErr {}

impl From<ParseIntError> for ParseBigIntErr {
    fn from(value: ParseIntError) -> Self {
        ParseBigIntErr::UnableToParseInt(format!("{value}"))
    }
}

pub enum ParseBigUintErr {
    UnhandledRadix(u32),
    IncorrectSymbol((bool, String)),
    UnableToParseInt(String),
}

impl Debug for ParseBigUintErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for ParseBigUintErr {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            ParseBigUintErr::UnhandledRadix(radix) =>
                format!("Cannot handle such radix in conversion, got: {radix}, can handle only radix 2 or 16"),
            ParseBigUintErr::IncorrectSymbol((is_binary, symbol)) =>
                format!("Can't handle such symbol: \"{symbol}\" in {} conversion",
                        if *is_binary {
                            "binary"
                        } else {
                            "hex"
                        }),
            ParseBigUintErr::UnableToParseInt(msg) => format!("Unable to parse int from str, message: {msg}")
        })
    }
}

impl Error for ParseBigUintErr {}

impl From<ParseIntError> for ParseBigUintErr {
    fn from(value: ParseIntError) -> Self {
        ParseBigUintErr::UnableToParseInt(format!("{value}"))
    }
}
