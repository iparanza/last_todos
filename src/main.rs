use std::{fs::File, io::BufReader};

use rev_lines::RevLines;

fn main() {
  let todo_file = File::open("/home/ipa/todo").expect("Failed to read todo file.");
  let rev_lines = RevLines::new(BufReader::new(todo_file)).unwrap();
  for line in rev_lines {
    if line.starts_with("---") {
      return;
    }
    println!("{}", line);
  }
}
