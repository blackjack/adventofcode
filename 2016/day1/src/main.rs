use std::io::Read;
use std::collections::HashSet;
use std::cmp::Ordering;

#[derive(Copy, Clone)]
enum Turn {
    Left = -1,
    Right = 1,
}

impl Turn {
    fn from_char(c: char) -> Turn {
        return match c {
            'L' => Turn::Left,
            'R' => Turn::Right,
            _ => panic!(format!("Invalid turn direction: {}", c)),
        };
    }
}
#[derive(Debug, Clone)]
#[allow(dead_code)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn turn(&self, t: Turn) -> Direction {
        let v = 4 + self.clone() as i8 + t as i8;
        unsafe { std::mem::transmute(v % 4) }
    }
}

#[derive(Eq, PartialEq, Clone, Debug, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn get_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn step(&self, other: &Point) -> (i32, i32) {
        let get = |from: i32, to: i32| match from.cmp(&to) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        };
        (get(self.x, other.x), get(self.y, other.y))
    }
}

#[derive(Debug)]
struct Motion {
    from: Point,
    to: Point,
    stepx: i32,
    stepy: i32,
}

impl Motion {
    fn new(from: Point, to: Point) -> Motion {
        let (stepx, stepy) = from.step(&to);
        Motion {
            from: from,
            to: to,
            stepx: stepx,
            stepy: stepy,
        }
    }
}

impl Iterator for Motion {
    type Item = Point;

    fn next(&mut self) -> Option<Point> {
        if self.from == self.to {
            return None;
        }
        let result = self.from.clone();
        self.from.x += self.stepx;
        self.from.y += self.stepy;
        Some(result)
    }
}

#[derive(Debug)]
struct Position {
    coordinates: Point,
    sight: Direction,
}

impl Position {
    fn new() -> Position {
        Position {
            coordinates: Point { x: 0, y: 0 },
            sight: Direction::North,
        }
    }

    fn go(&mut self, turn: Turn, distance: i32) -> Motion {
        self.sight = self.sight.turn(turn);

        let from = self.coordinates.clone();
        let mut to = self.coordinates.clone();
        match self.sight {
            Direction::North => to.y += distance,
            Direction::East => to.x += distance,
            Direction::South => to.y -= distance,
            Direction::West => to.x -= distance,
        };

        self.coordinates = to.clone();
        Motion::new(from, to)
    }
}

fn run(s: &str) -> (i32, i32) {
    let mut pos = Position::new();
    let mut visited: HashSet<Point> = HashSet::new();
    let mut bunny: i32 = 0;

    for s in s.split(", ") {
        let mut chars = s.chars();
        let turn = Turn::from_char(chars.next().unwrap());
        let distance = chars.as_str().trim().parse::<i32>().unwrap();

        for point in pos.go(turn, distance) {
            if bunny == 0 {
                if !visited.contains(&point) {
                    visited.insert(point.clone());
                } else {
                    bunny = point.get_distance();
                }
            }
        }
    }

    (pos.coordinates.get_distance(), bunny)
}


fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    let (distance, bunny) = run(&s);
    println!("Distance: {}, Bunny: {}", distance, bunny);
}


#[test]
fn check1() {
    let input = "R2, L3";
    let expect = 5;
    assert_eq!(expect, run(input).0);
}

#[test]
fn check2() {
    let input = "R2, R2, R2";
    let expect = 2;
    assert_eq!(expect, run(input).0);
}

#[test]
fn check3() {
    let input = "R5, L5, R5, R3";
    let expect = 12;
    assert_eq!(expect, run(input).0);
}

#[test]
fn bunny() {
    let input = "R8, R4, R4, R8";
    let expect = 4;
    assert_eq!(expect, run(input).1);
}
