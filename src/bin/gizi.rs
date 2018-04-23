extern crate toml;

#[macro_use] extern crate serde_derive;
extern crate chrono;

use chrono::Local;
use std::env;
use std::fs;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::option::Option;
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

    let mut latest_file = match latest_file(project_name, &config) {
        None => {
            File::create(&file_name).unwrap();
            open_editor(&file_name, &config.gizi.editor);
            return
        },
        Some(l) => {
            l.unwrap()
        }
    };
    let mut f = File::create(&file_name).unwrap();
    let mut latest = String::new();
    latest_file.read_to_string(&mut latest).unwrap();
    f.write_all(latest.as_bytes()).unwrap();

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

fn latest_file(project_name: &str, config: &Config) -> Option<std::io::Result<File>> {
    let dir_name = format!("{}/{}", config.gizi.projects, project_name);
    let paths = fs::read_dir(&dir_name).unwrap();
    let mut files: Vec<String> = vec![];
    for path in paths {
        let name = path.unwrap().path().display().to_string();
        let names: Vec<&str> = name.split("/").collect();
        let file_name = names.last().unwrap().to_string();
        let file_len = names.last().unwrap().len();
        if &file_name[(file_len - 2)..file_len] != config.gizi.extension {
            println!("Ignore name: {}", file_name);
            continue;
        }
        let file_name = file_name[..(file_len - 3)].to_string();

        files.push(file_name);
    }
    if files.len() > 2 {
        files.sort_by(|a, b| {
            let a_v: Vec<&str> = a.split("-").collect();
            let b_v: Vec<&str> = b.split("-").collect();
            let a_i: i32 = a_v.concat().parse().unwrap();
            let b_i: i32 = b_v.concat().parse().unwrap();
            a_i.cmp(&b_i)
        });
    } else if files.len() == 0 {
        return None
    }
    let file_name = format!("{}/{}.{}", &dir_name, files.last().unwrap(), config.gizi.extension);
    println!("Selected Name: {}", file_name);
    Some(File::open(file_name))
}
