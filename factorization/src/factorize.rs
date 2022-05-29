use crate::factor_algorithms::{
    brillhart_morrison, factor_trial_division, is_prime, rho_pollard_method,
    DEFAULT_PRIME_TEST_ITER, MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER,
};

// pick from range 0..1
const PICK_DEFAULT_VEC_INDEX_BRILLHART_MORRISON: usize = 0;

//big function that uses all factorization methods multiple times to factor number
pub fn factorize_number(n: &u128, verbose: bool) -> Vec<u128> {
    let mut result_vec = Vec::new();

    if is_prime(n, &DEFAULT_PRIME_TEST_ITER).unwrap() {
        result_vec.push(*n);
        return result_vec;
    }
    let mut n = *n;

    //trial division
    while let Some(divider) = factor_trial_division(&n, |x: u128| {
        if x <= 47 {
            return x;
        } else {
            return 47_u128;
        }
    }) {
        result_vec.push(divider);
        n /= divider;
    }

    //rho pollard
    if is_prime(&n, &DEFAULT_PRIME_TEST_ITER).unwrap() {
        result_vec.push(n);
        return result_vec;
    }

    while let Some(divider) = rho_pollard_method(&n) {
        result_vec.push(divider);
        n /= divider;
    }

    //brillhart morison
    if is_prime(&n, &DEFAULT_PRIME_TEST_ITER).unwrap() {
        result_vec.push(n);
        return result_vec;
    }

    let mut alfa_coef_multiplier: f64 = 1.;
    let mut counter: u8 = 0;
    while counter <= MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER {
        if is_prime(&n, &DEFAULT_PRIME_TEST_ITER).unwrap() {
            result_vec.push(n);
            return result_vec;
        }
        match brillhart_morrison(&n, &alfa_coef_multiplier) {
            Some(dividers) => {
                if dividers[0] == 1 && dividers[1] != 1 {
                    result_vec.push(dividers[1]);
                    n /= dividers[1];
                } else if dividers[0] != 1 && dividers[1] == 1 {
                    result_vec.push(dividers[0]);
                    n /= dividers[0];
                } else if dividers[0] != 1 && dividers[1] != 1 {
                    result_vec.push(dividers[PICK_DEFAULT_VEC_INDEX_BRILLHART_MORRISON]);
                    n /= dividers[PICK_DEFAULT_VEC_INDEX_BRILLHART_MORRISON];
                }
            }
            None => {
                if counter <= MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER {
                    alfa_coef_multiplier += 0.5;
                    counter += 1;
                } else {
                    println!("я не можу знайти канонiчний розклад числа :( ")
                }
            }
        }
    }

    return result_vec;
}

