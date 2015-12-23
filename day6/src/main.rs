extern crate regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;

fn turn_on(bulb: u8)->u8 {
    1
}

fn turn_off(bulb: u8)->u8 {
    0
}

fn toggle(bulb: u8)->u8 {
    if bulb==1 {0} else {1}
}

struct Action {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    f: fn(u8)->u8,
}


impl Action {
    pub fn parse_string(s: &str)->Action {
        let re = Regex::new("(turn on|turn off|toggle) (\\d+),(\\d+) through (\\d+),(\\d+)").unwrap();
        let captures = re.captures(s).unwrap();

        let action_str = captures.at(1).unwrap();
        let x1 = captures.at(2).unwrap().parse::<i32>().unwrap();
        let y1 = captures.at(3).unwrap().parse::<i32>().unwrap();
        let x2 = captures.at(4).unwrap().parse::<i32>().unwrap();
        let y2 = captures.at(5).unwrap().parse::<i32>().unwrap();

        let f: fn(u8)->u8 = match action_str {
            "turn on" => turn_on,
            "turn off" => turn_off,
            _ => toggle
        };

        return Action { x1: x1, y1: y1, x2: x2, y2: y2, f: f };
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut map = [[0u8; 1000]; 1000];

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() { continue; }

        let act = Action::parse_string(&line);

        for x in act.x1..act.x2+1 {
            for y in act.y1..act.y2+1 {
                map[x as usize][y as usize] = (act.f)(map[x as usize][y as usize]);
            }
        }
    }

    let mut count = 0;
    for x in map.iter() {
        for y in x.iter() {
            count+=if *y>0 {1} else {0};
        }
    }
    println!("Number of bulbs lit: {}",count);
}




