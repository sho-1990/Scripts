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


#[test]
fn test_do_create_dir() {
   let path = "./test_do_create_dir";
   let dir = std::path::Path::new(path);
   if dir.exists() {
      fs::remove_dir_all(path).expect("failed remove dir");
   }
   do_create_dir(path).unwrap();
   assert_eq!(true, dir.exists());

   fs::remove_dir_all(path).expect("failed remove dir after test");

}