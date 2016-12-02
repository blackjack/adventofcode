use std::io::Read;

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

#[derive(Debug)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

impl Direction {
    fn turn(self, t: Turn) -> Direction {
        let v = 4 + self as i32 + t as i32;
        match v % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            _ => Direction::West,
        }
    }
}

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32,
    sight: Direction,
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            sight: Direction::North,
        }
    }

    fn get_distance(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    fn go(self, turn: Turn, distance: i32) -> Position {
        let new_sight = self.sight.turn(turn);

        let (newx, newy) = match new_sight {
            Direction::North => (self.x, self.y + distance),
            Direction::East => (self.x + distance, self.y),
            Direction::South => (self.x, self.y - distance),
            Direction::West => (self.x - distance, self.y),
        };

        Position {
            x: newx,
            y: newy,
            sight: new_sight,
        }
    }
}

fn run(s: &str) -> i32 {
    let mut pos = Position::new();

    for s in s.split(", ") {
        let mut chars = s.chars();
        let turn = Turn::from_char(chars.next().unwrap());
        let distance = chars.as_str().trim().parse::<i32>().unwrap();

        pos = pos.go(turn, distance);
    }

    pos.get_distance()
}


fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    println!("Distance: {}", run(&s));
}


#[test]
fn check1() {
    let input = "R2, L3";
    let expect = 5;
    assert_eq!(expect, run(input));
}

#[test]
fn check2() {
    let input = "R2, R2, R2";
    let expect = 2;
    assert_eq!(expect, run(input));
}

#[test]
fn check3() {
    let input = "R5, L5, R5, R3";
    let expect = 12;
    assert_eq!(expect, run(input));
}
