use factorization::factorize;
use factorization::factorize::factorize_all_algorithms;
use std::error::Error;
use std::fs::File;
use std::{env, process};

// cargo run -- -v 25511
// cargo run -- 25511
// cargo run -- file.csv
#[derive(Debug)]
struct Arguments {
    verbose: bool,
    number: u128,
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

        if arg1.contains("-v") || arg1.contains("-V") && args.len() == 3 {
            // cargo run -- -v 25511
            if let Ok(number) = u128::from_str_radix(&args[2], 10) {
                return Ok(Arguments {
                    verbose: true,
                    number,
                    file: false,
                    file_path: String::new(),
                });
            } else {
                return Err("troubles with converting number");
            }
        } else if let Ok(number) = u128::from_str_radix(&args[1], 10) {
            // cargo run -- 25511
            return Ok(Arguments {
                verbose: false,
                number,
                file: false,
                file_path: String::new(),
            });
        } else if arg1.contains(".csv") && std::path::Path::new(&arg1).exists() {
            // cargo run -- file.csv
            return Ok(Arguments {
                verbose: false,
                number: 0_u128,
                file: true,
                file_path: arg1,
            });
        }
        return Err("invalid syntax");
    }
}

fn process_arguments(args: &Arguments, output_filename: &String) -> Result<(), Box<dyn Error>> {
    if args.file == false {
        if args.verbose == true {
            println!("{:?}", factorize_all_algorithms(&args.number, true));
        } else {
            println!("{:?}", factorize_all_algorithms(&args.number, false));
        }
        return Ok(());
    } else {
        let num_vec = read_csv(&args.file_path).unwrap_or_else(|err| {
            eprintln!("{} problem parsing arguments from .csv file", err);
            process::exit(0);
        });
        let mut file = File::create(output_filename).expect("Error encountered while creating file!");
        let mut wtr = csv::Writer::from_writer(file);
        for num in num_vec.iter() {
            let string_vec: Vec<String> = factorize::factorize_all_algorithms(num, false)
                .iter()
                .map(|x: &u128| x.to_string())
                .collect();
            wtr.write_record(&string_vec);
        }
        wtr.flush()?;
        Ok(())
    }
}

fn read_csv(file_path: &String) -> Result<Vec<u128>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(file_path);
    let mut num_vec = Vec::new();
    for result in rdr.unwrap().records() {
        let record = result?;
        num_vec.push(
            u128::from_str_radix(record.as_slice(), 10).unwrap_or_else(|err| {
                eprintln!("{} failed to read/extract numbers from .csv", err);
                process::exit(0);
            }),
        );
    }
    Ok(num_vec)
}

fn main() {
    let output_filename: &String = &String::from("cp_1_output.csv");
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let vec = [String::from("target/debug/factorization"), String::from("/home/ikripaka/Documents/learning-rust/factorization/src/cp_1_input.csv")];
    let arguments = Arguments::new(&vec).unwrap_or_else(|err| {
        eprintln!("{} problem parsing arguments: {}", program, err);
        process::exit(0);
    });

    process_arguments(&arguments, output_filename).unwrap_or_else(|err| {
        eprintln!("{} problem with writing to csv file: {}", program, err);
        process::exit(0);
    });
}
