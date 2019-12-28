use walkdir::WalkDir;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::error::Error;
use serde_derive::{Deserialize};

#[derive(Debug, PartialEq, Deserialize)]
struct YamlHeader {
    title: String,
    author: String,
    #[serde(default)]
    published: bool
}

fn open_post(file: File) -> Result<(), Box<dyn Error>> {
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  match buf_reader.read_to_string(&mut contents) {
    Err(e) => println!("Error: {:?}", e),
    _ => {
      // println!("{:?}", contents);
      let end_of_yaml = contents[4..].find("---").unwrap() + 4;
      let yaml = &contents[..end_of_yaml];
      println!("{:?}", yaml);
      let YamlHeader {
        author,
        title,
        published
      } = serde_yaml::from_str(yaml)?;
      println!("{:?}", YamlHeader { author, published, title });
    }
  }
  Ok(())
}

fn traverse_posts_directory() -> Result<(), Box<dyn Error>> {
  for entry in WalkDir::new("./posts")
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let file_path = entry.path();
    let file_name = entry.file_name().to_string_lossy();
    let file = File::open(file_path)?;
    if file_name.ends_with(".md") {
      open_post(file).unwrap();
    }
  }
  Ok(())
}

fn main() {
  traverse_posts_directory().unwrap();
}
