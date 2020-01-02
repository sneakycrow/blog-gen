use comrak::ComrakOptions;
use serde_derive::Deserialize;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use walkdir::WalkDir;

#[derive(Debug, PartialEq, Deserialize)]
struct YamlHeader {
  title: String,
  author: String,
  #[serde(default)]
  published: bool,
}

#[derive(Debug)]
pub struct Post {
  yaml: YamlHeader,
  content: String
}

fn get_yaml(contents: &String) -> Result<YamlHeader, Box<dyn Error>> {
  let end_of_yaml = contents[4..].find("---").unwrap() + 4;
  let yaml = &contents[..end_of_yaml];
  let YamlHeader {
    author,
    title,
    published,
  } = serde_yaml::from_str(yaml)?;
  Ok(YamlHeader { author, title, published })
}

fn get_post_html(contents: &String) -> String {
  let end_of_yaml = contents[4..].find("---").unwrap() + 4;
  let options = ComrakOptions {
    ext_header_ids: Some(String::new()),
    unsafe_: true, // Allow rendering of raw HTML
    ..ComrakOptions::default()
  };

  let contents = comrak::markdown_to_html(&contents[end_of_yaml + 5..], &options);
  contents
}

fn open_post(file: File) -> Result<Post, Box<dyn Error>> {
  let mut buf_reader = BufReader::new(file);
  let mut contents = String::new();
  match buf_reader.read_to_string(&mut contents) {
    _ => {
      let yaml_content = get_yaml(&contents).unwrap();
      let post_content = get_post_html(&contents);
      let post = Post {
        yaml: yaml_content,
        content: post_content
      };
      Ok(post)
    }
  }
}

pub fn get_all_posts() -> Result<Vec<Post>, Box<dyn Error>> {
  let mut all_posts: Vec<Post> = Vec::new();
  for entry in WalkDir::new("./posts")
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let file_path = entry.path();
    let file_name = entry.file_name().to_string_lossy();
    let file = File::open(file_path)?;
    if file_name.ends_with(".md") {
      let post = open_post(file).unwrap();
      all_posts.push(post);
    }
  }
  Ok(all_posts)
}