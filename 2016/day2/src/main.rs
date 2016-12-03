use std::cmp::{min, max};
use std::io::Read;

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Direction {
        match c {
            'U' => Direction::Up,
            'R' => Direction::Right,
            'D' => Direction::Down,
            'L' => Direction::Left,
            _ => panic!("Invalid char: {}", c),
        }
    }
}

struct Dial {
    x: i8,
    y: i8,
    buttons: Vec<Vec<char>>,
}

fn clamp(val: i8, lb: i8, ub: i8) -> i8 {
    min(max(val, lb), ub)
}

impl Dial {
    fn part1() -> Dial {
        let v = |s: &str| s.chars().collect();
        Dial {
            x: 1,
            y: 1,
            buttons: vec![v("123"), v("456"), v("789")],
        }
    }

    fn part2() -> Dial {
        let v = |s: &str| s.chars().collect();
        Dial {
            x: 0,
            y: 2,
            buttons: vec![v("\0\01\0\0"), v("\0234\0"), v("56789"), v("\0ABC\0"), v("\0\0D\0\0")],
        }
    }

    fn mv(&mut self, d: &Direction) {
        let (dx, dy) = match d {
            &Direction::Up => (0, -1),
            &Direction::Right => (1, 0),
            &Direction::Down => (0, 1),
            &Direction::Left => (-1, 0),
        };

        let x = clamp(self.x + dx, 0, self.buttons.len() as i8 - 1);
        let y = clamp(self.y + dy, 0, self.buttons.len() as i8 - 1);

        if self.at(x, y) != '\0' {
            self.x = x;
            self.y = y;
        }
    }

    fn at(&self, x: i8, y: i8) -> char {
        return self.buttons[y as usize][x as usize];
    }

    fn get(&self) -> char {
        return self.at(self.x, self.y);
    }
}

fn run(dial: &mut Dial, s: &str) -> String {
    let mut code = String::new();

    for s in s.split('\n').map(|s| s.trim()).filter(|s| !s.is_empty()) {
        for c in s.chars() {
            dial.mv(&Direction::from_char(c));
        }
        code.push(dial.get());
    }
    code
}


fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let mut dial1 = Dial::part1();
    let code1 = run(&mut dial1, &s);

    let mut dial2 = Dial::part2();
    let code2 = run(&mut dial2, &s);

    println!("Code #1: {}, #2: {}", code1, code2);
}



#[test]
fn test1() {
    let mut dial = Dial::part1();
    let input = "ULL
        RRDDD
        LURDL
        UUUUD
        ";

    let expected = "1985";

    assert_eq!(expected, run(&mut dial, input));
}

#[test]
fn test2() {
    let mut dial = Dial::part2();
    let input = "ULL
        RRDDD
        LURDL
        UUUUD
        ";

    let expected = "5DB3";

    assert_eq!(expected, run(&mut dial, input));
}
