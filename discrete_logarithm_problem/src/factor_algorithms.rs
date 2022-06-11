use std::cmp::min;
use std::io;

use num_bigint::{BigInt};
use num_traits::cast::ToPrimitive;
use rand::{Rng};
use unicode_segmentation::UnicodeSegmentation;

use crate::chain_fraction::ChainFraction;
use crate::factor_base::FactorBase;

pub const DEFAULT_PRIME_TEST_ITER: u128 = 100;
pub const MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER: u8 = 3;

static GEN_RANDOM_VALUE: fn(u128, u128) -> u128 = |begin: u128, end: u128| -> u128 {
    let mut rng = rand::thread_rng();
    rng.gen_range(begin..=end)
};

/// Number is testing for primality with Miller-Rabin test
/// n - number that would be tested
/// k - iterations of random generating bases
pub fn is_prime(p: &u128, k: &u128) -> Result<bool, io::Error> {
    if *p == 1 {
        return Ok(true);
    }

    let mut s: u128 = 0;
    let mut d = *p - 1;
    while d % 2 == 0 && d != 0 {
        d /= 2;
        s += 1;
    }

    for _ in 1..*k {
        let mut counter: u128 = 0;
        let mut x = GEN_RANDOM_VALUE(1, *p - 1);
        if gcd(&x, &p) > 1 {
            return Ok(false);
        }

        // println!("d: {}, x: {}, ", x, d);

        // x = x.pow(d as u32) % p;
        x = pow_mod(&x, &d, p).unwrap();
        // println!("x: {}, ", x);
        if x == 1 || x == p - 1 {
            continue;
        }

        if s < 1 {
            return Ok(false);
        }
        for _ in 1..s {
            x = (x * x) % p;
            if x == p - 1 {
                break;
            }
            counter += 1;
        }
        if counter == s - 1 && x != p - 1 && x != 1 {
            return Ok(false);
        }
    }
    return Ok(true);
}

/// Performs trial division algorithm
/// Number is tested for division on elements that < sqrt(n)
pub fn factor_trial_division(n: &u128, get_bound: fn(n: u128) -> u128) -> Option<u128> {
    let mut prime_test_iter: u128;
    for i in 2..=get_bound(*n) {
        //(f64::sqrt(*n as f64).floor()
        if i < DEFAULT_PRIME_TEST_ITER {
            prime_test_iter = i;
        } else {
            prime_test_iter = DEFAULT_PRIME_TEST_ITER;
        }
        if is_prime(&i, &prime_test_iter).unwrap() && is_dividing_dec(&n, &i).unwrap() {
            return Some(i);
        }
    }
    None
}

/// Performs rho pollard method
/// Finding 1 number n factor
pub fn rho_pollard_method(n: &u128) -> Option<u128> {
    let pollard_function = |x: u128, n: u128| -> u128 { (x * x + 1) % n };
    let mod_difference: fn(u128, u128, u128) -> u128 = |a: u128, b: u128, module| -> u128 {
        if a > b {
            module - (a - b)
        } else {
            module - (b - a)
        }
    };
    let mut counter = 0;

    let mut xi: u128 = 1;
    let mut yi: u128 = xi;
    let mut gcd: u128 = 1;

    while gcd == 1 {
        xi = pollard_function(xi, *n);
        yi = pollard_function(pollard_function(yi, *n), *n);
        gcd = self::gcd(&(mod_difference(xi, yi, *n)), n);
        if xi == yi && counter <= MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER {
            xi = GEN_RANDOM_VALUE(1, *n);
            yi = xi;
            counter += 1;
        } else if xi == yi && counter > MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER {
            return None;
        }
    }
    if gcd == 1 {
        return None;
    }
    Some(gcd)
}

