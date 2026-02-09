use crate::errors::*;
use std::io::{self, Write};
use std::path::Path;

/// Separator line.
pub const SEPARATOR_LINE: &str = "────────────────────────────────────────────────────────────────────────────────";

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

pub fn step_progress() {
  print!("·");
  io::stdout().flush().unwrap();
}
