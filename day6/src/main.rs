extern crate regex;

use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use regex::Regex;

struct Action {
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    f1: fn(i32)->i32,
    f2: fn(i32)->i32,
}

#[allow(unused_variables)]
fn turn_on1(b: i32)->i32 { 1 }
#[allow(unused_variables)]
fn turn_off1(b: i32)->i32 { 0 }
fn toggle1(b: i32)->i32 { if b>0 {0} else {1} }

fn turn_on2(b: i32)->i32 { b+1 }
fn turn_off2(b: i32)->i32 { if b>1 {b-1} else {0} }
fn toggle2(b: i32)->i32 { b+2 }

impl Action {
    pub fn parse_string(s: &str)->Action {
        let re = Regex::new("(turn on|turn off|toggle) (\\d+),(\\d+) through (\\d+),(\\d+)").unwrap();
        let captures = re.captures(s).unwrap();

        let action_str = captures.at(1).unwrap();
        let x1 = captures.at(2).unwrap().parse::<i32>().unwrap();
        let y1 = captures.at(3).unwrap().parse::<i32>().unwrap();
        let x2 = captures.at(4).unwrap().parse::<i32>().unwrap();
        let y2 = captures.at(5).unwrap().parse::<i32>().unwrap();

        let f1: fn(i32)->i32 = match action_str {
            "turn on" => turn_on1,
            "turn off" => turn_off1,
            _ => toggle1,
        };

        let f2: fn(i32)->i32 = match action_str {
            "turn on" => turn_on2,
            "turn off" => turn_off2,
            _ => toggle2,
        };

        return Action { x1: x1, y1: y1, x2: x2, y2: y2, f1: f1, f2: f2 };
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);
    let mut map1 = [[0i32; 1000]; 1000];
    let mut map2 = [[0i32; 1000]; 1000];

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() { continue; }

        let act = Action::parse_string(&line);

        for x in act.x1..act.x2+1 {
            for y in act.y1..act.y2+1 {
                let x = x as usize;
                let y = y as usize;
                map1[x][y] = (act.f1)(map1[x][y]);
                map2[x][y] = (act.f2)(map2[x][y]);
            }
        }
    }

    let mut count1 = 0;
    let mut count2 = 0;
    for x in map1.iter() {
        for y in x.iter() {
            count1+=if *y>0 {1} else {0};
        }
    }
    for x in map2.iter() {
        for y in x.iter() {
            count2+=*y;
        }
    }
    println!("Number of bulbs lit: {}, total brightness: {}",count1,count2);
}




