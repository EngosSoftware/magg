#![doc = include_str!("../docs/PUBLISHER.md")]

use crate::errors::*;
use crate::utils;
use crate::utils::{ask_for_choice, get_line_index, parse_toml};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// A crate dependency defined in the workspace manifest to be published.
struct CrateToPublish {
  /// Name of the crate in the workspace manifest to be published.
  name: String,
  /// Local path where the crate is defined.
  path: String,
  /// Line number where the dependency to this crate is defined in the workspace manifest.
  line: usize,
  /// Search prefix in the original workspace manifest.
  prefix: String,
  /// Published crate dependency.
  published_prefix: String,
  /// Working directory for published crate.
  dir: PathBuf,
}

pub fn publish_crates(file_name: &str, dir: &str, timeout: u64) -> Result<()> {
  let file_path = Path::new(file_name);
  let working_dir = utils::canonicalize(Path::new(dir))?;
  let workspace_manifest_file = utils::canonicalize(working_dir.join(file_path))?;
  let workspace_manifest_path = workspace_manifest_file.as_path();
  let mut workspace_maifest_content = utils::read_file(workspace_manifest_path)?;
  let workspace_manifest_toml = parse_toml(workspace_manifest_path)?;
  // Check if the manifest file is a Rust workspace.
  let Some(workspace) = workspace_manifest_toml.get("workspace") else {
    return Err(MaggError::new(format!(
      "missing [workspace] section in manifest file: {}",
      workspace_manifest_path.display()
    )));
  };
  // Check if the workspace manifest has package section.
  let Some(workspace_package) = workspace.get("package") else {
    return Err(MaggError::new("missing [workspace.package] section"));
  };
  // Check if the workspace manifest has defined the version to be published.
  let Some(workspace_package_version) = workspace_package.get("version") else {
    return Err(MaggError::new("missing 'version' in [workspace.package]"));
  };
  // Check if the version is a string.
  let Some(publish_version) = workspace_package_version.as_str() else {
    return Err(MaggError::new("invalid 'version' in [workspace.package]"));
  };
  // Get dependencies section.
  let Some(workspace_dependencies) = workspace.get("dependencies") else {
    return Err(MaggError::new("missing [workspace.dependencies] section"));
  };
  // Check is dependencies section is a table.
  let Some(workspace_dependencies_table) = workspace_dependencies.as_table() else {
    return Err(MaggError::new("[workspace.dependencies] section is not a table"));
  };
  // Collect crates to be published.
  let mut crates_to_publish = vec![];
  for (key, value) in workspace_dependencies_table {
    if value.get("path").is_some() {
      let name = key.to_string();
      if value.get("version").is_some() {
        return Err(MaggError::new(format!("dependency '{name}' may not have 'version' set")));
      }
      let path = utils::strip_quotes(value["path"].as_str().unwrap()).to_string();
      let prefix = format!("{} = {{ path = \"{}\"", name, path);
      let published_prefix = format!("{} = {{ version = \"{}\"", name, publish_version);
      let dir = utils::canonicalize(working_dir.join(Path::new(&path)))?;
      let Some(line) = get_line_index(&workspace_maifest_content, &prefix) else {
        return Err(MaggError::new(format!("invalid formatting for dependency '{name}'")));
      };
      crates_to_publish.push(CrateToPublish {
        name,
        path,
        line,
        prefix,
        published_prefix,
        dir,
      });
    }
  }
  // Sort crates so the publishing order is preserved.
  crates_to_publish.sort_by_key(|value| value.line);
  if crates_to_publish.is_empty() {
    return Err(MaggError::new("no crates to publish"));
  }
  // Validate crates' manifest files.
  for crate_to_publish in &crates_to_publish {
    let name = crate_to_publish.name.to_string();
    let crate_manifest_file = utils::canonicalize(working_dir.join(Path::new(&crate_to_publish.path)).join(file_path))?;
    let crate_manifest_toml = parse_toml(crate_manifest_file)?;
    let Some(crate_package) = crate_manifest_toml.get("package") else {
      return Err(MaggError::new(format!("missing [package] section in dependency '{name}'")));
    };
    let Some(crate_package_name) = crate_package.get("name") else {
      return Err(MaggError::new(format!("missing [package].name attribute in dependency '{name}'")));
    };
    let Some(package_name) = crate_package_name.as_str() else {
      return Err(MaggError::new(format!("invalid [package].name attribute in dependency '{name}'")));
    };
    if crate_to_publish.name != package_name {
      return Err(MaggError::new(format!("expected 'name = \"{name}\"', actual 'name = \"{}\"'", package_name)));
    }
    let Some(crate_package_version) = crate_package.get("version") else {
      return Err(MaggError::new(format!("missing [package].version attribute in dependency '{name}'")));
    };
    let Some(crate_package_version_workspace) = crate_package_version.get("workspace") else {
      return Err(MaggError::new(format!("missing [package].version.workspace attribute in dependency '{name}'")));
    };
    let Some(crate_package_version_workspace_value) = crate_package_version_workspace.as_bool() else {
      return Err(MaggError::new(format!("invalid [package].version.workspace attribute in dependency '{name}'")));
    };
    if !crate_package_version_workspace_value {
      return Err(MaggError::new(format!("[package].version.workspace attribute in crate '{name}' must have value \"true\"")));
    }
    validate_crate_dependencies(crate_package, "dependencies", &name, &crates_to_publish)?;
    validate_crate_dependencies(crate_package, "dev-dependencies", &name, &crates_to_publish)?;
  }

  //---------------------
  // Publish crates
  //---------------------

  // Ask if the version to be published is the right one.
  println!();
  println!("Publishing version: {}", publish_version);
  if !ask_for_choice("Is this correct?")? {
    return Ok(());
  }

  // List all crates to be publishe with versions and ask if the list is correct.\
  println!();
  println!("Publishing crates:");
  for crate_to_publish in &crates_to_publish {
    println!("{} v{} {}", crate_to_publish.name, publish_version, crate_to_publish.dir.display());
  }
  println!();
  if !ask_for_choice("Is this correct?")? {
    return Ok(());
  }

  for crate_to_publish in &crates_to_publish {
    // Ask if perform dry-run before publishing.
    println!("\nCrate: {} v{} {}", crate_to_publish.name, publish_version, crate_to_publish.dir.display());
    if ask_for_choice("Perform dry-run before publishing?")? {
      execute_command("cargo", ["publish", "--dry-run", "--color=always"], crate_to_publish.dir.clone())?;
    }
    // Ask if publish the crate.
    println!("\nCrate: {} v{} {}", crate_to_publish.name, publish_version, crate_to_publish.dir.display());
    if ask_for_choice("Publish this crate?")? {
      execute_command("cargo", ["publish", "--color=always"], crate_to_publish.dir.clone())?;
    }
    if timeout > 0 {
      // Wait a timeout, just to make sure the crate is fully published.
      std::thread::sleep(std::time::Duration::new(timeout, 0));
    }

    // After publishing the crate, replace the 'path' with the published 'version'.
    workspace_maifest_content = workspace_maifest_content.replace(&crate_to_publish.prefix, &crate_to_publish.published_prefix);
    // Save the modified version of the workspace manifest, so other crates will use the published versions of dependencies.
    utils::write_file(workspace_manifest_path, &workspace_maifest_content);
  }
  Ok(())
}

