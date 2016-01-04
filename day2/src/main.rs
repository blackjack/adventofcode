use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::min;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut total = 0;
    let mut ribbon = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let mut vec: Vec<u32> = line.split("x").map(|s| s.parse::<u32>().unwrap()).collect();

        assert_eq!(vec.len(), 3);
        let l = vec[0];
        let w = vec[1];
        let h = vec[2];

        let extra = min(l * w, min(l * h, w * h));
        total += 2 * l * w + 2 * w * h + 2 * h * l + extra;

        vec.sort();
        let r1 = vec[0];
        let r2 = vec[1];

        ribbon += r1 + r1 + r2 + r2 + l * w * h;
    }

    println!("Total paper required: {}, total ribbon length: {}",
             total,
             ribbon);
}
