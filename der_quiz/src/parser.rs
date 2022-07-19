use std::fs::File;
use std::io::Read;
use std::u128::MAX;

#[derive(Debug, Clone)]
pub struct Arguments {
    pub name: String,
    pub salt: Vec<u8>,
    pub password: String,
    pub params: Vec<String>,
    pub filepath: String,
    pub ciphered_exe_vec: Vec<u8>,
}

// 0 1 2 3 4 5 6 7 8 9 A B C D E F
const SEQUENCE_TAG: u8 = 0x30;
const PRINTABLE_STRING_TAG: u8 = 0x13;
const OCTET_STRING_TAG: u8 = 0x04;

const BYTE_SIZE: u128 = 8;
const FIRST_BIT_MASK: u8 = 128;
const SHIFT_BY_SEVEN: u8 = 7;
pub const BITS_IN_BYTE: u8 = 8;

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("not enough arguments");
        } else if args.len() > 2 {
            return Err("to many arguments");
        }
        let arg1 = &args[1].clone();

        if arg1.contains(".enc") && std::path::Path::new(&arg1).exists() {
            let mut file = File::open(arg1.clone()).unwrap();
            let mut buffer = [0_u8; 10];

            //reading sequence 30 in hexdump
            let mut buff_len = file.read(&mut buffer[..]).unwrap();
            let mut i: usize = 0;
            let mut byte_counter: u128 = MAX;

            if buffer[0] == SEQUENCE_TAG {
                (i, buff_len, byte_counter) =
                    read_length(&mut buffer, &mut file, 1, &mut byte_counter);

                let mut printable_str_size = 0;
                if buffer[i] == PRINTABLE_STRING_TAG {
                    //get next byte after tag
                    (i, buff_len) = get_next_byte(&mut buffer, &mut file, i, &mut byte_counter);

                    (i, buff_len, printable_str_size) =
                        read_length(&mut buffer, &mut file, i, &mut byte_counter);

                    let mut name = String::new();
                    (i, buff_len, name) = read_printable_string(
                        &mut buffer,
                        &mut file,
                        i,
                        printable_str_size,
                        &mut byte_counter,
                    );

                    if buffer[i] == OCTET_STRING_TAG {
                        (i, buff_len) = get_next_byte(&mut buffer, &mut file, i, &mut byte_counter);

                        let mut salt_length: u128 = 0;
                        (i, buff_len, salt_length) =
                            read_length(&mut buffer, &mut file, i, &mut byte_counter);

                        let mut salt: Vec<u8> = Vec::new();
                        (i, buff_len, salt) = read_octet_string(
                            &mut buffer,
                            &mut file,
                            i,
                            salt_length,
                            &mut byte_counter,
                        );

                        let mut password: String = String::new();
                        if buffer[i] == PRINTABLE_STRING_TAG {
                            (i, buff_len) =
                                get_next_byte(&mut buffer, &mut file, i, &mut byte_counter);

                            (i, buff_len, printable_str_size) =
                                read_length(&mut buffer, &mut file, i, &mut byte_counter);

                            (i, buff_len, password) = read_printable_string(
                                &mut buffer,
                                &mut file,
                                i,
                                printable_str_size,
                                &mut byte_counter,
                            );
                        }

                        if buffer[i] == SEQUENCE_TAG {
                            (i, buff_len) =
                                get_next_byte(&mut buffer, &mut file, i, &mut byte_counter);

                            let mut sequence_len = 0;
                            let mut params = Vec::new();
                            (i, buff_len, sequence_len) =
                                read_length(&mut buffer, &mut file, i, &mut byte_counter);

                            while sequence_len != 0 {
                                if buffer[i] == PRINTABLE_STRING_TAG {
                                    (i, buff_len) =
                                        get_next_byte(&mut buffer, &mut file, i, &mut byte_counter);

                                    sequence_len -= 1;

                                    let mut printable_str_size = 0;
                                    let byte_counter_before = byte_counter;

                                    (i, buff_len, printable_str_size) =
                                        read_length(&mut buffer, &mut file, i, &mut byte_counter);

                                    sequence_len -=
                                        printable_str_size + byte_counter_before - byte_counter;

                                    let mut data = String::new();
                                    (i, buff_len, data) = read_printable_string(
                                        &mut buffer,
                                        &mut file,
                                        i,
                                        printable_str_size,
                                        &mut byte_counter,
                                    );
                                    params.push(data.clone());
                                }
                            }

                            if buffer[i] == OCTET_STRING_TAG {
                                (i, buff_len) =
                                    get_next_byte(&mut buffer, &mut file, i, &mut byte_counter);

                                let mut file_octet_length = 0;
                                (i, buff_len, file_octet_length) =
                                    read_length(&mut buffer, &mut file, i, &mut byte_counter);

                                let mut ciphered_exe: Vec<u8> = Vec::new();
                                (i, buff_len, ciphered_exe) = read_octet_string(
                                    &mut buffer,
                                    &mut file,
                                    i,
                                    file_octet_length,
                                    &mut byte_counter,
                                );

                                return Ok(Arguments {
                                    name: name.clone(),
                                    salt: salt.clone(),
                                    password: password.clone(),
                                    params,
                                    filepath: arg1.clone(),
                                    ciphered_exe_vec: ciphered_exe.clone(),
                                });
                            }
                        }
                    }
                }
            }
            return Err("failed to parse sequence");
        }
        return Err("invalid filename or file path");
    }
}

