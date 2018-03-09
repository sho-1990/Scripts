use std::env;
use std::fs;
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", "Usage: serialize [FILE NAME]");
        return;
    }
    let f = BufReader::new(fs::File::open(&args[1]).unwrap());
    let mut vec: Vec<String> = Vec::new();
    for line in f.lines() {
        let s = line.unwrap();
        vec.push(s.trim().to_string());
    }
    println!("{}", vec.join("").replace("\n", ""));
}