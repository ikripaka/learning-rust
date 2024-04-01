mod addition;
mod division;
mod multiplication;
mod subtraction;

use crate::Digit;
use core::hash;
use num_traits::{Num, NumAssignOps, NumAssignRef, NumOps, NumRef, One, RefNum, Zero};
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hasher;
use std::ops::{AddAssign, DivAssign, MulAssign, RemAssign, SubAssign};

pub struct BigUint {
    data: Vec<Digit>,
}

impl BigUint {
    pub fn from() -> Self {
        todo!()
    }

    // implement as an idea creation from PackedStruct slice
}

impl Zero for BigUint {
    fn zero() -> Self {
        todo!()
    }

    fn is_zero(&self) -> bool {
        todo!()
    }
}

impl One for BigUint {
    fn one() -> Self {
        todo!()
    }
}
impl Num for BigUint {
    type FromStrRadixErr = ();

    fn from_str_radix(str: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        todo!()
    }
}

impl Clone for BigUint {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl hash::Hash for BigUint {
    fn hash<H: Hasher>(&self, state: &mut H) {
        todo!()
    }
}

impl Eq for BigUint {}

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
        todo!()
    }
}

impl fmt::Debug for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Display for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::LowerHex for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::UpperHex for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Binary for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

impl fmt::Octal for BigUint {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
