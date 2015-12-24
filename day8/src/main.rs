use std::fs::File;
use std::io::prelude::*;


fn escape(s: String) -> String {
    let mut result: String = s.to_string();
    if result.is_empty() { return result }


    result = result.replace("\\\"","");
    result = result.replace("\\\\","");

    let matches: Vec<_> = result.match_indices("\\x").collect();
    for (pos,m) in matches {
        let c1 =result.char_at(pos+2);
        let c2 =result.char_at(pos+3);
        let c = c1.to_digit(16).unwrap()*10+c2.to_digit(16).unwrap();

        result = result.replace(&result[pos..4],&c.to_string());
    }

    result.trim_matches('"');


    return result;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = std::io::BufReader::new(file);


    for line in reader.lines() {
        let line = line.unwrap();

        let code = line.len();

        let strlen = escape(line).len();
    }
}




