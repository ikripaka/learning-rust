//keyspace
// A-Z + a-z + 0-9 + ' ' + ''' + '(' + ')' + '+' + ',' + '-' + '/' + ':' + '=' + '?' + '.'
// 26 + 26 + 10 + 12 = 74
// всього 256 бітів ключ
// 74 * 74 * 74 * 74 = 29_986_576
// ключ має таку форму
// ключ{4 байти}000000000000{12байтів} xor SALT --- (128 бітів)
// sha256( 128 бітів ) -> 256 бітів пароль
// 29_986_576 різних геш функцій sha256

use crate::parser;
use crate::parser::Arguments;
use openssl::symm::*;
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::Write;

const ALPHABET: [char; 74] = [
    ' ', '\'', '(', ')', '+', ',', '-', '.', '/', ':', '=', '?', 'a', 'b', 'c', 'd', 'e', 'f', 'g',
    'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R', 'S',
    'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
];
const IV: [u8; 16] = [0; 16];
const MZ: u16 = 0x4D5A;
const PE: u16 = 0x5045;
const THRESHOLD_FOR_SEARCHING_PE: usize = 150;
const SEPARATOR: &str = ".";

//deciphers and creates file with informative .exe file
pub fn decipher(arguments: &Arguments) {
    let split_name = arguments.name.split(SEPARATOR).collect::<Vec<&str>>();
    let mut possible_variants_counter = 0;

    for letter1 in ALPHABET.iter() {
        for letter2 in ALPHABET.iter() {
            for letter3 in ALPHABET.iter() {
                for letter4 in ALPHABET.iter() {
                    let mut key: Vec<u8> = vec![
                        letter1.to_string().as_bytes()[0],
                        letter2.to_string().as_bytes()[0],
                        letter3.to_string().as_bytes()[0],
                        letter4.to_string().as_bytes()[0],
                    ];

                    pad(&mut key, 16);
                    key = xor(&key, &arguments.salt);
                    let mut hasher = Sha256::new();
                    hasher.update(key);
                    let key: [u8; 32] = hasher.finalize().to_vec().try_into().unwrap();

                    let mut crypter =
                        Crypter::new(Cipher::aes_256_cbc(), Mode::Decrypt, &key, Some(&IV))
                            .unwrap();
                    crypter.pad(false);

                    let mut result: Vec<u8> = vec![
                        0;
                        arguments.ciphered_exe_vec.len()
                            + Cipher::aes_256_cbc().block_size()
                    ];
                    let count = crypter
                        .update(&*arguments.ciphered_exe_vec, &mut result)
                        .unwrap();
                    let rest = crypter.finalize(&mut result).unwrap();
                    result.truncate(count + rest);

                    if ((result[0] as u16) << (parser::BITS_IN_BYTE as u16)) | result[1] as u16
                        == MZ
                        && search_pe(&result)
                    {
                        let new_filename = format!(
                            "{}_{}.{}",
                            split_name.get(0).unwrap(),
                            possible_variants_counter,
                            split_name.get(1).unwrap()
                        );
                        println!(
                            "- Deciphered file {} with key: {:?} --- '{}{}{}{}'",
                            new_filename, key, *letter1, *letter2, *letter3, *letter4
                        );
                        let mut exe_file = File::create(new_filename).unwrap();
                        exe_file.write_all(&*result).unwrap();
                        possible_variants_counter += 1;
                    }
                }
            }
        }
    }
}

//searching for PE combination in file
fn search_pe(byte_vec: &Vec<u8>) -> bool {
    //beginning from 2 because first 2 symbols must be -- MZ
    for i in 2..THRESHOLD_FOR_SEARCHING_PE {
        if ((*byte_vec.get(i).unwrap() as u16) << (parser::BITS_IN_BYTE as u16))
            | *byte_vec.get(i + 1).unwrap() as u16
            == PE
        {
            return true;
        }
    }
    false
}

fn xor(a: &Vec<u8>, b: &Vec<u8>) -> Vec<u8> {
    let mut result = Vec::new();
    for (num1, num2) in a.iter().zip(b.iter()) {
        result.push(num1 ^ num2);
    }
    result
}

fn pad(num_vec: &mut Vec<u8>, vec_byte_size: usize) {
    if vec_byte_size > num_vec.len() {
        for _ in num_vec.len()..vec_byte_size {
            num_vec.push(0_u8);
        }
    }
}
