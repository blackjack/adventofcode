use std::fs::File;
use std::io::prelude::*;

fn decode(s: &[u8]) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
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
                Some((_,ch)) if *ch as char != 'x' => { result.push(c); continue },
                Some((_,ch)) if *ch as char == 'x' => {
                    let h1 = *chars.next().unwrap().1 as char;
                    let h2 = *chars.next().unwrap().1 as char;
                    let xx = h1.to_digit(16).unwrap()*16 + h2.to_digit(16).unwrap();
                    result.push( xx as u8 as char );
                    continue;
                },
                _ => break,
            };
        }

        result.push(c);
    }

    return result;
}

fn encode(s: &[u8]) -> Vec<char> {
    let mut result: Vec<char> = Vec::new();
    result.reserve(s.len());

    result.push('"');
    for c in s.iter() {
        let ch = *c as char;
        if ch=='"' || ch=='\\' {
            result.push('\\');
        }
        result.push(ch);
    }
    result.push('"');

    return result;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut codelen = 0;
    let mut declen = 0;
    let mut enclen = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        let d = decode(line.as_bytes());
        let e = encode(line.as_bytes());

        codelen += line.len();
        declen += d.len();
        enclen += e.len();
    }

    println!("Total code chars: {}, total decoded chars: {}, total encoded chars: {}, diff(dec): {}, diff(enc): {}",
             codelen,declen,enclen,codelen-declen,enclen-codelen);
}



#[test]
fn test_decode() {
    let strings = vec!(
        "\"\"",
        "\"abc\"",
        "\"aaa\\\"aaa\"",
        "\"\\x27\""
    );

    let l = &strings[0];
    let esc = decode(l.as_bytes());
    assert_eq!(esc.len(),0);

    let l = &strings[1];
    let esc = decode(l.as_bytes());
    assert_eq!(esc.len(),3);

    let l = &strings[2];
    let esc = decode(l.as_bytes());
    assert_eq!(esc.len(),7);

    let l = &strings[3];
    let esc = decode(l.as_bytes());
    assert_eq!(esc.len(),1);
}

#[test]
fn test_encode() {
    let strings = vec!(
        "\"\"",
        "\"abc\"",
        "\"aaa\\\"aaa\"",
        "\"\\x27\""
    );

    let l = &strings[0];
    let esc = encode(l.as_bytes());
    assert_eq!(esc.len(),6);

    let l = &strings[1];
    let esc = encode(l.as_bytes());
    assert_eq!(esc.len(),9);

    let l = &strings[2];
    let esc = encode(l.as_bytes());
    assert_eq!(esc.len(),16);

    let l = &strings[3];
    let esc = encode(l.as_bytes());
    assert_eq!(esc.len(),11);
}
