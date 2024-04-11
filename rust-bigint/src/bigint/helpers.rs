use std::cmp::Ordering;
use crate::{BigInt, BigUint};
use crate::bigint::Sign;

/// **partial_cmp** -- compares both BigUint values
pub(crate) fn partial_cmp(x: &BigInt, y: &BigInt) -> Option<Ordering> {
    match (&x.sign, &y.sign){
        (Sign::Positive, Sign::Negative) => Some(Ordering::Greater),
        (Sign::Negative, Sign::Positive) => Some(Ordering::Less),
        (Sign::Positive, Sign::Positive) => {
            x.partial_cmp(y)
        },
        (Sign::Negative, Sign::Negative) => y.partial_cmp(x),
    }
}