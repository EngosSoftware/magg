use std::fs;
use std::path::Path;

pub fn get_file(file_name: impl AsRef<Path>) -> String {
  fs::read_to_string(file_name).expect("failed to read file")
}

pub fn parse_toml(file_name: impl AsRef<Path>) -> toml::Value {
  toml::from_str(&get_file(file_name)).expect("failed to parse TOML file")
}

pub fn get_package_name(parsed: &toml::Value) -> &str {
  parsed["package"]["name"].as_str().expect("package.name not found in Cargo.toml")
}
pub fn get_repository(parsed: &toml::Value) -> &str {
  parsed["package"]["repository"].as_str().expect("package.repository not found in Cargo.toml")
}
