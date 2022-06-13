use crate::factorize::factorize_number;
use num_bigint::BigInt;
use num_traits::{One, ToPrimitive, Zero};
use std::collections::HashMap;
use std::ptr::eq;

// x = a (mod n)
#[derive(Clone, Debug)]
pub struct ModuleEquation {
    a: BigInt,
    n: BigInt,
}

// Solving dlp problem using Silver Pohlig Hellman algorithm
pub fn silver_pohlig_hellman<'a>(
    a: &'a BigInt,
    b: &'a BigInt,
    n: &'a BigInt,
    verbose: bool,
) -> Result<BigInt, &'static str> {
    if *a > BigInt::from(std::u128::MAX)
        || *b > BigInt::from(std::u128::MAX)
        || *n > BigInt::from(std::u128::MAX)
    {
        return Err("values overflow u128, try to put smaller numbers");
    };

    // factorizing number "n-1"
    let mut module_factor = factorize_number(&(n - BigInt::one()).to_u128().unwrap(), false);

    //delete one '1'
    match module_factor.iter().position(|&x| x == 1_u128) {
        Some(index) => {
            module_factor.remove(index);
            ()
        }
        _ => (),
    }

    if verbose {
        println!(
            "\nSolving log {} ({}) = x\nFactor of 'n-1' {} = {:?}",
            a,
            b,
            (n - BigInt::one()).to_u128().unwrap(),
            module_factor
        );
    }

    //creating number powers vector
    // Hashmap < p_i, l_i >
    let mut map: HashMap<BigInt, BigInt> = HashMap::new();
    for num in module_factor.iter() {
        if map.contains_key(&BigInt::from(*num)) {
            map.insert(
                BigInt::from(*num),
                (map.get(&BigInt::from(*num)).unwrap()) + BigInt::one(),
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
            if verbose {
                println!(
                    "i:{}, key:{:?}, powmod:{:?}",
                    i,
                    key,
                    a.modpow(&(((n - BigInt::one()) * BigInt::from(i)) / key), n)
                );
            }

            pow_map.insert(
                a.modpow(&(((n - BigInt::one()) * BigInt::from(i)) / key), n),
                BigInt::from(i),
            );
        }

        if verbose {
            println!("Power hashmap:{:?}", pow_map);
        }

        precalculated_tables.insert(key.clone(), pow_map);
    }

    let mut equations: Vec<ModuleEquation> = Vec::new();
    let inverse_a = inverse(a, n).unwrap();
    for key in map.keys() {
        let mut x = BigInt::zero();
        let p_i_hashmap = precalculated_tables.get(key).unwrap();

        if verbose {
            println!(
                "\nIterations to calculate x for number '{}' :{}",
                key,
                (*map.get(&key).unwrap()).to_u128().unwrap()
            );
        }

        for i in 0..(*map.get(&key).unwrap()).to_u128().unwrap() {
            let x_i = p_i_hashmap
                .get(
                    &((b * inverse_a.modpow(&x, n))
                        .modpow(&((n - BigInt::one()) / key.pow((i + 1) as u32)), n)),
                )
                .unwrap();
            x += x_i * key.pow(i as u32);

            if verbose {
                println!(
                    "p_i:{}, i:{}, x_i: {}, x: {}, x_i * 2^{}: {}",
                    key,
                    i,
                    x_i,
                    x,
                    i,
                    x_i * BigInt::from(2).pow(i as u32)
                );
            }
        }
        if verbose {
            println!("x for '{}': {}", key, x);
        }
        equations.push(ModuleEquation {
            a: x,
            n: key.pow((*map.get(&key).unwrap()).to_u32().unwrap()),
        })
    }
    if verbose {
        println!("Precalculated tables: {:?}", precalculated_tables);
        println!("Equations x = a (mod n) : {:?}", equations);
    }

    match solve_equations(&equations, n, verbose) {
        Ok(result) => {
            if verbose {
                println!("Result: x = {}\n", result);
            }
            return Ok(result);
        }
        Err(_) => return Err("failed to solve equations"),
    }
}

