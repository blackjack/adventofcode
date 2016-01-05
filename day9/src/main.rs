extern crate regex;

use std::fs::File;
use std::io::BufRead;
use regex::Regex;

struct Cities {
    data: Vec<String>,
}

impl Cities {
    fn new() -> Cities {
        Cities { data: Vec::new() }
    }
    fn get(&self, s: usize) -> &str {
        &self.data[s]
    }

    fn insert(&mut self, s: &str) -> usize {
        let s = s.to_string();
        match self.data.iter().position(|x|*x==s) {
            Some(x) => x,
            _ => { self.data.push(s); self.data.len()-1 }
        }
    }
}

struct Distances {
    data: Vec<Vec<u32>>,
}

impl Distances {
    fn new() -> Distances {
        Distances { data: Vec::new() }
    }

    fn get_mut(&mut self, x: usize, y: usize) -> &mut u32 {
        if self.data.len() < x + 1 {
            self.data.resize(x + 1, Vec::new());
        }

        let mut yvec = self.data.get_mut(x).unwrap();
        if yvec.len() < y + 1 {
            yvec.resize(y + 1, 999999u32);
        }
        return yvec.get_mut(y).unwrap();
    }
}

fn parse_line(s: &str) -> (&str, &str, u32) {
    let re = Regex::new("(\\w+) to (\\w+) = (\\d+)").unwrap();
    let captures = re.captures(s).unwrap();
    (captures.at(1).unwrap(),
     captures.at(2).unwrap(),
     captures.at(3).unwrap().parse::<u32>().unwrap())
}

fn main() {
    let mut file = File::open("input.txt").unwrap();
    let mut cities = Cities::new();
    let mut distances = Distances::new();

    let reader = std::io::BufReader::new(&file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (f, t, d) = parse_line(&line);

        let x = cities.insert(f);
        let y = cities.insert(t);
        *distances.get_mut(x, y) = d;
        *distances.get_mut(y, x) = d;
    }

    let vec2: Vec<&str> = cities.data.iter().map(|x|&x[..if x.len()>6 {6} else {x.len()}]).collect();
    println!("\t{}", vec2.join("\t"));
    for (y, yvec) in distances.data.iter().enumerate() {
        print!("{:.6}\t", cities.get(y as usize).to_string());
        for val in yvec.iter() {
            print!("{0: <6}\t", val);
        }
        print!("\n");
    }


}
