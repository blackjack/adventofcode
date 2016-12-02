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
    let re =
        Regex::new(r#"(\w+) would (gain|lose) (\d+) happiness units by sitting next to (\w+)."#)
        .unwrap();
    let captures = re.captures(s).unwrap();
    (captures.at(1).unwrap(),
    captures.at(4).unwrap(),
    match captures.at(2).unwrap() {
        "lose" => -1i32,
        _ => 1,
    } * captures.at(3).unwrap().parse::<i32>().unwrap())
}

fn get_happiness(people: &HashSet<String>, happiness_map: &HashMap<(String,String),i32>) -> i32 {
    let mut people: Vec<_> = people.iter().collect();
    let mut perms = Heap::new(&mut people);
    let mut max = 0i32;

    while let Some(route) = perms.next_permutation() {
        let mut tot = route.clone();
        tot.push(route[0]);

        let happiness = tot.windows(2).map(|x|
                                           happiness_map[&(x[0].clone(),x[1].clone())]+
                                           happiness_map[&(x[1].clone(),x[0].clone())]
                                          ).sum();
        max = std::cmp::max(max, happiness);
    }
    return max;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let mut happiness_map: HashMap<(String, String), i32> = HashMap::new();
    let mut people: HashSet<String> = HashSet::new();

    let reader = std::io::BufReader::new(&file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (f, t, d) = parse_line(&line);

        happiness_map.insert((f.to_string(), t.to_string()), d);

        people.insert(f.to_string());
        people.insert(t.to_string());
    }

    let without = get_happiness(&people, &happiness_map);

    let me = "Me".to_string();
    {
        let ref mut ppl = people;
        for p in ppl.iter() {
            happiness_map.insert((me.clone(),p.clone()),0);
            happiness_map.insert((p.clone(),me.clone()),0);
        }
    }
    people.insert(me);
    let with= get_happiness(&people, &happiness_map);

    println!("Optimal happiness without you: {}, with you: {}", without,with);
}
