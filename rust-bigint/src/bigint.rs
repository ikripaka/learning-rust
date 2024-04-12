use std::cmp::Ordering;
use std::fmt;
use std::fmt::{Display, Formatter};

use num_traits::{Num, One, Zero};

use crate::bigint::helpers::partial_cmp;
use crate::bigint::multiplication::pow_mod;
use crate::biguint::conversion::{
    parse_from_bit_str, parse_from_byte_slice, parse_from_hex_str, to_binary, to_lower_hex,
    to_octal, to_upper_hex,
};
use crate::biguint::BigUint;
use crate::{ParseBigIntErr, ParseBigUintErr};

mod addition;
mod conversion;
mod division;
mod helpers;
mod multiplication;
mod shift;
mod subtraction;

impl BigInt {
    fn from_bytes_radix(data: &[u8], radix: u32) -> Result<Self, ParseBigUintErr> {
        // if str.is_empty() {
        //     return Ok(BigInt {
        //         sign: Sign::Positive,
        //         data: BigUint::zero(),
        //     });
        // }
        // Ok({
        //     let (sign, str) = extract_sign(str)?;
        //     let mut n = match radix {
        //         2 => parse_from_byte_slice(data)?,
        //         16 => parse_from_byte_slice(data)?,
        //         _ => return Err(ParseBigUintErr::UnhandledRadix(radix)),
        //     };
        //     n.fit();
        //     assert!(
        //         !(n.is_zero() && sign == Sign::Negative),
        //         "Can't be -0 number"
        //     );
        //     BigInt { sign, data: n }
        // })
        Ok(BigInt::one())
    }
    /// **to_biguint** -- converts BigInt to unsinged BigUint number
    pub fn to_biguint(&self) -> BigUint {
        self.data.clone()
    }
    /// **to_lower_hex_string** -- converts BigInt to lower hex number format string
    pub fn to_lower_hex_string(&self) -> String {
        format!("{:x}", self)
    }
    /// **to_upper_hex_string** -- converts BigInt to upper hex number format string
    pub fn to_upper_hex_string(&self) -> String {
        format!("{:X}", self)
    }
    /// **to_binary_string** -- converts BigInt to binary number format string
    pub fn to_binary_string(&self) -> String {
        format!("{:b}", self)
    }
    /// **pow_mod** -- powers number to certain power using sliding 4bit window algorithm
    pub fn pow_mod(&self, power: &BigUint, module: &BigUint) -> BigInt {
        pow_mod(self, power, module)
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub(crate) enum Sign {
    Positive,
    Negative,
}

#[derive(Eq, Clone)]
pub struct BigInt {
    pub(crate) sign: Sign,
    pub(crate) data: BigUint,
}

impl Display for Sign {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Sign::Positive => "".to_string(),
                Sign::Negative => "-".to_string(),
            }
        )
    }
}

impl Zero for BigInt {
    fn zero() -> Self {
        Self {
            sign: Sign::Positive,
            data: BigUint::zero(),
        }
    }

    fn is_zero(&self) -> bool {
        if let Sign::Positive = self.sign {
            return self.data.is_zero();
        }
        false
    }
}

impl One for BigInt {
    fn one() -> Self {
        Self {
            sign: Sign::Positive,
            data: BigUint::one(),
        }
    }
}

impl Num for BigInt {
    type FromStrRadixErr = ParseBigIntErr;

    /// Creates BigUint struct from radix 2 and 16.
    /// Input has to be in ASCII code.
    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        if str.is_empty() {
            return Ok(BigInt {
                sign: Sign::Positive,
                data: BigUint::zero(),
            });
        }
        Ok({
            let (sign, str) = extract_sign(str)?;
            let mut n = match radix {
                2 => parse_from_bit_str(str)?,
                16 => parse_from_hex_str(str)?,
                _ => return Err(ParseBigIntErr::UnhandledRadix(radix)),
            };
            n.fit();
            assert!(
                !(n.is_zero() && sign == Sign::Negative),
                "Can't be -0 number"
            );
            BigInt { sign, data: n }
        })
    }
}

fn extract_sign(s: &str) -> Result<(Sign, &str), ParseBigIntErr> {
    let mut chars = s.chars();
    match chars.next() {
        None => Ok((Sign::Positive, s)),
        Some(c) => {
            if c == '+' {
                Ok((Sign::Positive, &s[1..]))
            } else if c == '-' {
                Ok((Sign::Negative, &s[1..]))
            } else if (c == '0' || c == '1')
                || (c >= '0' && c <= '9')
                || (c >= 'a' && c <= 'f')
                || (c >= 'A' && c <= 'F')
            {
                // or insert here regex
                Ok((Sign::Positive, s))
            } else {
                Err(ParseBigIntErr::CantParseSign(c.to_string()))
            }
        }
    }
}

impl PartialEq<Self> for BigInt {
    fn eq(&self, other: &Self) -> bool {
        if self.sign == other.sign {
            return self.data.eq(&other.data);
        }
        false
    }
}

impl PartialOrd<Self> for BigInt {
    /// in our situations **partial_cmp** is possible in all cases, so we don't have to use None
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        partial_cmp(&self, other)
    }
}

impl Ord for BigInt {
    fn cmp(&self, other: &Self) -> Ordering {
        partial_cmp(&self, other).unwrap()
    }
}

impl Default for BigInt {
    fn default() -> Self {
        Self::zero()
    }
}

impl fmt::Debug for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}{:X?}", self.sign, self.data))
    }
}

impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", format!("{}{:X?}", self.sign, self.data))
    }
}

impl fmt::LowerHex for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.sign, to_lower_hex(&self.data))
    }
}

impl fmt::UpperHex for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.sign, to_upper_hex(&self.data))
    }
}

impl fmt::Binary for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.sign, to_binary(&self.data))
    }
}

impl fmt::Octal for BigInt {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.sign, to_octal(&self.data))
    }
}
