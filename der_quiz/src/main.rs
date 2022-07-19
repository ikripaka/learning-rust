use der_quiz::aes_cipher::decipher;
use der_quiz::parser::Arguments;
use std::error::Error;
use std::{env, process};

// cargo run -- /home/ikripaka/Documents/learning-rust/der_quiz/Program0.enc

fn main() {
    let args: Vec<String> = env::args().collect();
    let args = vec![String::from("program"), String::from("/home/ikripaka/Documents/learning-rust/der_quiz/Program0.enc")];
    let program = args[0].clone();

    println!("Parsing arguments..");
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        eprintln!("{} problem parsing arguments: {}", program, err);
        process::exit(0);
    });

    println!(
        "Filepath: {}\nFilename: {}\nSalt: {:?}\nInput arguments: {:?}",
        arguments.filepath, arguments.name, arguments.salt, arguments.params
    );

    println!("Deciphering file..");
    decipher(&arguments);
}
