use regex::Regex;
use std::fmt;

#[derive(Debug)]
pub struct TuringMachineState {
    first_tape: String,
    second_tape: String,
    is_q_placed: bool,
    get_num_before_q: Regex,
}

impl TuringMachineState {
    pub fn new(
        first_input_tape: &String,
        second_input_tape: &String,
    ) -> Result<TuringMachineState, &'static str> {
        let is_unary: Regex = Regex::new(r"^1+$").unwrap();

        if !(is_unary.is_match(first_input_tape) && is_unary.is_match(second_input_tape)) {
            return Err("incorrect input for unary number representation");
        }
        let swapped_numbers = swap(first_input_tape, second_input_tape);
        return Ok(TuringMachineState {
            first_tape: swapped_numbers.0.clone(),
            second_tape: swapped_numbers.1.clone(),
            is_q_placed: false,
            get_num_before_q: Regex::new(r"^1+").unwrap(),
        });
    }

    //performs only one
    pub fn run(&mut self) -> Result<bool, &'static str> {
        if !self.is_q_placed {
            self.second_tape.push('q');
            self.is_q_placed = true;
        } else if compare_numbers(
            &self.first_tape,
            &String::from(
                self.get_num_before_q
                    .captures(&self.second_tape)
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .as_str(),
            ),
        ) {
            let b = self
                .get_num_before_q
                .captures(&self.second_tape)
                .unwrap()
                .get(0)
                .unwrap()
                .as_str()
                .len();
            self.first_tape.drain(..b);
            self.second_tape.push('1');
        } else {
            return Ok(false);
        }

        Ok(true)
    }
}

// compares numbers len (according to unary numbers)
// true --- a.len >= b.len
// false --- a.len < b.len
fn compare_numbers(a: &String, b: &String) -> bool {
    if a.len() >= b.len() {
        return true;
    }
    return false;
}

//swapping numbers if (A is less than B) (A<B)
fn swap<'a>(a: &'a String, b: &'a String) -> (&'a String, &'a String) {
    if a.len() > b.len() {
        return (a, b);
    }
    (b, a)
}

impl fmt::Display for TuringMachineState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "1: #{}#\n2: #{}#\n", self.first_tape, self.second_tape)
    }
}
