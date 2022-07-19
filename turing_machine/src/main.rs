use std::error::Error;
use std::{env, process};

// cargo run -- first_num second_num
// cargo run -- 111111111 111

//interpret last output as
//1: ##
//2: #11q11# => 11*11 = 1111 => 2*2 = 4

//struct to define arguments that passed through cli
#[derive(Debug)]
struct Arguments {
    a: String,
    b: String,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        } else if args.len() > 3 {
            return Err("to many arguments");
        }
        return Ok(Arguments {
            a: args[1].clone(),
            b: args[2].clone(),
        });
    }
}

//processing arguments and runs turing machine sequentially
fn process_arguments(args: &Arguments) -> Result<(), Box<dyn Error>> {
    let mut tm = turing_machine::turing_machine::TuringMachineState::new(&args.a, &args.b)
        .unwrap_or_else(|err| {
            eprintln!("{} problem with calculating", err);
            process::exit(0);
        });

    loop {
        let run = tm.run();
        match run {
            Ok(is_finished) => {
                if is_finished {
                    println!("{}", tm);
                } else {
                    println!("TM finished work, final tapes:");
                    println!("{}", tm);
                    break;
                }
            }
            Err(err) => {
                eprintln!("err in next step: {}", err);
                process::exit(0);
            }
        }
    }

    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("{} problem parsing arguments: {}", program, err);
        process::exit(0);
    });

    process_arguments(&arguments).unwrap_or_else(|err| {
        eprintln!("{} problem with executing TM: {}", program, err);
        process::exit(0);
    });
}
