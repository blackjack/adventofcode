extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate failure;

use structopt::StructOpt;
use failure::Error;
use std::fs::File;
use std::io::Read;

#[derive(StructOpt, Debug)]
#[structopt(name = "day1", about = "Advent of code 2017 day 1")]
struct Opts {
    #[structopt(help = "Input file (stdin if empty)")]
    input: Option<String>,
}


fn read_input(path: &Option<String>) -> Result<String, Error> {
    let mut result = String::new();
    match path {
        &Some(ref f) => {
            File::open(f)?.read_to_string(&mut result)?;
        }
        &None => {
            let stdin = std::io::stdin();
            stdin.lock().read_to_string(&mut result)?;
        }
    }
    Ok(result)
}

fn count_digits(input: &str) -> u32 {
    let input = input.as_bytes();
    let mut count: u32 = input
        .windows(2)
        .map(|w| if w[0] == w[1] {
            (w[0] as char).to_digit(10).unwrap()
        } else {
            0
        })
        .sum();

    if input.len() > 2 && input.last() == input.first() {
        count += (*input.last().unwrap() as char).to_digit(10).unwrap();
    }

    count
}

fn main() {
    let opt = Opts::from_args();
    let input = read_input(&opt.input).unwrap_or_else(|e| panic!("Failed to read input: {}", e));

    println!("Result: {}", count_digits(input.trim()));
}


#[test]
fn test_count() {
    assert_eq!(count_digits("1122"), 3);
    assert_eq!(count_digits("1111"), 4);
    assert_eq!(count_digits("1234"), 0);
    assert_eq!(count_digits("91212129"), 9);
    assert_eq!(count_digits("32333"), 9);
}
