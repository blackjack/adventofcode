use std::io::Read;

struct Triangle {
    a: u32,
    b: u32,
    c: u32,
}

fn parse_string(s: &str) -> (u32, u32, u32) {
    let mut iter = s.split_whitespace();
    let result = (iter.next().unwrap().parse::<u32>().unwrap(),
                  iter.next().unwrap().parse::<u32>().unwrap(),
                  iter.next().unwrap().parse::<u32>().unwrap());

    assert_eq!(None, iter.next());
    result
}

impl Triangle {
    fn from_string(s: &str) -> Triangle {
        let vals = parse_string(s);

        Triangle {
            a: vals.0,
            b: vals.1,
            c: vals.2,
        }
    }

    fn possible(&self) -> bool {
        (self.a + self.b > self.c) && (self.a + self.c > self.b) && (self.b + self.c > self.a)
    }
}

fn run1(s: &str) -> u32 {
    s.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| Triangle::from_string(s))
        .fold(0, |acc, t| acc + t.possible() as u32)
}

fn run2(s: &str) -> u32 {
    s.split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let l1 = parse_string(chunk[0]);
            let l2 = parse_string(chunk[1]);
            let l3 = parse_string(chunk[2]);
            (Triangle {
                 a: l1.0,
                 b: l2.0,
                 c: l3.0,
             },
             Triangle {
                 a: l1.1,
                 b: l2.1,
                 c: l3.1,
             },
             Triangle {
                 a: l1.2,
                 b: l2.2,
                 c: l3.2,
             })
        })
        .fold(0, |acc, ts| {
            acc + ts.0.possible() as u32 + ts.1.possible() as u32 + ts.2.possible() as u32
        })
}


fn main() {
    let mut file = std::fs::File::open("input.txt").unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();

    let count1 = run1(&s);
    let count2 = run2(&s);

    println!("Valid row triangles: {}, valid columnt triangles: {}",
             count1,
             count2);
}


#[test]
fn test1() {
    let s = "5 10 25";
    assert_eq!(false, Triangle::from_string(s).possible());
    assert_eq!(0, run1(s));
}

#[test]
fn test2() {
    let s = "1 1 2
             1 2 5
             1 3 6";
    assert_eq!(2, run2(s));
}