fn validate_crate_dependencies(package: &toml::Value, dependencies: &str, crate_name: &str, crates_to_publish: &[CrateToPublish]) -> Result<()> {
  if let Some(crate_package_dependencies) = package.get(dependencies) {
    let Some(crate_dependencies_table) = crate_package_dependencies.as_table() else {
      return Err(MaggError::new(format!("[package.dependencies] section is not a table in crate {crate_name}")));
    };
    for (key, value) in crate_dependencies_table {
      if crates_to_publish.iter().any(|value| value.name == *key) {
        let Some(crate_dependency_workspace) = value.get("workspace") else {
          return Err(MaggError::new(format!("missing [package.dependencies].{key}.workspace attribute in crate '{crate_name}'")));
        };
        let Some(crate_dependency_workspace_value) = crate_dependency_workspace.as_bool() else {
          return Err(MaggError::new(format!("invalid [package.dependencies].{key}.workspace attribute in crate '{crate_name}'")));
        };
        if !crate_dependency_workspace_value {
          return Err(MaggError::new(format!(
            "[package.dependencies].{key}.workspace attribute in crate '{crate_name}' must have value \"true\""
          )));
        }
      }
    }
  };
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
  let exit_status = child.wait().map_err(|e| MaggError::new(e.to_string()))?;
  if !exit_status.success() {
    return Err(MaggError::new(format!("executing command failed with status code: {}", exit_status)));
  }
  Ok(())
}
