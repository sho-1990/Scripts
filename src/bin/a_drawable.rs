use std::fs;
use std::io::Error;
use std::result::Result;

fn main() {
   let mut dirs: Vec<&str> = vec![];
   dirs.push("./drawable");
   dirs.push("./drawable-ldpi");
   dirs.push("./drawable-mdpi");
   dirs.push("./drawable-hdpi");
   dirs.push("./drawable-xhdpi");
   dirs.push("./drawable-xxhdpi");
   dirs.push("./drawable-xxxhdpi");

   for d in dirs {
      match do_create_dir(d) {
         Ok(_) => { println!("Create! {}", d) }
         Err(e) => {
            eprintln!("{}", e.to_string())
         }
      }
   }
}

fn do_create_dir(path: &str) -> Result<(), Error> {
   fs::create_dir(path)?;
   Ok(())
}