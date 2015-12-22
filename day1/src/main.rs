use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt").unwrap();

    let mut floor = 0;
    for byte in file.bytes() {
        match byte.unwrap() as char {
            '(' => floor+=1,
            ')' => floor-=1,
            _   => {}
        }
    }

    println!("Result floor is: {}",floor);
}
