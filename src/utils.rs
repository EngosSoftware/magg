use std::fs;
use std::path::Path;

/// Separator line.
pub const SEPARATOR_LINE: &str = "────────────────────────────────────────────────────────────────────────────────";

pub fn read_file(file_name: impl AsRef<Path>) -> String {
  fs::read_to_string(file_name).expect("failed to read file")
}

pub fn write_file(file_name: impl AsRef<Path>, contents: &str) {
  fs::write(file_name, contents).expect("failed to write file")
}

pub fn parse_toml(file_name: impl AsRef<Path>) -> toml::Value {
  toml::from_str(&read_file(file_name)).expect("failed to parse TOML file")
}

pub fn get_package_name(parsed: &toml::Value) -> &str {
  parsed["package"]["name"].as_str().expect("package.name not found in Cargo.toml")
}
pub fn get_repository(parsed: &toml::Value) -> &str {
  parsed["package"]["repository"].as_str().expect("package.repository not found in Cargo.toml")
}
