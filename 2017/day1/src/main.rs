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

fn get_other_index_part1(i: usize, len: usize) -> usize {
    (i + 1) % len
}

fn get_other_index_part2(i: usize, len: usize) -> usize {
    (i + len / 2) % len
}

fn count_digits<T: Fn(usize, usize) -> usize>(input: &str, get_other_index: T) -> u32 {
    let input = input.as_bytes();
    input
        .iter()
        .enumerate()
        .map(|(idx, &val)| if val ==
            input[get_other_index(idx, input.len())]
        {
            (val as char).to_digit(10).unwrap()
        } else {
            0
        })
        .sum()
}

fn main() {
    let opt = Opts::from_args();
    let input = read_input(&opt.input).unwrap_or_else(|e| panic!("Failed to read input: {}", e));

    println!(
        "Result of part 1: {}",
        count_digits(input.trim(), &get_other_index_part1)
    );
    println!(
        "Result of part 2: {}",
        count_digits(input.trim(), &get_other_index_part2)
    );
}

#[test]
fn test_get_other_index_part1() {
    assert_eq!(get_other_index_part1(0, 3), 1);
    assert_eq!(get_other_index_part1(2, 3), 0);
}

#[test]
fn test_get_other_index_part2() {
    assert_eq!(get_other_index_part2(0, 4), 2);
    assert_eq!(get_other_index_part2(3, 4), 1);
    assert_eq!(get_other_index_part2(4, 6), 1);
}

#[test]
fn test_count_part1() {
    let idx = get_other_index_part1;
    assert_eq!(count_digits("1122", &idx), 3);
    assert_eq!(count_digits("1111", &idx), 4);
    assert_eq!(count_digits("1234", &idx), 0);
    assert_eq!(count_digits("91212129", &idx), 9);
    assert_eq!(count_digits("32333", &idx), 9);
}
