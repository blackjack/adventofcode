use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


struct VisitMap {
    data : HashMap<i32, HashMap<i32, i32>>,
}


impl VisitMap { 
    pub fn new() -> VisitMap {
        VisitMap { data: HashMap::new() }
    }
    pub fn visit(&mut self, x: i32, y: i32) {
        let ymap = self.data.entry(x).or_insert(HashMap::new());
        let counter = ymap.entry(y).or_insert(0);
        *counter += 1;
    }

    pub fn number(&self) -> usize {
        let mut count = 0;
        for ymap in self.data.values() {
            count+=ymap.len();
        }
        return count;
    }
}



fn main() {
    let file = File::open("input.txt").unwrap();

    let mut x = 0;
    let mut y = 0;
    let mut visits = VisitMap::new();

    visits.visit(x,y);

    for byte in file.bytes() {
        let b = byte.unwrap() as char;
        match b {
            '<' => x-=1,
            '>' => x+=1,
            '^' => y+=1,
            'v' => y-=1,
            _   => continue,
        }
        visits.visit(x,y);
    }

    println!("Houses visited: {}",visits.number());
}
