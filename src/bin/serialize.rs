use std::env;
use std::fs;
use std::io::{BufReader, BufRead};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("{}", "Usage: serialize [FILE NAME]");
        return;
    }
    let result = serialize(&args[1]);
    println!("{}", result);
}

fn serialize(path: &str) -> String {
    let f = BufReader::new(fs::File::open(path).unwrap());
    let mut vec: Vec<String> = Vec::new();
    for line in f.lines() {
        let s = line.unwrap();
        vec.push(s.trim().to_string());
    }
    vec.join("").replace("\n", "")
}

#[test]
fn serialize_test() {
    let result = serialize("./test_files/serialize_test.xml");
    assert_eq!("<test><item></item></test>", result);
}