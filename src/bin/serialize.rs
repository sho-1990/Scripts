use std::env;
use std::fs;
use std::io::{BufReader, BufRead};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = if args.len() < 2 {
        "serialize.txt"
    } else {
        &args[1]
    };
    let path = Path::new(file_name);
    if !path.exists() {
        println!("Usage serialize [FILE NAME] or setting ./serialize.txt");
        return
    }

    let result = serialize(file_name);
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