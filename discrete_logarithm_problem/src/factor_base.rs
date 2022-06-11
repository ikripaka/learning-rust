use std::f64::consts::E;
use std::fmt;

use crate::factor_algorithms::is_prime;

pub struct FactorBase {
    alfa_coef_multipier: f64,
    base: Vec<i128>,
    n: i128,
}

impl fmt::Display for FactorBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "n:{}, base: {:?}", self.n, self.base)
    }
}

impl FactorBase {
    pub fn new(n: &u128, alfa_coef_multipier: &f64) -> Self {
        let mut base = FactorBase {
            alfa_coef_multipier: *alfa_coef_multipier,
            base: Vec::new(),
            n: (*n as i128),
        };
        base.make_factor_base();
        base
    }

    pub fn len(&self) -> usize {
        self.base.len()
    }

    //calculates factor base and saves it
    fn make_factor_base(&mut self) {
        let l = (E.powf(((self.n as f64).ln() * (self.n as f64).ln().ln()).sqrt()))
            .powf(self.alfa_coef_multipier / 2_f64.sqrt());
        self.base.push(-1_i128);
        for i in 2..l as i128 {
            if is_prime(&(i as u128), &10_u128).unwrap() && calc_legendre(&self.n, &i) == 1 {
                self.base.push(i);
            }
        }
    }

    pub fn is_smooth(&self, n: &i128) -> Option<Vec<i128>> {
        let mut num_factor_vec: Vec<i128> = vec![0; self.base.len()];
        let mut n = *n;
        if n < 0 {
            num_factor_vec[0] = 1;
            n = -n;
        }
        for (i, base) in self.base.iter().enumerate().skip(1) {
            while n % base == 0 && n != 0 {
                n /= base;
                num_factor_vec[i] += 1;
            }
        }

        if n != 1 || n == 0 {
            return None;
        }
        Some(num_factor_vec)
    }

    pub fn get_base(&self) -> Vec<i128> {
        self.base.clone()
    }
}

//calculates legendre symbol
fn calc_legendre(a: &i128, p: &i128) -> i8 {
    let mut a = a % p;
    let mut p = *p;
    let mut t = 1;
    while a != 0 {
        while a % 2 == 0 {
            a = a >> 1;
            let r = p % 8;
            if r == 3 || r == 5 {
                t = -t;
            }
        }
        (a, p) = (p, a);
        if a % 4 == 3 && p % 4 == 3 {
            t = -t;
        }
        a = a % p;
    }
    if p == 1 {
        return t;
    }
    return 0;
}

#[cfg(test)]
mod tests {
    use crate::factor_base::{calc_legendre, FactorBase};

    #[test]
    fn calculate_legendre_test() {
        assert_eq!(calc_legendre(&1649_i128, &2_i128), 1);
        assert_eq!(calc_legendre(&1649_i128, &3_i128), -1);
        assert_eq!(calc_legendre(&1649_i128, &17_i128), 0);
        assert_eq!(calc_legendre(&3_i128, &7_i128), -1);
        assert_eq!(calc_legendre(&9_i128, &11_i128), 1);
    }

    #[test]
    fn base_test() {
        let mut factor_base = FactorBase::new(&25511, &1.);
        factor_base.make_factor_base();
        println!("{}", factor_base);
    }
}
