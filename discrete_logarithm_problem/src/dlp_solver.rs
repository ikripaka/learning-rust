use num_bigint::BigInt;
use num_traits::One;

pub fn solve_log(verbose: bool, a: &BigInt, b:&BigInt, n:&BigInt) -> BigInt{
    if verbose{
        println!("input data:\na:{}, b:{}, n:{}", a.to_str_radix(10), b.to_str_radix(10), n.to_str_radix(10));
    }
    return BigInt::one();
}