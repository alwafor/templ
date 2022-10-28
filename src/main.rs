use colored::Colorize;

use std::{
  env,
  fs::File,
  io::{BufRead, BufReader, Error},
};
fn main() {
  let args: Vec<String> = env::args().collect();
  let result = get_file_text(args.get(1).unwrap_or(&"text.txt".to_owned()));
  println!("{}", result.unwrap());
}

fn get_file_text(path: &String) -> Result<String, Error> {
  let mut input = File::open(path)?;
  let buffered = BufReader::new(input);

  let mut text_data = String::from("");
  for line in buffered.lines() {
    text_data.push_str(&line?);
    text_data.push_str("\n")
  }

  Ok(text_data)
}
