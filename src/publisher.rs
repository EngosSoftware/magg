#![doc = include_str!("../docs/PUBLISHER.md")]

use crate::errors::*;
use crate::utils;
use crate::utils::{get_line_index, parse_toml};
use std::ffi::OsStr;
use std::path::Path;

struct WorkspaceDependency {
  /// Name of the dependency in the workspace manifest.
  name: String,
  /// Local path where the dependency is defined.
  path: String,
  /// Line number where the dependency is defined in the workspace manifest.
  line: usize,
  /// Search prefix in the original workspace manifest.
  prefix: String,
}

pub fn publish_crates(file_name: &str, dir: &str) -> Result<()> {
  let file_path = Path::new(file_name);
  let working_dir = utils::canonicalize(Path::new(dir))?;
  let workspace_manifest_file = working_dir.join(file_path);
  let workspace_maifest_content = utils::read_file(workspace_manifest_file.clone())?;
  let workspace_manifest_toml = parse_toml(workspace_manifest_file.as_path())?;
  // Check if the manifest file is a Rust workspace.
  if workspace_manifest_toml.get("workspace").is_none() {
    return Err(MaggError::new(format!("not a workspace manifest: {}", workspace_manifest_file.display())));
  }
  let workspace = &workspace_manifest_toml["workspace"];
  // Check if the workspace manifest has defined the publishing version.
  let Some(package) = workspace.get("package") else {
    return Err(MaggError::new("missing [workspace.package] section"));
  };
  let Some(version_value) = package.get("version") else {
    return Err(MaggError::new("missing 'version' in [workspace.package]"));
  };
  let Some(version) = version_value.as_str() else {
    return Err(MaggError::new("invalid 'version' in [workspace.package]"));
  };
  // Get dependencies having `path` attribute set.
  if workspace.get("dependencies").is_none() {
    return Err(MaggError::new("missing [workspace.dependencies] section"));
  }
  let dependencies = &workspace["dependencies"];
  if dependencies.as_table().is_none() {
    return Err(MaggError::new("[workspace.dependencies] section is not a table"));
  }
  let mut workspace_dependencies = vec![];
  for (key, value) in dependencies.as_table().unwrap() {
    if value.get("path").is_some() {
      let name = key.to_string();
      if value.get("version").is_some() {
        return Err(MaggError::new(format!("dependency '{name}' may not have 'version' set")));
      }
      let path = utils::strip_quotes(value["path"].as_str().unwrap()).to_string();
      let prefix = format!("{} = {{ path = \"{}\"", name, path);
      let Some(line) = get_line_index(&workspace_maifest_content, &prefix) else {
        return Err(MaggError::new(format!("invalid formatting for dependency '{name}'")));
      };
      workspace_dependencies.push(WorkspaceDependency { name, path, line, prefix });
    }
  }
  // Sort the workspace dependencies, to the publishing order is preserved.
  workspace_dependencies.sort_by_key(|value| value.line);
  if workspace_dependencies.is_empty() {
    return Err(MaggError::new("no crates to publish"));
  }
  // Validate crates in the workspace
  for workspace_dependency in &workspace_dependencies {
    println!("{}", workspace_dependency.name);
    let crate_manifest_file = utils::canonicalize(working_dir.join(Path::new(&workspace_dependency.path)).join(file_path))?;
    let crate_manifest_toml = parse_toml(crate_manifest_file)?;
  }
  println!("Publishing version: {}", version);
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
