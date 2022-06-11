use num_bigint::BigInt;
use num_traits::Zero;
use std::error::Error;
use std::fs::File;
use std::str::FromStr;
use std::{env, process};

// log a (x) = b (mod n)
// cargo run -- -v 370,670,911
//                  a,  b,  c
// cargo run -- 370,670,911
// cargo run -- file.csv
#[derive(Debug)]
struct Arguments {
    verbose: bool,
    a: BigInt,
    b: BigInt,
    n: BigInt,
    file: bool,
    file_path: String,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() == 1 || args.len() == 0 {
            return Err("not enough arguments");
        } else if args.len() > 4 {
            return Err("to many arguments");
        }
        let arg1 = args[1].clone();

        let number_vec: Vec<&str> = args[1].split(",").collect();

        if number_vec.len() == 3 && args.len() == 2 {
            let num1 = BigInt::from_str(&number_vec[0]);
            let num2 = BigInt::from_str(&number_vec[1]);
            let num3 = BigInt::from_str(&number_vec[2]);
            if num1.is_ok() && num2.is_ok() && num3.is_ok() {
                let num1 = num1.unwrap();
                let num2 = num2.unwrap();
                let num3 = num3.unwrap();
                if num1 <= BigInt::zero() || num2 <= BigInt::zero() || num3 <= BigInt::zero() {
                    return Err("invalid input");
                }
                return Ok(Arguments {
                    verbose: false,
                    a: num1,
                    b: num2,
                    n: num3,
                    file: false,
                    file_path: String::new(),
                });
            } else {
                return Err("troubles with converting number");
            }
        } else if arg1.contains("-v") || arg1.contains("-V") && args.len() == 3 {
            // cargo run -- -v 370,670,911
            let number_vec: Vec<&str> = args[2].split(",").collect();
            match u128::from_str_radix(&number_vec[0], 10){
                Ok(_)=>(),
                Err(err) => Err(format!("u128 overflow/err: {}", err))
            }
            match u128::from_str_radix(&number_vec[1], 10){
                Ok(_)=>(),
                Err(err) => Err(format!("u128 overflow/err: {}", err))
            }
            match u128::from_str_radix(&number_vec[2], 10){
                Ok(_)=>(),
                Err(err) => Err(format!("u128 overflow/err: {}", err))
            }


            let num1 = BigInt::from_str(&number_vec[0]);
            let num2 = BigInt::from_str(&number_vec[1]);
            let num3 = BigInt::from_str(&number_vec[2]);
            if num1.is_ok() && num2.is_ok() && num3.is_ok() {
                let num1 = num1.unwrap();
                let num2 = num2.unwrap();
                let num3 = num3.unwrap();
                if num1 <= BigInt::zero() || num2 <= BigInt::zero() || num3 <= BigInt::zero() {
                    return Err("invalid input");
                }
                return Ok(Arguments {
                    verbose: true,
                    a: num1,
                    b: num2,
                    n: num3,
                    file: false,
                    file_path: String::new(),
                });
            } else {
                return Err("troubles with converting number");
            }
        } else if arg1.contains(".csv") && std::path::Path::new(&arg1).exists() {
            // cargo run -- file.csv
            return Ok(Arguments {
                verbose: false,
                n: BigInt::zero(),
                a: BigInt::zero(),
                b: BigInt::zero(),
                file: true,
                file_path: arg1,
            });
        }
        return Err("invalid syntax/or file path");
    }
}

fn process_arguments(args: &Arguments, output_filename: &String) -> Result<(), Box<dyn Error>> {
    if args.file == false {
        if args.verbose == true {
            println!(
                "{:?}",
                discrete_logarithm_problem::dlp_solver::solve_log(true, &args.a, &args.b, &args.n)
            );
            println!("solving log verbose")
        } else {
            println!(
                "{:?}",
                discrete_logarithm_problem::dlp_solver::solve_log(false, &args.a, &args.b, &args.n)
            );
            println!("solving log")
        }
        return Ok(());
    } else {
        let num_vec: Vec<(BigInt, BigInt, BigInt)> =
            read_csv(&args.file_path).unwrap_or_else(|err| {
                eprintln!("{} problem parsing arguments from .csv file", err);
                process::exit(0);
            });
        let file = File::create(output_filename).expect("Error encountered while creating file!");
        let mut wtr = csv::WriterBuilder::new().from_writer(file);
        for num in num_vec.iter() {
            println!("solving log in csv file {:?}", *num);
            let string_vec: Vec<String> = vec![discrete_logarithm_problem::dlp_solver::solve_log(
                false, &num.0, &num.1, &num.2,
            )
            .to_str_radix(10)];
            wtr.write_record(&string_vec)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

fn read_csv(file_path: &String) -> Result<Vec<(BigInt, BigInt, BigInt)>, Box<dyn Error>> {
    let rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path);
    let mut num_vec = Vec::new();
    for result in rdr.unwrap().records() {
        let csv_record = result?;

        num_vec.push((
            BigInt::from_str(csv_record.get(0).unwrap_or_else(|| {
                eprintln!(
                    "{} / failed to read / extract numbers from .csv",
                    "problem with 1 csv argument"
                );
                process::exit(0);
            }))
            .unwrap_or_else(|err| {
                eprintln!(
                    "{} / failed to read / extract numbers from .csv",
                    err
                );
                process::exit(0);
            }),
            BigInt::from_str(csv_record.get(1).unwrap_or_else(|| {
                eprintln!(
                    "{} failed to read / extract numbers from .csv",
                    "problem with 2 csv argument"
                );
                process::exit(0);
            }))
            .unwrap_or_else(|err| {
                eprintln!(
                    "{} / failed to read / extract numbers from .csv",
                    err
                );
                process::exit(0);
            }),
            BigInt::from_str(csv_record.get(2).unwrap_or_else(|| {
                eprintln!(
                    "{} / failed to read / extract numbers from .csv",
                    "problem with 3 csv argument"
                );
                process::exit(0);
            }))
            .unwrap_or_else(|err| {
                eprintln!(
                    "{} / failed to read / extract numbers from .csv",
                    err
                );
                process::exit(0);
            }),
        ));
    }
    Ok(num_vec)
}

fn main() {
    let output_filename: &String = &String::from("cp_2_output.csv");
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    // let args = vec![
    //     String::from("target/debug/main"),
    //     String::from(
    //         "/home/ikripaka/Documents/learning-rust/discrete_logarithm_problem/cp_2_input.csv",
    //     ),
    // ];
    println!("{:?}", args);
    // let vec = [String::from("target/debug/factorization"), String::from("-v"), String::from("397357310")];
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("{} problem parsing arguments: {}", program, err);
        process::exit(0);
    });

    println!("{:?}", arguments);

    process_arguments(&arguments, output_filename).unwrap_or_else(|err| {
        eprintln!("{} problem with writing to csv file: {}", program, err);
        process::exit(0);
    });
}
