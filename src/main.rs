mod posts;

fn main() {
  let posts = posts::get_all_posts().unwrap();
  println!("{:?}", posts);
}
