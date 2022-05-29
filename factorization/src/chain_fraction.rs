extern crate integer_sqrt;

use std::fmt;

use integer_sqrt::IntegerSquareRoot;

pub struct ChainFraction {
    i: usize,
    v_i: i128,
    a_i: i128,
    u_i: i128,
    n_sqrt: i128,
    n: i128,
    a_s: Vec<i128>,
    b_s: Vec<i128>,
    b_squared: Vec<i128>,
}

impl ChainFraction {
    pub fn new(n: &u128) -> Self {
        let sqrt_n = (*n as i128).integer_sqrt();
        ChainFraction {
            i: 0,
            v_i: 1_i128,
            a_i: sqrt_n,
            u_i: sqrt_n,
            n_sqrt: sqrt_n,
            n: *n as i128,
            a_s: vec![0, sqrt_n],
            b_s: vec![1, sqrt_n],
            b_squared: vec![1; 2],
        }
    }
    pub fn get_index(&self) -> usize {
        self.i
    }
    pub fn get_last_b_s(&self) -> i128 {
        self.b_s[self.i]
    }
}

impl ChainFraction {
    fn get_calculations(&self) -> (Vec<i128>, Vec<i128>, Vec<i128>) {
        (self.a_s.clone(), self.b_s.clone(), self.b_squared.clone())
    }
}

impl Iterator for ChainFraction {
    type Item = i128;

    //gets nex chain fraction number
    fn next(&mut self) -> Option<Self::Item> {
        //reducing number from (25281 -> -230)
        // let reduce_number = |x: &i128, n: &i128| if *x > n * 3 / 5 { x - n } else { *x };
        let reduce_number = |x: &i128, n: &i128| match *x > n * 3 / 5 {
            true => x - n,
            false => *x,
        };

        if self.i == 0 {
            self.b_squared[self.i + 1] = reduce_number(
                &((self.b_s[self.i + 1] * self.b_s[self.i + 1]) % self.n),
                &self.n,
            );
        } else {
            self.v_i = (self.n - self.u_i.pow(2)) / self.v_i;
            self.a_i = (self.n_sqrt + self.u_i) / self.v_i;
            self.u_i = self.a_i * self.v_i - self.u_i;

            self.a_s.push(self.a_i);
            self.b_s.push(
                (self.a_s.last().unwrap() * self.b_s.last().unwrap()
                    + self.b_s[self.b_s.len() - 2])
                    % self.n,
            );
            self.b_squared.push(reduce_number(
                &(self.b_s.last().unwrap() * self.b_s.last().unwrap() % self.n),
                &self.n,
            ));
        }
        self.i += 1;
        Some(*self.b_squared.last().unwrap())
    }
}

impl fmt::Display for ChainFraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "\na_s:  {:?}\nb_s  : {:?}\nb_s^2: {:?}\n",
            self.a_s, self.b_s, self.b_squared
        )
    }
}

#[cfg(test)]
mod tests {
    use crate::chain_fraction;
    use crate::chain_fraction::ChainFraction;

    #[test]
    fn calculate_legendre_test() {
        let mut chain = ChainFraction::new(&25511);
        for _ in 0..12 {
            println!("{}", chain.next().unwrap())
        }
        println!("{}", chain);
    }
}