//Brillhart-Morrison method
/// Finding 2 number n factor
pub fn brillhart_morrison(n: &u128, alfa_coef_multiplier: &f64) -> Option<Vec<u128>> {
    let factor_base = FactorBase::new(n, alfa_coef_multiplier);
    let mut smooth_num_factor = Vec::new();
    let mut smooth_num_index = Vec::new();
    let mut b_s_vec = Vec::new();
    let mut chain_fractions = ChainFraction::new(n);

    while factor_base.len() + 1 > smooth_num_factor.len() {
        let pseudo_smooth = chain_fractions.next().unwrap();
        let curr_b_s = chain_fractions.get_last_b_s();
        match factor_base.is_smooth(&pseudo_smooth) {
            Some(vec) => {
                smooth_num_factor.push(vec);
                smooth_num_index.push(chain_fractions.get_index());
                b_s_vec.push(curr_b_s)
            }
            None => continue,
        }
    }

    let add_vec = |x: &Vec<i128>, y: &Vec<i128>, is_mod2: bool| -> Vec<i128> {
        let mut c: Vec<i128> = Vec::new();
        for i in 0..x.len() {
            if is_mod2 {
                c.push((x[i] + y[i]) % 2);
            } else {
                c.push(x[i] + y[i]);
            }
        }
        c
    };
    let is_zero = |x: &Vec<i128>| -> bool {
        for i in 0..x.len() {
            if x[i] != 0 {
                return false;
            }
        }
        true
    };

    //go through all possible solutions
    let ceiling_bound = BigInt::from(2)
        .pow((factor_base.len() + 1) as u32)
        .to_u128()
        .unwrap();
    for i in 1..ceiling_bound {
        let mut result_vec = vec![0; factor_base.len()];
        let mut inequality_solution = Vec::new();
        for j in 0..(factor_base.len() + 1) as u128 {
            if (i & (1_u128 << factor_base.len() - (j as usize)))
                >> (factor_base.len() - (j as usize))
                == 1
            {
                inequality_solution.push(j);
                result_vec = add_vec(&result_vec, &smooth_num_factor[j as usize], true)
            }
        }
        if is_zero(&result_vec) {
            let mut x = 1;
            let mut y = vec![0; factor_base.len()];
            for k in inequality_solution.iter() {
                y = add_vec(&y, &smooth_num_factor[*k as usize], false);
                x = (x * b_s_vec[*k as usize]) % (*n as i128);
            }
            for i in 0..y.len() {
                y[i] /= 2;
            }

            let y = get_num_by_factor_base(&factor_base, &y, n);
            if x != y || x != (-y + *n as i128) {
                let gcd1 = gcd(&(((x + y) % *n as i128) as u128), n);
                let gcd2 = gcd(&(((x - y) % *n as i128) as u128), n);

                let prime = is_prime(n, &200).unwrap();
                if gcd1 == 1 && gcd2 == 1 && prime {
                    return Some(vec![gcd1, *n]);
                } else if gcd1 != 1 && gcd2 != 1 {
                    return Some(vec![gcd1, gcd2]);
                }
            }
        }
    }
    None
}

//Transforms factor base vector to number
fn get_num_by_factor_base(factor_base: &FactorBase, power_vec: &Vec<i128>, n: &u128) -> i128 {
    let mut x = BigInt::from(1);
    for (i, k) in factor_base.get_base().iter().enumerate() {
        x = (x * BigInt::from(*k).modpow(&BigInt::from(power_vec[i]), &BigInt::from(*n)))
            % BigInt::from(*n);
    }
    x.to_i128().unwrap()
}

/// Determines the sign of Pascal's divisibility
/// decimal
fn is_dividing_dec(n: &u128, m: &u128) -> Result<bool, io::Error> {
    let n_string_rev: String = format!("{}", n).graphemes(true).rev().collect();
    let mut sum: u128 = 0;
    let radix = 10;

    let mut r_i = 1;
    for digit in n_string_rev.chars() {
        r_i = (r_i * radix) % m;
        sum = (sum + (digit.to_digit(10).unwrap() as u128) * r_i) % m;
    }
    if sum % m == 0 && *n % m == sum {
        return Ok(true);
    }
    return Ok(false);
}

/// raises number to the power with Horner scheme
#[warn(dead_code)]
pub fn pow_mod_horner(a: &u128, pow: &u128, module: &u128) -> Result<u128, io::Error> {
    let pow_binary: String = format!("{:b}", *pow);
    let pow_binary_len = format!("{:b}", *pow).graphemes(true).count();
    let mut c = 1;
    for (i, bit) in pow_binary.chars().enumerate() {
        if bit.to_digit(10).unwrap() == 1 {
            c = (c * a) % module;
        }
        if i < pow_binary_len - 1 {
            c = (c * c) % module;
        }
    }
    return Ok(c);
}

/// raises number to the power with Horner scheme
pub fn pow_mod(a: &u128, pow: &u128, module: &u128) -> Option<u128> {
    BigInt::from(*a)
        .modpow(&BigInt::from(*pow), &BigInt::from(*module))
        .to_u128()
}

/// Using euclid algorithm to calculate GCD
fn gcd(a: &u128, b: &u128) -> u128 {
    if *a == 0 || *b == 0 {
        return 1;
    }
    let mut a = *a;
    let mut b = *b;
    let mut d = 1;

    while (a & 1) == 0 && (b & 1) == 0 {
        a >>= 1;
        b >>= 1;
        d <<= 1;
    }
    while (a & 1) == 0 {
        a >>= 1;
    }
    while b != 0 {
        while (b & 1) == 0 {
            b >>= 1;
        }

        if a > b {
            (a, b) = (min(a, b), a - b);
        } else {
            (a, b) = (min(a, b), b - a);
        }
    }

    return d * a;
}

/// Using property of GCD to calculate LCM
/// lcm(a,b) = a*b/gcd(a,b)
#[warn(dead_code)]
fn lcm(a: &u128, b: &u128) -> u128 {
    return (a * b) / gcd(a, b);
}

#[cfg(test)]
mod tests {
    use crate::factor_algorithms::{
        brillhart_morrison, gcd, is_dividing_dec, is_prime, lcm, pow_mod, rho_pollard_method,
    };
    use crate::factor_base::FactorBase;