//Using all 3 algorithms only one time to factor number
pub fn factorize_all_algorithms(n: &u128, verbose: bool) -> Vec<u128> {
    let mut result_vec = Vec::new();
    if verbose == true {
        println!("Factorizing number: {}", n);
    }

    if is_prime(n, &DEFAULT_PRIME_TEST_ITER).unwrap() {
        result_vec.push(*n);
        return result_vec;
    }
    let mut n = *n;

    //trial division
    if verbose == true {
        println!("Input number in trial division: {}", n);
    }
    while let Some(divider) = factor_trial_division(&n, |x: u128| {
        if x <= 47 {
            return x;
        } else {
            return 47_u128;
        }
    }) {
        result_vec.push(divider);
        n /= divider;
        if verbose == true {
            println!("Trial division divider: {}", divider)
        }
    }

    //rho pollard
    if is_prime(&n, &DEFAULT_PRIME_TEST_ITER).unwrap() {
        result_vec.push(n);
        return result_vec;
    }
    if verbose == true {
        println!("Input number in rho pollard: {}", n);
    }
    match rho_pollard_method(&n) {
        Some(divider) => {
            result_vec.push(divider);
            n /= divider;
            if verbose == true {
                println!("Rho pollard dividers: {}", divider)
            }
        }
        _ => {}
    }

    //brillhart morison
    if is_prime(&n, &DEFAULT_PRIME_TEST_ITER).unwrap() {
        result_vec.push(n);
        return result_vec;
    }
    if verbose == true {
        println!("Input number in brillhart morrison: {}", n);
    }
    let mut alfa_coef_multiplier: f64 = 1.;
    let mut counter: u8 = 2;
    while counter <= MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER {
        if is_prime(&n, &DEFAULT_PRIME_TEST_ITER).unwrap() {
            result_vec.push(n);
            return result_vec;
        }
        match brillhart_morrison(&n, &alfa_coef_multiplier) {
            Some(dividers) => {
                if dividers[0] == 1 && dividers[1] != 1 {
                    result_vec.push(dividers[1]);
                    n /= dividers[1];
                    if verbose == true {
                        println!("Brillhart morrison dividers: {}, {}", dividers[1], n);
                    }
                } else if dividers[0] != 1 && dividers[1] == 1 {
                    result_vec.push(dividers[0]);
                    n /= dividers[0];
                    if verbose == true {
                        println!("Brillhart morrison dividers: {}, {}", dividers[0], n);
                    }
                } else if dividers[0] != 1 && dividers[1] != 1 {
                    result_vec.push(dividers[PICK_DEFAULT_VEC_INDEX_BRILLHART_MORRISON]);
                    n /= dividers[PICK_DEFAULT_VEC_INDEX_BRILLHART_MORRISON];
                    if verbose == true {
                        println!(
                            "Brillhart morrison dividers: {}, {}",
                            dividers[PICK_DEFAULT_VEC_INDEX_BRILLHART_MORRISON], n
                        );
                    }
                }
                result_vec.push(n);
                return result_vec;
            }
            None => {
                if counter <= MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER {
                    alfa_coef_multiplier += 0.5;
                    counter += 1;
                } else {
                    println!("я не можу знайти канонiчний розклад числа :( ");
                    break;
                }
            }
        }
    }
    return result_vec;
}

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use crate::factor_algorithms::{
        brillhart_morrison, is_prime, rho_pollard_method, DEFAULT_PRIME_TEST_ITER,
        MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER,
    };
    use crate::factorize::{
        factorize_all_algorithms, factorize_number, PICK_DEFAULT_VEC_INDEX_BRILLHART_MORRISON,
    };

    #[test]
    fn factor_test() {
        println!("{:?}", factorize_number(&25511, false));
        println!("{:?}", factorize_number(&49347803087, false));
        println!("{:?}", factorize_number(&40314824977, false));
        println!("{:?}", factorize_number(&56872139357, false));
        println!("{:?}", factorize_number(&25169050345, false));
        println!("{:?}", factorize_number(&17807879769, false));
        println!("{:?}", factorize_number(&37007068943, false));
        println!("{:?}", factorize_number(&36478753357, false));
        println!("{:?}", factorize_number(&44729396957, false));
        println!("{:?}", factorize_number(&62728529929, false));
        println!("{:?}", factorize_number(&20057299527, false));
    }

    #[test]
    fn factor_all_algorithms_test() {
        println!("{:?}", factorize_all_algorithms(&25511, false));
        println!("{:?}", factorize_all_algorithms(&49347803087, false));
        println!("{:?}", factorize_all_algorithms(&40314824977, false));
        println!("{:?}", factorize_all_algorithms(&56872139357, false));
        println!("{:?}", factorize_all_algorithms(&25169050345, false));
        println!("{:?}", factorize_all_algorithms(&17807879769, false));
        println!("{:?}", factorize_all_algorithms(&37007068943, false));
        println!("{:?}", factorize_all_algorithms(&36478753357, false));
        println!("{:?}", factorize_all_algorithms(&44729396957, false));
        println!("{:?}", factorize_all_algorithms(&62728529929, false));
        println!("{:?}", factorize_all_algorithms(&20057299527, false));
    }

    #[test]
    fn test_rho_pollard_method_bench() {
        let input_numbers: Vec<u128> = vec![
            9172639163, 9172639163862795, 8937716743, 278874899, 99400891, 116381389, 4252083239,
            6633776623, 227349247, 3568572617, 25511, 3973573, 10528813, 4248311, 10252159, 5746331, 2708561, 10169711, 4888993,
            12404297, 11587117,
        ];
        println!("Rho pollard bench test:");
        for i in 0..input_numbers.len() {
            let mut n: u128 = input_numbers[i];
            let now1 = Instant::now();
            while let Some(divider) = rho_pollard_method(&n) {
                n /= divider;
            }
            let now2 = Instant::now();
            println!("number: {}, duration:{:?}", i, now1.elapsed())
        }
    }

    #[test]
    fn brillhart_morrison_bench() {
        let input_numbers: Vec<u128> = vec![
            25511, 3973573, 10528813, 4248311, 10252159, 5746331, 2708561, 10169711, 4888993,
            12404297, 11587117];
        println!("Brillhart morrison bench test:");
        for i in 0..input_numbers.len() {
            let mut n: u128 = input_numbers[i];
            let now1 = Instant::now();
            let mut alfa_coef_multiplier: f64 = 1.;
            let mut counter: u8 = 0;
            while counter <= MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER {
                if is_prime(&n, &DEFAULT_PRIME_TEST_ITER).unwrap() {
                    counter = MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER + 1;
                    continue;
                }
                match brillhart_morrison(&n, &alfa_coef_multiplier) {
                    Some(dividers) => {
                        if dividers[0] == 1 && dividers[1] != 1 {
                            n /= dividers[1];
                        } else if dividers[0] != 1 && dividers[1] == 1 {
                            n /= dividers[0];
                        } else if dividers[0] != 1 && dividers[1] != 1 {
                            n /= dividers[PICK_DEFAULT_VEC_INDEX_BRILLHART_MORRISON];
                        }
                        println!("number: {}, duration:{:?}", input_numbers[i], now1.elapsed());
                    }
                    None => {
                        if counter <= MAX_FAILED_ATTEMPTS_TO_FACTORIZE_NUMBER {
                            alfa_coef_multiplier += 0.5;
                            counter += 1;
                        } else {
                            println!("я не можу знайти канонiчний розклад числа :( ")
                        }
                    }
                }
            }
            println!("number: {}, duration:{:?}", input_numbers[i], n)
        }
    }
}
