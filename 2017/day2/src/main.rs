extern crate structopt;
#[macro_use]
extern crate structopt_derive;
extern crate failure;

use structopt::StructOpt;
use failure::Error;
use std::fs::File;
use std::io::Read;
use std::cmp;

#[derive(StructOpt, Debug)]
#[structopt(name = "day2", about = "Advent of code 2017 day 2")]
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

type Matrix = Vec<Vec<u32>>;

fn parse_line(input: &str) -> Result<Vec<u32>, Error> {
    input
        .split_whitespace()
        .map(|s| s.parse::<u32>().map_err(|e| Error::from(e)))
        .collect()
}

fn parse_matrix(input: &str) -> Result<Matrix, Error> {
    input.split('\n').map(parse_line).collect()
}

fn get_min_max(row: &[u32]) -> u32 {
    let (min, max) = row.iter().skip(1).fold(
        (row[0], row[0]),
        |(min, max), &element| {
            (cmp::min(min, element), cmp::max(max, element))
        },
    );
    max - min
}

fn get_divisible(row: &[u32]) -> u32 {
    for (idx, element) in row.iter().enumerate() {
        for other in &row[idx + 1..] {
            if other % element == 0 {
                return other / element;
            }
            if element % other == 0 {
                return element / other;
            }
        }
    }
    0
}

fn get_checksum<T: Fn(&[u32]) -> u32>(matrix: &Matrix, row_function: T) -> u32 {
    matrix.iter().map(Vec::as_slice).map(row_function).sum()
}


fn main() {
    let opt = Opts::from_args();
    let input = read_input(&opt.input).unwrap_or_else(|e| panic!("Failed to read input: {}", e));

    let matrix =
        parse_matrix(input.trim()).unwrap_or_else(|e| panic!("Failed to parse input: {}", e));

    println!(
        "Checksum for part 1 is: {}",
        get_checksum(&matrix, get_min_max)
    );
    println!(
        "Checksum for part 2 is: {}",
        get_checksum(&matrix, get_divisible)
    );
}

#[test]
fn test_parse_line() {
    assert_eq!(vec![1, 2, 3], parse_line("1 2 3").unwrap());
}

#[test]
fn test_parse_matrix() {
    assert_eq!(
        vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]],
        parse_matrix("1 2 3\n4 5 6\n7 8 9").unwrap()
    );
}

#[test]
fn test_min_max() {
    let d = vec![1, 2, 3, 4, 5];
    assert_eq!(get_min_max(&d), 4);
}

#[test]
fn test_get_divisible() {
    let d = vec![5, 9, 2, 8];
    assert_eq!(get_divisible(&d), 4);
}