// solving module equations by using (Generalized Chinese Remainder Theorem)
fn solve_equations(
    equations_vec: &Vec<ModuleEquation>,
    n: &BigInt,
    verbose: bool,
) -> Result<BigInt, &'static str> {
    let mut m_i = vec![BigInt::zero(); equations_vec.len()];
    let mut m = BigInt::one();

    for equation in equations_vec.iter() {
        m *= &equation.n;
    }
    for i in 0..equations_vec.len() {
        m_i[i] = &m/&equations_vec[i].n;
    }

    let mut n_i = Vec::new();
    for i in 0..equations_vec.len() {
        n_i.push(inverse(&m_i[i], &equations_vec[i].n).unwrap())
    }

    if verbose {
        println!("Solving equations with : M_i: {:?},\n N_i: {:?}", m_i, n_i);
    }

    let mut x = BigInt::zero();
    for i in 0..equations_vec.len() {
        if verbose {
            println!(
                "i:{}, x_i: {}, m_i: {}, n_i: {}, multiply (x_i*m_i*n_i) : {}",
                i,
                equations_vec[i].a,
                m_i[i],
                n_i[i],
                (&equations_vec[i].a * &m_i[i] * &n_i[i])
            );
        }

        x += (&equations_vec[i].a * &m_i[i] * &n_i[i]) % &(n - BigInt::one());
    }
    if verbose {
        println!(
            "module: {}, {} mod n = {}",
            n - BigInt::one(),
            x,
            &x % &(n - BigInt::one())
        );
    }

    return Ok(x % &(n - BigInt::one()));
}

// Algorithm to find inverse by module using Extended Euclides algorithm
fn inverse(a: &BigInt, n: &BigInt) -> Result<BigInt, &'static str> {
    let mut a_mut = a.clone();
    if a >= n {
        a_mut %= n;
    }

    let mut t = BigInt::zero();
    let mut r = n.clone();
    let mut new_t = BigInt::one();
    let mut new_r = a_mut.clone();
    while new_r != BigInt::zero() {
        let quotient = &r / &new_r;
        let new_t_aux = t;
        t = new_t.clone();
        new_t = new_t_aux - &quotient * &new_t;
        let new_r_aux = r; //auxiliary
        r = new_r.clone();
        new_r = new_r_aux - &quotient * &new_r;
    }
    if r > BigInt::one() {
        return Err("number is not invertible");
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
    use num_traits::Zero;
    use std::time::Instant;
    use std::u128::MAX;

    #[test]
    fn silver_pohlig_test() {
        println!(
            "{}",
            silver_pohlig_hellman(
                &BigInt::from(10_u128),
                &BigInt::from(13_u128),
                &BigInt::from(29_u128),
                true,
            )
                .unwrap_or_else(|err| {
                    eprintln!("an error occurred {}", err);
                    BigInt::zero()
                })
        );

        println!(
            "{}",
            silver_pohlig_hellman(
                &BigInt::from(3_u128),
                &BigInt::from(15_u128),
                &BigInt::from(43_u128),
                false,
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
                &BigInt::from(97_u128),
                false,
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
                false,
            )
                .unwrap_or_else(|err| {
                    eprintln!("an error occurred {}", err);
                    BigInt::zero()
                })
        );
        println!(
            "{}",
            silver_pohlig_hellman(
                &BigInt::from(1517),
                &BigInt::from(86875),
                &BigInt::from(181243),
                false,
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
            inverse(&BigInt::from(123_u128), &BigInt::from(4272331909_u128)).unwrap_or_else(
                |err| {
                    eprintln!("an error occurred {}", err);
                    BigInt::zero()
                }
            )
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

    #[test]
    fn silver_pohlig_hellman_bench() {
        let input_numbers = vec![
            (
                BigInt::from(77783),
                BigInt::from(78557),
                BigInt::from(79939),
            ),
            (BigInt::from(21), BigInt::from(28), BigInt::from(53)),
            (BigInt::from(364), BigInt::from(50), BigInt::from(401)),
            (
                BigInt::from(77783),
                BigInt::from(78557),
                BigInt::from(79939),
            ),
            (
                BigInt::from(77783),
                BigInt::from(78557),
                BigInt::from(79939),
            ),
        ];
        println!("Silver Pohlig Hellman bench test:");
        for i in 0..input_numbers.len() {
            let input_data = &input_numbers[i];
            let now1 = Instant::now();
            println!(
                "number: {:?}, duration:{:?},\n log {} ({}) mod {} = {}",
                input_numbers[i],
                now1.elapsed(),
                &input_data.0,
                &input_data.1,
                &input_data.2,
                silver_pohlig_hellman(&input_data.0, &input_data.1, &input_data.2, false).unwrap()
            );
        }
    }

    #[test]
    fn silver_pohlig_hellman_test_one() {
        let input_numbers = vec![(
            BigInt::from(2_u128),
            BigInt::from(1_u128),
            BigInt::from(3_u128),
        )];
        println!("Silver Pohlig Hellman bench test:");

        let input_data = &input_numbers[0];
        let now1 = Instant::now();
        println!(
            "number: {:?}, duration:{:?},\n log {} ({}) mod {} = {}",
            input_numbers[0],
            now1.elapsed(),
            &input_data.0,
            &input_data.1,
            &input_data.2,
            silver_pohlig_hellman(&input_data.0, &input_data.1, &input_data.2, true).unwrap()
        );
    }
}
