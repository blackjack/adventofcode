use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;


struct VisitMap {
    data: HashMap<i32, HashMap<i32, i32>>,
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
            count += ymap.len();
        }
        return count;
    }
}


fn move_santa(x: i32, y: i32, c: char) -> (i32, i32) {
    match c {
        '<' => (x - 1, y),
        '>' => (x + 1, y),
        '^' => (x, y + 1),
        'v' => (x, y - 1),
        _ => (x, y),
    }
}

fn main() {
    let file = File::open("input.txt").unwrap();

    let (mut x1, mut x2, mut x3, mut y1, mut y2, mut y3) = (0, 0, 0, 0, 0, 0);
    let mut visits_one = VisitMap::new();
    let mut visits_two = VisitMap::new();

    let mut buf = vec![];
    loop {
        buf.clear();
        let f = &file;
        let mut t = f.take(2);

        if t.read_to_end(&mut buf).is_err() || buf.len() < 2 {
            break;
        }

        let (x, y) = move_santa(x1, y1, buf[0] as char);
        visits_one.visit(x, y);
        x1 = x;
        y1 = y;
        let (x, y) = move_santa(x1, y1, buf[1] as char);
        visits_one.visit(x, y);
        x1 = x;
        y1 = y;

        let (x, y) = move_santa(x2, y2, buf[0] as char);
        visits_two.visit(x, y);
        x2 = x;
        y2 = y;
        let (x, y) = move_santa(x3, y3, buf[1] as char);
        visits_two.visit(x, y);
        x3 = x;
        y3 = y;
    }
    println!("Houses visited by santa alone: {}", visits_one.number());
    println!("Houses visited by santa and robo santa: {}",
             visits_two.number());
}
