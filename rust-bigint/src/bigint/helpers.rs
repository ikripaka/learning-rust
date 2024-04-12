use std::cmp::Ordering;

use crate::bigint::Sign;
use crate::BigInt;

/// **partial_cmp** -- compares both BigUint values
pub(crate) fn partial_cmp(x: &BigInt, y: &BigInt) -> Option<Ordering> {
    match (&x.sign, &y.sign) {
        (Sign::Positive, Sign::Negative) => Some(Ordering::Greater),
        (Sign::Negative, Sign::Positive) => Some(Ordering::Less),
        (Sign::Positive, Sign::Positive) => x.data.partial_cmp(&y.data),
        (Sign::Negative, Sign::Negative) => y.data.partial_cmp(&x.data),
    }
}