// increasing counter of passed bytes (i)
fn get_next_byte(
    buffer: &mut [u8; 10],
    file: &mut File,
    i: usize,
    byte_counter: &mut u128,
) -> (usize, usize) {
    let mut i = i;
    let mut buff_len = buffer.len();
    if i == buff_len - 1 {
        i = 0;
        buff_len = file.read(buffer).unwrap();
    } else {
        i += 1;
    }
    *byte_counter -= 1;

    (i, buff_len)
}

fn read_octet_string(
    buffer: &mut [u8; 10],
    file: &mut File,
    i: usize,
    octet_str_size: u128,
    byte_counter: &mut u128,
) -> (usize, usize, Vec<u8>) {
    let mut buff_len = buffer.len();
    let mut i = i;
    let mut result = Vec::new();
    let mut is_finished = false;
    let mut octet_str_size = octet_str_size;

    while !is_finished {
        for j in i..buff_len {
            octet_str_size -= 1;
            *byte_counter -= 1;
            result.push(buffer[j]);

            if octet_str_size == 0 {
                is_finished = true;

                if j == buff_len - 1 {
                    i = 0;
                    buff_len = file.read(buffer).unwrap();
                } else {
                    i = j + 1;
                }
                break;
            }

            if j == buff_len - 1 {
                i = 0;
                buff_len = file.read(buffer).unwrap();
            }
        }
    }

    (i, buff_len, result)
}

// returning next position according to where reading stopped
fn read_printable_string(
    buffer: &mut [u8; 10],
    file: &mut File,
    i: usize,
    string_size: u128,
    byte_counter: &mut u128,
) -> (usize, usize, String) {
    let mut buff_len = buffer.len();
    let mut i = i;
    let mut result = String::new();
    let mut is_finished = false;
    let mut string_size = string_size;

    while !is_finished {
        for j in i..buff_len {
            string_size -= 1;
            *byte_counter -= 1;
            result.push_str(std::str::from_utf8(&[buffer[j]]).unwrap());

            if string_size == 0 {
                is_finished = true;

                if j == buff_len - 1 {
                    i = 0;
                    buff_len = file.read(buffer).unwrap();
                } else {
                    i = j + 1;
                }
                break;
            }

            if j == buff_len - 1 {
                i = 0;
                buff_len = file.read(buffer).unwrap();
            }
        }
    }

    (i, buff_len, result)
}

// returning next position according to where reading stopped
fn read_length(
    buffer: &mut [u8; 10],
    file: &mut File,
    i: usize,
    byte_counter: &mut u128,
) -> (usize, usize, u128) {
    let mut buff_len = buffer.len();
    let mut i = i;
    let mut result: u128 = 0;
    let mut is_finished = false;

    if (buffer[i] & FIRST_BIT_MASK) >> SHIFT_BY_SEVEN == 0 {
        *byte_counter -= 1;
        result = buffer[i] as u128;

        if i + 1 == buff_len - 1 {
            i = 0;
            buff_len = file.read(buffer).unwrap();
        } else {
            i += 1;
        }
        return (i, buff_len, result);
    } else {
        let mut bits_to_read: u8 = buffer[i] ^ FIRST_BIT_MASK;
        *byte_counter -= 1;

        i += 1;
        if i == buff_len - 1 {
            i = 0;
            buff_len = file.read(buffer).unwrap();
        }

        while !is_finished {
            for j in i..buff_len {
                result = (buffer[j] as u128) | (result << BITS_IN_BYTE);
                *byte_counter -= 1;
                bits_to_read -= 1;

                if bits_to_read == 0 {
                    is_finished = true;

                    if j == buff_len - 1 {
                        i = 0;
                        buff_len = file.read(buffer).unwrap();
                    } else {
                        i = j + 1;
                    }
                    break;
                }

                if j == buff_len - 1 {
                    i = 0;
                    buff_len = file.read(buffer).unwrap();
                }
            }
        }
    }
    (i, buff_len, result)
}
