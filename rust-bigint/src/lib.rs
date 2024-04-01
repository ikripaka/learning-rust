extern crate core;

use std::error::Error;
use std::fmt::{Debug, Display, Formatter};

mod bigint;
mod biguint;

pub(crate) type Digit = u64;
pub(crate) type DoubleDigit = u128;

pub enum BigIntError {}

impl Debug for BigIntError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Error for BigIntError {}

impl Display for BigIntError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
