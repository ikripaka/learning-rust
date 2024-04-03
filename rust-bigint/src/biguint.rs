mod addition;
mod division;
mod multiplication;
mod subtraction;
pub(crate) mod conversion;

use crate::{Digit, ParseBigUintErr};
use core::hash;
use num_traits::{Num, NumAssignOps, NumAssignRef, NumOps, NumRef, One, RefNum, Zero};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hasher;
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};
use crate::biguint::conversion::{parse_from_bit_str, parse_from_byte_slice, parse_from_hex_str, to_binary, to_lower_hex, to_octal, to_upper_hex};

#[derive(Hash, Clone, Eq)]
pub struct BigUint {
    data: Vec<Digit>,
}

impl BigUint {
    //todo: create from method that would trim zeros
    pub fn from_bytes_radix(data: &[u8], radix: u32) -> Result<Self, ParseBigUintErr> {
        Ok(match radix {
            2 => parse_from_byte_slice(data)?,
            16 => parse_from_byte_slice(data)?,
            _ => return Err(ParseBigUintErr::UnhandledRadix(radix))
        })
    }

    // implement as an idea creation from PackedStruct slice
}

impl Zero for BigUint {
    fn zero() -> Self {
        BigUint { data: vec![0] }
    }

    fn is_zero(&self) -> bool {
        for x in &self.data {
            if !x.is_zero() {
                return false;
            }
        }
        true
    }
}

impl One for BigUint {
    fn one() -> Self {
        BigUint {
            data: vec![1]
        }
    }
}

impl Num for BigUint {
    type FromStrRadixErr = ParseBigUintErr;

    /// Creates BigUint struct from radix 2 and 16.
    /// Input has to be in ASCII code.
    /// ```rust
    /// use rust_bigint::BigUint;
    ///
    /// ```
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        Ok(match radix {
            2 => parse_from_bit_str(str)?,
            16 => parse_from_hex_str(str)?,
            _ => return Err(ParseBigUintErr::UnhandledRadix(radix))
        })
    }
}


impl PartialEq<Self> for BigUint {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd<Self> for BigUint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}

impl Ord for BigUint {
    fn cmp(&self, other: &Self) -> Ordering {
        todo!()
    }
}

impl Default for BigUint {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Debug for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",format!("{:8X?}",self.data))
    }
}

impl fmt::Display for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}",format!("{:X?}", self))
    }
}

impl fmt::LowerHex for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", to_lower_hex(&self))
    }
}

impl fmt::UpperHex for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", to_upper_hex(&self))
    }
}

impl fmt::Binary for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", to_binary(&self))
    }
}

impl fmt::Octal for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", to_octal(&self))
    }
}