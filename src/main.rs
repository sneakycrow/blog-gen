use walkdir::WalkDir;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn open_post(file: File) {
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  match buf_reader.read_to_string(&mut contents) {
    Err(e) => println!("Error: {:?}", e),
    _ => println!("{:?}", contents)
  }
}

fn traverse_posts_directory() -> Result<(), ()> {
  for entry in WalkDir::new("./posts")
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let file_path = entry.path();
    let file_name = entry.file_name().to_string_lossy();
    let file = File::open(file_path);
    if file_name.ends_with(".md") {
      match file {
        Ok(file) => open_post(file),
        Err(error) => println!("Error: {:?}", error)
      }
    }
  }
  Ok(())
}

fn main() {
  traverse_posts_directory().unwrap();
}
