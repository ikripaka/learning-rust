use crate::biguint::BigUint;

pub(crate) enum Sign {
    Positive,
    Negative,
}

pub struct BigInt {
    sign: Sign,
    data: BigUint,
}
