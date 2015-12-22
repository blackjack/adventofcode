use std::io::BufReader;
use std::io::BufRead;
use std::fs::File;
use std::cmp::min;

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(&file);

    let mut total = 0;

    for line in reader.lines() {
        let l = line.unwrap();
        if l.is_empty() { continue; }
        let vec: Vec<&str> = l.split("x").collect();

        assert_eq!(vec.len(),3);
        let l = vec[0].parse::<u32>().unwrap();
        let w = vec[1].parse::<u32>().unwrap();
        let h = vec[2].parse::<u32>().unwrap();

        let extra = min(l*w,min(l*h,w*h));
        total+=2*l*w + 2*w*h + 2*h*l + extra;
    }

    println!("Total paper required: {}",total);
}
