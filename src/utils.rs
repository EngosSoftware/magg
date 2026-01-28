use crate::errors::*;
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Separator line.
pub const SEPARATOR_LINE: &str = "────────────────────────────────────────────────────────────────────────────────";

/// Default name of the Rust manifest file.
pub const RUST_MANIFEST_FILE_NAME: &str = "Cargo.toml";

pub fn read_file(file_name: impl AsRef<Path>) -> Result<String> {
  let path = file_name.as_ref();
  std::fs::read_to_string(path).map_err(|e| error_read_file(path, e))
}

pub fn write_file(file_name: impl AsRef<Path>, contents: &str) -> Result<()> {
  let file_path = file_name.as_ref();
  std::fs::write(file_path, contents).map_err(|e| MaggError::new(format!("failed to write file {}, reason: {}", file_path.display(), e)))
}

pub fn parse_toml(file_name: impl AsRef<Path>) -> Result<toml::Value> {
  toml::from_str(&read_file(file_name)?).map_err(|e| MaggError::new(e.to_string()))
}

pub fn get_package_name(parsed: &toml::Value) -> &str {
  parsed["package"]["name"].as_str().expect("package.name not found in Cargo.toml")
}
pub fn get_repository(parsed: &toml::Value) -> &str {
  parsed["package"]["repository"].as_str().expect("package.repository not found in Cargo.toml")
}

pub fn get_line_number(content: &str, prefix: &str) -> Option<usize> {
  for (index, line) in content.lines().enumerate() {
    if line.starts_with(prefix) {
      return Some(index + 1);
    }
  }
  None
}

pub fn strip_quotes(s: &str) -> &str {
  match s.as_bytes() {
    [b'"', .., b'"'] | [b'\'', .., b'\''] => &s[1..s.len() - 1],
    _ => s,
  }
}

pub fn canonicalize(path: impl AsRef<Path>) -> Result<PathBuf> {
  path
    .as_ref()
    .canonicalize()
    .map_err(|e| MaggError::new(format!("failed to canonicalize path: {}, reason: {}", path.as_ref().display(), e)))
}

pub fn step_progress() {
  print!("·");
  io::stdout().flush().unwrap();
}

pub fn ask_for_choice(prompt: &str, accept: bool) -> Result<bool> {
  if accept {
    return Ok(true);
  }
  loop {
    print!("{} [Y/n]: ", prompt);
    io::stdout().flush().map_err(|e| MaggError::new(format!("failed to flush stdout, reason: {}", e)))?;
    let mut input = String::new();
    io::stdin()
      .read_line(&mut input)
      .map_err(|e| MaggError::new(format!("failed to read line, reason: {}", e)))?;
    match input.trim().to_lowercase().as_str() {
      "y" => return Ok(true),
      "n" => return Ok(false),
      _ => println!("Please enter 'Y' or 'n'"),
    }
  }
}
