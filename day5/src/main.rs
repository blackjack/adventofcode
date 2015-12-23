use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;

fn is_vowel(c: char) -> bool {
    "aeiou".find(c).is_some()
}

fn good_ver_1(chars: &[u8]) -> bool {
    let mut num_vowels = 0;
    let mut forbidden = false;
    let mut double = false;

    for i in 0..chars.len() {
        let c1 = chars[i] as char;

        if is_vowel(c1) {
            num_vowels+=1;
        }

        if i==chars.len()-1 { break }
        let c2 = chars[i+1] as char;

        if c1==c2 {
            double=true;
        }

        if (c1=='a' && c2=='b') || (c1=='c' && c2=='d') || (c1=='p' && c2=='q') || (c1=='x' && c2=='y') {
            forbidden=true;
        }
    }

    return !forbidden && num_vowels>=3 && double;
}


fn good_ver_2(chars: &[u8]) -> bool {
    let mut repeat = false;
    let mut has_pair = false;
    for i in 0..chars.len()-2 {
        if repeat && has_pair { break }

        let c1 = chars[i] as char;
        let c2 = chars[i+1] as char;
        let c3 = chars[i+2] as char;

        if c1==c3 {
            repeat = true;
        }

        let rest = std::str::from_utf8(&chars[i+2..chars.len()]).unwrap();
        let pair = c1.to_string() + &c2.to_string();
        if rest.find(&pair).is_some() {
            has_pair=true;
        }
    }
    return repeat && has_pair;

}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut num_good_v1 = 0;
    let mut num_good_v2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let chars = line.as_bytes();
        num_good_v1+=if good_ver_1(chars) {1} else {0};
        num_good_v2+=if good_ver_2(chars) {1} else {0};
    }

    println!("Number of good strings for v1: {}, for v2: {}",num_good_v1,num_good_v2);
}
