use std::fs::File;
use std::io::prelude::*;

fn escape(s: &String) -> String {
    let mut result: String = String::new();
    if s.is_empty() { return "".to_string(); }

    result.reserve(s.len());
    let mut chars = s.chars().enumerate();

    loop {
        let (idx,c) = match chars.next() {
            Some((idx,ch)) => (idx,ch),
            _ => break
        };

        if c=='"' && (idx==0||idx==s.len()-1) { continue }
        if c=='\\' {
            match chars.next() {
                Some((_,ch)) if ch!='x' => { result.push(ch); continue },
                Some((_,ch)) if ch=='x' => {
                    let (_,h1) = chars.next().unwrap();
                    let (_,h2) = chars.next().unwrap();
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

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);

    let mut out = File::create("output.txt").unwrap();

    let mut codelen = 0;
    let mut strlen = 0;
    for line in reader.lines() {
        let line = line.unwrap();

        codelen+=line.len();
        let esc = escape(&line);

        println!("Source: [{}], Escaped: [{}]",line,esc);
        strlen+=esc.len();
        break;
    }

    let s = "\"aaa\\\"aaa\"".to_string();
    let e = escape(&s);
    println!("A: {}, B: {}, D: {}",s.len(),e.len(),s.len()-e.len());
    println!("Total code chars: {}, total escaped chars: {}, diff: {}",codelen,strlen,codelen-strlen);
}



#[test]
fn it_works() {
    let strings: Vec<String> = vec!(
        "\"\"",
        "\"abc\"",
        "\"aaa\\\"aaa\"",
        "\"\\x27\""
    ).iter().map(|s| s.to_string()).collect();

    let l = &strings[0];
    let esc = escape(&l);
    println!("[{}] [{}]",l,esc);
    assert_eq!(l.len(),2);
    assert_eq!(esc.len(),0);

    let l = &strings[1];
    let esc = escape(&l);
    assert_eq!(l.len(),5);
    assert_eq!(esc.len(),3);

    let l = &strings[2];
    let esc = escape(&l);
    assert_eq!(l.len(),10);
    assert_eq!(esc.len(),7);

    let l = &strings[3];
    let esc = escape(&l);
    assert_eq!(l.len(),6);
    assert_eq!(esc.len(),1);
}
