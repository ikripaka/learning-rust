use crate::dlp_algorithms::silver_pohlig_hellman;
use num_bigint::BigInt;
use num_traits::One;

pub fn solve_log(
    verbose: bool,
    a: &BigInt,
    b: &BigInt,
    n: &BigInt,
) -> Result<BigInt, &'static str> {
    if verbose {
        println!(
            "Input data:\na:{}, b:{}, n:{}",
            a.to_str_radix(10),
            b.to_str_radix(10),
            n.to_str_radix(10)
        );
        return silver_pohlig_hellman(a, b, n, true);
    } else {
        return silver_pohlig_hellman(a, b, n, false);
    }
}
