use crate::factorize::factorize_number;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};
use std::collections::HashMap;
use crate::factor_algorithms::brillhart_morrison;

// x = a (mod n)
#[derive(Clone)]
struct ModuleEquation {
    a: BigInt,
    n: BigInt,
}

// Solving dlp problem using Silver Pohlig Hellman algorithm
fn silver_pohlig_hellman<'a>(
    a: &'a BigInt,
    b: &'a BigInt,
    n: &'a BigInt,
) -> Result<BigInt, &'static str> {
    if *a > BigInt::from(std::u128::MAX)
        || *b > BigInt::from(std::u128::MAX)
        || *n > BigInt::from(std::u128::MAX)
    {
        return Err("values overflow u128, try to put smaller numbers");
    };

    // factorizing number "n-1"
    let mut module_factor = factorize_number(&(n - BigInt::one()).to_u128().unwrap(), false);
    //delete 1

    match module_factor.iter().position(|&x| x == 1_u128) {
        Some(index) => {
            module_factor.remove(index);
            ()
        }
        _ => (),
    }

    println!(
        "{} = {:?}",
        (n - BigInt::one()).to_u128().unwrap(),
        module_factor
    );

    //creating number powers vector
    // Hashmap < p_i, l_i >
    let mut map: HashMap<BigInt, BigInt> = HashMap::new();
    for num in module_factor.iter() {
        if map.contains_key(&BigInt::from(*num)) {
            map.insert(
                BigInt::from(*num),
                ((map.get(&BigInt::from(*num)).unwrap()) + BigInt::one()),
            );
        } else {
            map.insert(BigInt::from(*num), BigInt::one());
        }
    }

    //calculating tables
    // Hashmap < p_i, Hashmap< r_ij , j > >
    let mut precalculated_tables: HashMap<BigInt, HashMap<BigInt, BigInt>> = HashMap::new();
    for key in map.keys() {
        let mut pow_map: HashMap<BigInt, BigInt> = HashMap::new();
        for i in 0..(key.to_u128().unwrap()) {
            println!(
                "i:{}, key:{:?}, powmod:{:?}",
                i,
                key,
                a.modpow(&(((n - BigInt::one()) * BigInt::from(i)) / key), n)
            );
            pow_map.insert(
                a.modpow(&(((n - BigInt::one()) * BigInt::from(i)) / key), n),
                BigInt::from(i),
            );
        }
        println!("pow_map:{:?}", pow_map);
        precalculated_tables.insert(key.clone(), pow_map);
    }

    let equations: Vec<ModuleEquation> = Vec::new();
    let inverse_a = inverse(a, n);
    for key in map.keys() {
        for i in 0..(*map.get(&key).unwrap()).to_u128().unwrap() {}
    }
    println!("{:?}", precalculated_tables);

    Ok(BigInt::one())
}

// Agorithm to find inverse by module using Extended Euclides algorithm
fn inverse(a: &BigInt, n: &BigInt) -> Result<BigInt, &'static str> {
    let mut a_mut = a.clone();
    if a >= n {
        a_mut %= n;
    }

    let mut t = BigInt::zero();
    let mut r = n.clone();
    let mut new_t = BigInt::one();
    let mut new_r = a.clone();
    while new_r != BigInt::zero() {
        let quotient = &r / &new_r;
        let new_t_aux = t;
        t = new_t.clone();
        new_t = new_t_aux - &quotient * &new_t;
        let new_r_aux = r; //auxiliary
        r = new_r.clone();
        new_r = new_r_aux - &quotient * &new_r;
    }
    if t < BigInt::zero() {
        t += n;
    }
    Ok(t)
}

#[cfg(test)]
mod tests {
    use crate::dlp_algorithms::{inverse, silver_pohlig_hellman};
    use num_bigint::BigInt;
    use num_traits::{One, Zero};
    use std::u128::MAX;

    #[test]
    fn silver_pohlig_test() {
        // println!(
        //     "{}",
        //     silver_pohlig_hellman(
        //         &BigInt::from(10_u128),
        //         &BigInt::from(13_u128),
        //         &BigInt::from(29_u128),
        //     )
        //         .unwrap_or_else(|err| {
        //             eprintln!("an error occurred {}", err);
        //             BigInt::zero()
        //         })
        // );

        println!(
            "{}",
            silver_pohlig_hellman(
                &BigInt::from(3_u128),
                &BigInt::from(15_u128),
                &BigInt::from(43_u128),
            )
                .unwrap_or_else(|err| {
                    eprintln!("an error occurred {}", err);
                    BigInt::zero()
                })
        );

        println!(
            "{}",
            silver_pohlig_hellman(
                &BigInt::from(5_u128),
                &BigInt::from(11_u128),
                &BigInt::from(73_u128),
            )
                .unwrap_or_else(|err| {
                    eprintln!("an error occurred {}", err);
                    BigInt::zero()
                })
        );
        // println!("{}", MAX);
        // println!("{}", silver_pohlig_hellman(&BigInt::from(10_u128), &BigInt::from(13_u128), &BigInt::from(MAX)).unwrap_or_else(|err| {
        //     eprintln!("an error occurred {}", err);
        //     BigInt::zero()
        // }));
    }

    #[test]
    fn reverse_test() {
        println!(
            "5^-1 mod 73 = {}",
            inverse(&BigInt::from(5), &BigInt::from(73)).unwrap_or_else(|err| {
                eprintln!("an error occurred {}", err);
                BigInt::zero()
            })
        );
        assert_eq!(
            inverse(&BigInt::from(5), &BigInt::from(73)).unwrap_or_else(|err| {
                eprintln!("an error occurred {}", err);
                BigInt::zero()
            }),
            BigInt::from(44)
        );

        println!(
            "3^-1 mod 43 = {}",
            inverse(&BigInt::from(3), &BigInt::from(43)).unwrap_or_else(|err| {
                eprintln!("an error occurred {}", err);
                BigInt::zero()
            })
        );

        assert_eq!(
            inverse(&BigInt::from(5), &BigInt::from(73)).unwrap_or_else(|err| {
                eprintln!("an error occurred {}", err);
                BigInt::zero()
            }),
            BigInt::from(44)
        );

        println!(
            "123^-1 mod 4272331909 = {}",
            inverse(&BigInt::from(123_u128), &BigInt::from(4272331909_u128)).unwrap_or_else(|err| {
                eprintln!("an error occurred {}", err);
                BigInt::zero()
            })
        );
        assert_eq!(
            inverse(&BigInt::from(123_u128), &BigInt::from(4272331909_u128)).unwrap_or_else(
                |err| {
                    eprintln!("an error occurred {}", err);
                    BigInt::zero()
                }
            ),
            BigInt::from(590484898_u128)
        );
    }
}
