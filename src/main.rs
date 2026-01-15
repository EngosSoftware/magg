use std::fs;
use std::path::Path;

fn parse_toml(file_name: impl AsRef<Path>) -> toml::Value {
  let content = fs::read_to_string(file_name).expect("failed to read TOML file");
  toml::from_str(&content).expect("failed to parse TOML file")
}

fn get_package_name(parsed: &toml::Value) -> &str {
  parsed["package"]["name"].as_str().expect("package.name not found in Cargo.toml")
}

fn main() {
  let parsed_toml = parse_toml("Cargo.toml");
  let package_name = get_package_name(&parsed_toml);
  println!("### {}", package_name);
  println!();
  println!("[![crates.io][crates-badge]][crates-url]");
  println!("[![Code coverage][cov-badge]][cov-url]");
  println!();
  println!("[crates-badge]: https://img.shields.io/crates/v/{package_name}.svg");
  println!("[crates-url]: https://crates.io/crates/{package_name}");
  println!("[cov-badge]: https://img.shields.io/badge/coverage-0%25-21b577.svg");
  println!("[cov-url]: https://crates.io/crates/coverio");
}
