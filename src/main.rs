use walkdir::WalkDir;

fn traverse_posts_directory() -> Result<(), ()> {
  for entry in WalkDir::new("./posts")
    .follow_links(true)
    .into_iter()
    .filter_map(|e| e.ok())
  {
    let file_name = entry.file_name().to_string_lossy();
    if file_name.ends_with(".md") {
      println!("{}", file_name);
    }
  }
  Ok(())
}

fn main() {
  traverse_posts_directory().unwrap();
}
