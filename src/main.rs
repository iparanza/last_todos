use std::{fs::File, io::BufReader};

use rev_lines::RevLines;


fn main() {
  #![allow(deprecated)] // Deprecated because of  windows support.
  let mut todo_file = std::env::home_dir().expect("Failed to get home dir.");
  todo_file.push("todo");
  let todo_file = File::open(todo_file).expect("Failed to read todo file.");
  let rev_lines = RevLines::new(BufReader::new(todo_file)).unwrap();
  for line in rev_lines {
    if line.starts_with("---") {
      return;
    }
    println!("{}", line);
  }
}
