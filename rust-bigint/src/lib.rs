extern crate core;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use std::num::ParseIntError;

mod bigint;
mod biguint;

pub use bigint::BigInt;
pub use biguint::BigUint;

pub(crate) type Digit = u64;
pub(crate) type DoubleDigit = u128;

pub enum ParseBigUintErr {
    UnhandledRadix(u32),
    IncorrectSymbol((bool, String)),
    UnableToParseInt(String)
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

impl From<ParseIntError> for ParseBigUintErr{
    fn from(value: ParseIntError) -> Self {
        ParseBigUintErr::UnableToParseInt(format!("{value}"))
    }
}
