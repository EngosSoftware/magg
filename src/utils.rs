use crate::errors::*;
use std::fs;
use std::path::{Path, PathBuf};

/// Separator line.
pub const SEPARATOR_LINE: &str = "────────────────────────────────────────────────────────────────────────────────";

/// Default name of the Rust manifest file.
pub const RUST_MANIFEST_FILE_NAME: &str = "Cargo.toml";

pub fn read_file(file_name: impl AsRef<Path>) -> Result<String> {
  let path = file_name.as_ref();
  fs::read_to_string(path).map_err(|e| error_read_file(path, e))
}

pub fn write_file(file_name: impl AsRef<Path>, contents: &str) {
  fs::write(file_name, contents).expect("failed to write file")
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

pub fn get_line_index(content: &str, prefix: &str) -> Option<usize> {
  for (index, line) in content.lines().enumerate() {
    // println!("line: {}", line);
    // println!("prefix: {}", prefix);
    if line.starts_with(prefix) {
      return Some(index);
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
    .map_err(|e| MaggError::new(format!("failed to canonicalize path: {}", path.as_ref().display())))
}
