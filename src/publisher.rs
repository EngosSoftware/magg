#![doc = include_str!("../docs/PUBLISHER.md")]

use crate::errors::*;
use std::ffi::OsStr;
use std::path::Path;

pub fn publish_crates(file_name: &str) -> Result<()> {
  println!("{}", file_name);
  Ok(())
}

fn execute_command<S, A, P>(program: S, args: A, dir: P) -> Result<()>
where
  S: AsRef<OsStr>,
  A: IntoIterator<Item = S>,
  P: AsRef<Path>,
{
  let mut command = std::process::Command::new(program);
  let mut child = command
    .args(args)
    .current_dir(dir)
    .stdin(std::process::Stdio::inherit())
    .stdout(std::process::Stdio::inherit())
    .stderr(std::process::Stdio::inherit())
    .spawn()
    .map_err(|e| MaggError::new(e.to_string()))?;
  child.wait().map_err(|e| MaggError::new(e.to_string()))?;
  Ok(())
}

#[cfg(test)]
mod tests {
  use crate::publisher::execute_command;
  use crate::utils::parse_toml;

  const TOML_FILE: &str = "../../CosmWasm/cosmwasm/Cargo.toml";

  #[test]
  fn validate_if_workspace_exists() {
    let toml_value = parse_toml(TOML_FILE);
    assert!(toml_value.get("workspace").is_some());
  }

  #[test]
  fn validate_that_package_does_not_exist() {
    let toml_value = parse_toml(TOML_FILE);
    assert!(toml_value.get("package").is_none());
  }

  #[test]
  fn validate_that_dependencies_exist() {
    let toml_value = parse_toml(TOML_FILE);
    assert!(toml_value.get("workspace").is_some());
    let workspace = &toml_value["workspace"];
    assert!(workspace.get("dependencies").is_some());
  }

  #[test]
  fn list_dependencies() {
    let toml_value = parse_toml(TOML_FILE);
    assert!(toml_value.get("workspace").is_some());
    let workspace = &toml_value["workspace"];
    assert!(workspace.get("dependencies").is_some());
    let dependencies = &workspace["dependencies"];
    assert!(dependencies.as_table().is_some());
    for (key, value) in dependencies.as_table().unwrap() {
      // Looks like the order of keys is preserved.
      _ = (key, value);
      //println!("{} {}", key, value);
    }
  }

  #[test]
  fn a() {
    execute_command("cargo", ["publish", "--dry-run", "--color=always"], ".").unwrap();
  }
}