    #[test]
    fn rho_pollard_method_test() {
        assert_eq!(rho_pollard_method(&8051).unwrap(), 97);
        assert_eq!(rho_pollard_method(&8633).unwrap(), 97);
        assert_eq!(rho_pollard_method(&25511).unwrap(), 97);
        assert_eq!(rho_pollard_method(&3973573).unwrap(), 397);
        assert_eq!(rho_pollard_method(&10528813).unwrap(), 1049);
        assert_eq!(rho_pollard_method(&4248311).unwrap(), 421);
        assert_eq!(rho_pollard_method(&10252159).unwrap(), 1019);
        assert_eq!(rho_pollard_method(&5746331).unwrap(), 569);
        assert_eq!(rho_pollard_method(&2708561).unwrap(), 269);
        assert_eq!(rho_pollard_method(&10169711).unwrap(), 1009);
        assert_eq!(rho_pollard_method(&4888993).unwrap(), 487);
        assert_eq!(rho_pollard_method(&12404297).unwrap(), 1229);
        assert_eq!(rho_pollard_method(&11587117).unwrap(), 1151);
    }

    #[test]
    fn is_dividing_dec_test() {
        assert_eq!(is_dividing_dec(&160, &4).unwrap(), true);
        assert_eq!(is_dividing_dec(&9680, &11).unwrap(), true);
        assert_eq!(is_dividing_dec(&5, &4).unwrap(), false);
        assert_eq!(is_dividing_dec(&1001, &13).unwrap(), true);
        assert_eq!(is_dividing_dec(&2147483647, &78).unwrap(), false);
        assert_eq!(is_dividing_dec(&193, &16).unwrap(), false);
        assert_eq!(is_dividing_dec(&524287, &13).unwrap(), false);
        assert_eq!(is_dividing_dec(&1052, &11).unwrap(), false);
        assert_eq!(is_dividing_dec(&17, &4).unwrap(), false);
        assert_eq!(is_dividing_dec(&409, &1).unwrap(), true);
    }

    #[test]
    fn is_prime_rand_test() {
        assert_eq!(is_prime(&9172639163, &200).unwrap(), false);
        assert_eq!(is_prime(&8, &6).unwrap(), false);
        assert_eq!(is_prime(&13, &6).unwrap(), true);
        assert_eq!(is_prime(&47, &10).unwrap(), true);
        assert_eq!(is_prime(&384, &50).unwrap(), false);
        assert_eq!(is_prime(&1052, &50).unwrap(), false);
        assert_eq!(is_prime(&409, &50).unwrap(), true);
        assert_eq!(is_prime(&193, &25).unwrap(), true);
        assert_eq!(is_prime(&524287, &50).unwrap(), true);
        assert_eq!(is_prime(&524286, &50).unwrap(), false);
        assert_eq!(is_prime(&2147483647, &200).unwrap(), true);
        assert_eq!(is_prime(&2137, &100).unwrap(), true);
    }

    #[test]
    fn pow_mod_test() {
        assert_eq!(pow_mod(&4294967295, &23, &2345).unwrap(), 2000);
        assert_eq!(pow_mod(&7295429496, &7, &7861).unwrap(), 3554);
        assert_eq!(pow_mod(&4681813, &41, &52).unwrap(), 41);
        assert_eq!(pow_mod(&15026749, &15, &3).unwrap(), 1);
        assert_eq!(pow_mod(&148028402, &46, &777).unwrap(), 508);
        assert_eq!(pow_mod(&851407573, &28, &217).unwrap(), 51);
        assert_eq!(pow_mod(&160722851, &32, &958).unwrap(), 587);
        assert_eq!(pow_mod(&903269, &51, &896541).unwrap(), 206423);
        assert_eq!(pow_mod(&211525930, &8, &12345879).unwrap(), 954604);
        assert_eq!(pow_mod(&3596, &84, &56).unwrap(), 8);
    }

    #[test]
    fn gcd_test() {
        assert_eq!(gcd(&2, &8), 2);
        assert_eq!(gcd(&1, &78954), 1);
        assert_eq!(gcd(&57, &78954), 3);
        assert_eq!(gcd(&89654123, &7895123), 1);
        assert_eq!(gcd(&12483185, &115970855), 12155);
        assert_eq!(gcd(&14591695, &324597), 41);
        assert_eq!(gcd(&64675741, &3212349), 1);
        assert_eq!(gcd(&9212349, &587475741), 3);
        assert_eq!(gcd(&22815595, &(u64::MAX as u128)), 5);
        assert_eq!(gcd(&(u64::MAX as u128), &(u32::MAX as u128)), 4294967295);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(&2, &8), 8);
        assert_eq!(lcm(&1, &78954), 78954);
        assert_eq!(lcm(&57, &78954), 1500126);
        assert_eq!(lcm(&89654123, &7895123), 707830328542129);
        assert_eq!(lcm(&12483185, &115970855), 119102068085);
        assert_eq!(lcm(&14591695, &324597), 115522449315);
        assert_eq!(lcm(&64675741, &3212349), 207761051925609);
        assert_eq!(lcm(&9212349, &587475741), 1804010518375203);
        assert_eq!(
            lcm(&19_598_446_868_193_105, &(u64::MAX as u128)),
            84174688370881455455887185
        );
        assert_eq!(
            lcm(&(u64::MAX as u128), &(u32::MAX as u128)),
            18446744073709551615
        );
    }
}
