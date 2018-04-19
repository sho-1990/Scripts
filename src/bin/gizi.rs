extern crate toml;

#[macro_use] extern crate serde_derive;
extern crate chrono;

use chrono::Local;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::process::Command;


#[derive(Debug)]
#[derive(Deserialize)]
struct Config {
    gizi: Gizi
}

#[derive(Debug)]
#[derive(Deserialize)]
struct Gizi {
    projects: String,
    editor: String,
    extension: String
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
       print_usage();
       return;
    }
    if &args[1] == "projects" {
        projects_process();
        return
    }
    if args.len() < 3 {
        open_process(&args[1]);
    } else if args.len() > 2 {
        new_process(&args[2]);
    } else {
        print_usage();
    }
}

fn new_process(project_name: &str) {
    let config: Config = read_config();
    match fs::create_dir(&config.gizi.projects) {
        Ok(_) => {}
        Err(_) => {}
    }
    let dir_name = format!("{}/{}", config.gizi.projects, project_name);
    let mut d = fs::create_dir(&dir_name);
    match d {
        Ok(_) => { println!("{} {}", &dir_name, "create!") }
        Err(why) => { println!("{}", why.to_string()) }
    }
}

fn open_process(project_name: &str) {
    let config: Config = read_config();
    let date = Local::now();
    let file_name = format!("{}.{}", date.format("%Y-%m-%d"), &config.gizi.extension);
    let file_name = format!("{}/{}/{}", &config.gizi.projects, project_name, file_name);
    // すでにファイルがあったら開くだけ
    let _ = match File::open(&file_name) {
        Ok(_) => {
            println!("{} exists", &file_name);
            open_editor(&file_name, &config.gizi.editor);
            return;
        },
        Err(_) => {}
    };
    let mut f = File::create(&file_name).unwrap();
    println!("{} create!", &file_name);
    open_editor(&file_name, &config.gizi.editor);
}

fn projects_process() {
    let config = read_config();
    match fs::read_dir(&config.gizi.projects) {
        Err(why) => println!("{:?}", why.kind()),
        Ok(paths) => for path in paths {
            println!("> {}", path.unwrap().path().to_str().unwrap())
        }
    }
}

fn read_config() -> Config {
    let path = Path::new("./Config.toml");
    let mut f = File::open(&path).expect("Please set config file");
    let mut s = String::new();
    f.read_to_string(&mut s).unwrap();
    toml::from_str(&s).unwrap()
}

fn print_usage() {
    eprintln!("Usage:\n gizi [project name]\n gizi new [project_name]\n gizi projects");
}

fn open_editor(file_name: &str, editor: &str) {
    Command::new(editor).arg(file_name).output().expect("Please set editor name");
}