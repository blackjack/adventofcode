extern crate regex;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::collections::HashMap;
use regex::Regex;


enum Argument {
    Variable(String),
    Value(i32),
    None,
}

struct Operation {
    x: Argument,
    y: Argument,
    f: fn(i32, i32) -> i32,
    dest: String,
}


fn and(x: i32, y: i32) -> i32 {
    x & y
}
fn or(x: i32, y: i32) -> i32 {
    x | y
}
#[allow(unused_variables)]
fn not(x: i32, y: i32) -> i32 {
    !y
}
#[allow(unused_variables)]
fn noop(x: i32, y: i32) -> i32 {
    x
}
fn lshift(x: i32, y: i32) -> i32 {
    x << y
}
fn rshift(x: i32, y: i32) -> i32 {
    x >> y
}


impl Argument {
    fn parse(s: &str) -> Argument {
        if s.is_empty() {
            return Argument::None;
        }

        match s.parse::<i32>() {
            Ok(n) => return Argument::Value(n),
            _ => return Argument::Variable(s.to_string()),
        }
    }
}

impl Operation {
    fn parse(line: &str) -> (String, Operation) {
        let re = Regex::new("(\\d+|[a-z]+)? ?(AND|NOT|OR|LSHIFT|RSHIFT)? ?(\\d+|[a-z]+)? -> \
                             ([a-z]+)")
                     .unwrap();
        let captures = re.captures(line).unwrap();

        let x = Argument::parse(captures.at(1).unwrap_or(""));
        let f: fn(i32, i32) -> i32 = match captures.at(2).unwrap_or("") {
            "AND" => and,
            "OR" => or,
            "NOT" => not,
            "LSHIFT" => lshift,
            "RSHIFT" => rshift,
            _ => noop,
        };
        let y = Argument::parse(captures.at(3).unwrap_or(""));

        let dest = captures.at(4).unwrap();
        return (dest.to_string(),
                Operation {
            x: x,
            y: y,
            f: f,
            dest: dest.to_string(),
        });
    }
    fn execute(&self,
               registers: &HashMap<String, Operation>,
               cache: &mut HashMap<String, i32>)
               -> i32 {
        {
            let cached = cache.get(&self.dest);
            if cached.is_some() {
                return *cached.unwrap();
            }
        }

        let xval = match self.x {
            Argument::Variable(ref v) => registers.get(v).unwrap().execute(&registers, cache),
            Argument::Value(n) => n,
            _ => 0i32,
        };

        let yval = match self.y {
            Argument::Variable(ref v) => registers.get(v).unwrap().execute(&registers, cache),
            Argument::Value(n) => n,
            _ => 0i32,
        };

        let result = (self.f)(xval, yval);
        cache.insert(self.dest.clone(), result);
        return result;
    }
}


fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut registers: HashMap<String, Operation> = HashMap::new();
    let mut cache: HashMap<String, i32> = HashMap::new();


    for line in reader.lines() {
        let line = line.unwrap();

        let (dest, op) = Operation::parse(&line);

        registers.insert(dest, op);
    }

    let a = registers.get("a").unwrap().execute(&registers, &mut cache);
    cache.clear();
    *registers.get_mut("b").unwrap() = Operation {
        x: Argument::Value(a),
        y: Argument::None,
        f: noop,
        dest: "b".to_string(),
    };
    let a2 = registers.get("a").unwrap().execute(&registers, &mut cache);

    println!("Value of a: {}, after permutations: {}", a, a2);
}
