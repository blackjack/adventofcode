extern crate crypto;
extern crate rustc_serialize;

use crypto::md5::Md5;
use crypto::digest::Digest;
use rustc_serialize::hex::ToHex;
use std::thread;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::env;


static INPUT: &'static str = "iwrupvqb";

fn md5(s: &'static str, i: i64) -> [u8; 16] {
    let mut h = Md5::new();
    h.input(s.as_bytes());
    h.input(i.to_string().as_bytes());

    let mut res = [0; 16];
    h.result(&mut res);
    return res;
}


fn run(thread_num: i64, total_threads: i64, found5: &Arc<AtomicBool>, found6: &Arc<AtomicBool>) {

    println!("Spawning thread #{}", thread_num);
    let mut n = thread_num;

    loop {
        if found6.load(Ordering::Relaxed) {
            return;
        }
        let out = md5(INPUT, n);
        let first_five = out[0] as i32 + out[1] as i32 + (out[2] >> 4) as i32;
        if first_five == 0 {

            if !found5.load(Ordering::Relaxed) {
                println!("Number for MD5 with 5 zeroes is: {}, md5 hash is: {}",
                         n,
                         out.to_hex());
                found5.store(true, Ordering::Relaxed);
            }

            if out[2] == 0 {
                println!("Number for MD5 with 6 zeroes is: {}, md5 hash is: {}",
                         n,
                         out.to_hex());
                found6.store(true, Ordering::Relaxed);
                break;
            }
        }
        n += total_threads;
    }
}

fn main() {
    let mut num_threads = 8i64;

    let mut children = vec![];

    let found5 = AtomicBool::new(false);
    let found6 = AtomicBool::new(false);
    let af5 = Arc::new(found5);
    let af6 = Arc::new(found6);

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        num_threads = args[1].parse::<i64>().unwrap();
    }

    for i in 0..num_threads {
        // Spin up another thread
        let f5 = af5.clone();
        let f6 = af6.clone();
        children.push(thread::spawn(move || {
            run(i, num_threads, &f5, &f6);
        }));
    }

    for child in children {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join();
    }
}
