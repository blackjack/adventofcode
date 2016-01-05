#![feature(iter_arith)]
extern crate regex;
extern crate permutohedron;

use std::fs::File;
use std::io::BufRead;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use permutohedron::*;


fn parse_line(s: &str) -> (&str, &str, u32) {
    let re = Regex::new("(\\w+) to (\\w+) = (\\d+)").unwrap();
    let captures = re.captures(s).unwrap();
    (captures.at(1).unwrap(),
     captures.at(2).unwrap(),
     captures.at(3).unwrap().parse::<u32>().unwrap())
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut distances: HashMap<(String, String), u32> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();

    let reader = std::io::BufReader::new(&file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (f, t, d) = parse_line(&line);

        distances.insert((f.to_string(), t.to_string()), d);
        distances.insert((t.to_string(), f.to_string()), d);
        cities.insert(f.to_string());
        cities.insert(t.to_string());
    }

    let mut cities: Vec<_> = cities.iter().collect();
    let mut perms = Heap::new(&mut cities);


    let mut min = 999999u32;
    let mut max = 0u32;
    while let Some(route) = perms.next_permutation() {
        let cost: u32 = route.windows(2).map(|t| distances[&(t[0].clone(), t[1].clone())]).sum();
        min = std::cmp::min(min,cost);
        max = std::cmp::max(max,cost);
    }
    println!("Min distance: {}, max distance: {}",min,max);
}
