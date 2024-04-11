mod addition;
mod conversion;
mod division;
mod multiplication;
mod shift;
mod subtraction;

use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use num_traits::{Num, One, Zero};
use crate::biguint::BigUint;
use crate::biguint::conversion::{parse_from_bit_str, parse_from_hex_str, to_binary, to_lower_hex, to_octal, to_upper_hex};
use crate::ParseBigUintErr;

#[derive(Debug, Clone, Hash)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

pub struct BigInt {
     sign: Sign,
    data: BigUint,
}

impl Zero for BigInt{
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl One for BigInt{
    fn one() -> Self {
        todo!()
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
        if str.is_empty() {
            return Ok(BigUint::zero());
        }
        Ok({
            let mut n = match radix {
                2 => parse_from_bit_str(str)?,
                16 => parse_from_hex_str(str)?,
                _ => return Err(ParseBigUintErr::UnhandledRadix(radix)),
            };
            n.fit();

            n
        })
    }
}

impl PartialEq<Self> for BigUint {
    fn eq(&self, other: &Self) -> bool {
        self.data.eq(&other.data)
    }
}

impl PartialOrd<Self> for BigUint {
    /// in our situations **partial_cmp** is possible in all cases, so we don't have to use None
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(crate::biguint::helpers::partial_cmp(&self, other))
    }
}

impl Ord for BigUint {
    fn cmp(&self, other: &Self) -> Ordering {
        crate::biguint::helpers::partial_cmp(&self, other)
    }
}

impl Default for BigUint {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Debug for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:X?}", self.data))
    }
}

impl fmt::Display for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{:X?}", self))
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
