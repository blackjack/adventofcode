use std::fs::File;
use std::io::prelude::*;

fn escape<'a>(s: &[u8]) -> Vec<u8> {
    let mut result: Vec<u8> = Vec::new();
    result.reserve(s.len());

    let mut chars = s.iter().enumerate();

    loop {
        let (idx,c) = match chars.next() {
            Some((idx,ch)) => (idx,ch),
            _ => break
        };
        let c = *c as char;

        if c=='"' && (idx==0||idx==s.len()-1) { continue }
        if c=='\\' {
            match chars.next() {
                Some((_,ch)) if *ch as char != 'x' => { result.push(*ch); continue },
                Some((_,ch)) if *ch as char == 'x' => {
                    let h1 = *chars.next().unwrap().1 as char;
                    let h2 = *chars.next().unwrap().1 as char;
                    let xx = h1.to_digit(16).unwrap()*16 + h2.to_digit(16).unwrap();
                    result.push( xx as u8 );
                    continue;
                },
                _ => break,
            };
        }

        result.push(c as u8);
    }

    return result;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut codelen = 0;
    let mut strlen = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        codelen+=line.len();
        let esc = escape(line.as_bytes());

        strlen+=esc.len();
    }

    let s = "\"aaa\\\"aaa\"";
    let e = escape(s.as_bytes());
    println!("A: {}, B: {}, D: {}",s.len(),e.len(),s.len()-e.len());
    println!("Total code chars: {}, total escaped chars: {}, diff: {}",codelen,strlen,codelen-strlen);
}



#[test]
fn it_works() {
    let strings = vec!(
        "\"\"",
        "\"abc\"",
        "\"aaa\\\"aaa\"",
        "\"\\x27\""
    );

    let l = &strings[0];
    let esc = escape(l.as_bytes());
    assert_eq!(l.len(),2);
    assert_eq!(esc.len(),0);

    let l = &strings[1];
    let esc = escape(l.as_bytes());
    assert_eq!(l.len(),5);
    assert_eq!(esc.len(),3);

    let l = &strings[2];
    let esc = escape(l.as_bytes());
    assert_eq!(l.len(),10);
    assert_eq!(esc.len(),7);

    let l = &strings[3];
    let esc = escape(l.as_bytes());
    assert_eq!(l.len(),6);
    assert_eq!(esc.len(),1);
}
