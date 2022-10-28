use colored::Colorize;

use std::{
  env, fs,
  io::Error,
  path::{Path, PathBuf},
};

fn main() {
  let args: Vec<String> = env::args().collect();
  args.get(1).unwrap_or_else(|| {
    panic!("Error! Arguments are not present!");
  });

  let path_buf = Path::new(&args[0]).join(&"..").join(&args[1]);
  let result = get_file_text(&path_buf);
  println!("{}", result.unwrap());
}

fn get_file_text(path: &PathBuf) -> Result<String, Error> {
  let data = fs::read_to_string(path)?;

  Ok(data)
}
