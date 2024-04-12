use num_traits::ToPrimitive;

use crate::bigint::Sign;
use crate::BigInt;

impl ToPrimitive for BigInt {
    fn to_i64(&self) -> Option<i64> {
        self.to_u64()
            .and_then(|x| x.to_i64())
            .and_then(|x| match self.sign {
                Sign::Positive => Some(x),
                Sign::Negative => Some(-x),
            })
    }

    fn to_i128(&self) -> Option<i128> {
        self.to_u128()
            .and_then(|x| x.to_i128())
            .and_then(|x| match self.sign {
                Sign::Positive => Some(x),
                Sign::Negative => Some(-x),
            })
    }

    fn to_u64(&self) -> Option<u64> {
        self.data.to_u64()
    }

    fn to_u128(&self) -> Option<u128> {
        self.data.to_u128()
    }
}
