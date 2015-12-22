use std::fs::File;
use std::io::prelude::*;

fn main() {
    let file = File::open("input.txt").unwrap();

    let mut floor = 0;
    let mut pos = 1;
    let mut found_pos = false;
    for byte in file.bytes() {
        match byte.unwrap() as char {
            '(' => floor+=1,
            ')' => floor-=1,
            _   => {}
        }
        if !found_pos && floor==-1 {
            println!("Position that causes basement entering: {}",pos);
            found_pos = true;
        }
        pos+=1;
    }

    println!("Result floor is: {}",floor);
}
