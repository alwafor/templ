use colored::Colorize;

use std::{
  env::{self, current_exe},
  fs,
  io::Error,
  path::{Path, PathBuf},
};

fn main() {
  let arguments = get_cli_arguments();
  let file_text = get_file_text(&arguments[0]);
  println!("{}", file_text.unwrap());
}

fn get_cli_arguments() -> Vec<String> {
  let args: Vec<String> = env::args().collect();
  args.get(1).unwrap_or_else(|| {
    panic!("Error! Arguments are not present!");
  });
  args[1..].to_vec()
}

fn get_file_text(path_part: &String) -> Result<String, Error> {
  let path_buf = Path::new(&current_exe()?).join(&"..").join(path_part);

  let data = fs::read_to_string(path_buf)?;

  Ok(data)
}
