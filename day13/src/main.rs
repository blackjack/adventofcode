#![feature(iter_arith)]
extern crate regex;
extern crate permutohedron;

use std::fs::File;
use std::io::BufRead;
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use permutohedron::*;


fn parse_line(s: &str) -> (&str, &str, i32) {
    let re = Regex::new(r#"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)."#).unwrap();
    let captures = re.captures(s).unwrap();
    (captures.at(1).unwrap(),
     captures.at(4).unwrap(),
     match captures.at(2).unwrap() {
         "lose" => -1i32,
         _ => 1,
     } * captures.at(3).unwrap().parse::<i32>().unwrap())
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut distances: HashMap<(String, String), i32> = HashMap::new();
    let mut cities: HashSet<String> = HashSet::new();

    let reader = std::io::BufReader::new(&file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (f, t, d) = parse_line(&line);

        distances.insert((f.to_string(), t.to_string()), d);
        cities.insert(f.to_string());
        cities.insert(t.to_string());
    }

    let mut cities: Vec<_> = cities.iter().collect();
    let mut perms = Heap::new(&mut cities);


    let mut min = 999999i32;
    let mut max = 0i32;
    while let Some(route) = perms.next_permutation() {
        let cost: i32 = route.windows(2).map(|t| distances[&(t[0].clone(), t[1].clone())]).sum();
        min = std::cmp::min(min,cost);
        max = std::cmp::max(max,cost);
    }

    println!("Distances: {:?}",distances);

    println!("Min distance: {}, max distance: {}",min,max);
}
