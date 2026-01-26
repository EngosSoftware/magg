#![doc = include_str!("../docs/PUBLISHER.md")]

use crate::errors::*;
use crate::utils;
use crate::utils::parse_toml;
use std::ffi::OsStr;
use std::path::Path;

struct WorkspaceDependency {
  /// Name of the dependency in the workspace manifest.
  name: String,
  /// Local path where the dependency is defined.
  path: String,
  /// Line number where the dependency is defined in the workspace manifest.
  line: usize,
}

pub fn publish_crates(file_name: &str, dir: &str) -> Result<()> {
  let working_dir = Path::new(dir).canonicalize().map_err(|e| MaggError::new(e.to_string()))?;
  let workspace_manifest_file = working_dir.join(Path::new(file_name));
  let workspace_maifest_content = utils::read_file(workspace_manifest_file.clone())?;
  let workspace_manifest_toml = parse_toml(workspace_manifest_file.as_path())?;
  // Check if the manifest file is a Rust workspace.
  if workspace_manifest_toml.get("workspace").is_none() {
    return Err(MaggError::new(format!("Not a workspace manifest: {}", workspace_manifest_file.display())));
  }
  let workspace = &workspace_manifest_toml["workspace"];
  // Get dependencies having `path` attribute set.
  if workspace.get("dependencies").is_none() {
    return Err(MaggError::new("No dependencies section defined in the workspace"));
  }
  let dependencies = &workspace["dependencies"];
  if dependencies.as_table().is_none() {
    return Err(MaggError::new("Dependencies section is not a table"));
  }
  for (key, value) in dependencies.as_table().unwrap() {
    if value.get("path").is_some() {
      let path = value["path"].to_string();
      println!("{} {}", key, path);
    }
  }
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
    let toml_value = parse_toml(TOML_FILE).unwrap();
    assert!(toml_value.get("workspace").is_some());
  }

  #[test]
  fn validate_that_package_does_not_exist() {
    let toml_value = parse_toml(TOML_FILE).unwrap();
    assert!(toml_value.get("package").is_none());
  }

  #[test]
  fn validate_that_dependencies_exist() {
    let toml_value = parse_toml(TOML_FILE).unwrap();
    assert!(toml_value.get("workspace").is_some());
    let workspace = &toml_value["workspace"];
    assert!(workspace.get("dependencies").is_some());
  }

  #[test]
  fn list_dependencies() {
    let toml_value = parse_toml(TOML_FILE).unwrap();
    assert!(toml_value.get("workspace").is_some());
    let workspace = &toml_value["workspace"];
    assert!(workspace.get("dependencies").is_some());
    let dependencies = &workspace["dependencies"];
    assert!(dependencies.as_table().is_some());
    for (key, value) in dependencies.as_table().unwrap() {
      _ = (key, value);
      //println!("{} {}", key, value);
    }
  }

  #[test]
  fn a() {
    execute_command("cargo", ["publish", "--dry-run", "--color=always"], ".").unwrap();
  }
}
