#![doc = include_str!("../docs/PUBLISHER.md")]

use crate::errors::*;
use crate::utils;
use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

type CratesToPublish = BTreeMap<usize, CrateToPublish>;

/// A crate dependency defined in the workspace manifest to be published.
struct CrateToPublish {
  /// Name of the crate in the workspace manifest to be published.
  name: String,
  /// Local path where the crate is defined.
  path: String,
  /// Search prefix in the original workspace manifest.
  prefix: String,
  /// Published crate dependency.
  published_prefix: String,
  /// Working directory for published crate.
  dir: PathBuf,
  /// Number of the line in the workspace manifest where the dependency is placed.
  line_number: usize,
}

pub fn publish_crates(file_name: &str, dir: &str, timeout: u64, accept_all: bool, simulation: bool) -> Result<()> {
  let file_path = Path::new(file_name);
  let working_dir = utils::canonicalize(Path::new(dir))?;
  let workspace_manifest_file = utils::canonicalize(working_dir.join(file_path))?;
  let workspace_manifest_path = workspace_manifest_file.as_path();
  let mut workspace_maifest_content = utils::read_file(workspace_manifest_path)?;
  let workspace_manifest_toml = utils::parse_toml(workspace_manifest_path)?;
  // Check if the manifest file is a Rust workspace.
  let Some(workspace) = workspace_manifest_toml.get("workspace") else {
    return Err(MaggError::new("missing [workspace] section in the workspace manifest file"));
  };
  // Check if the workspace manifest has package section.
  let Some(workspace_package) = workspace.get("package") else {
    return Err(MaggError::new("missing [workspace.package] section"));
  };
  // Check if the workspace manifest has defined the version to be published.
  let Some(workspace_package_version) = workspace_package.get("version") else {
    return Err(MaggError::new("missing 'version' entry in [workspace.package] section"));
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

  //-----------------------------------
  // Collect crates to publish
  //-----------------------------------

  let mut crates_to_publish = CratesToPublish::new();
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
      let Some(line_number) = utils::get_line_number(&workspace_maifest_content, &prefix) else {
        return Err(MaggError::new(format!("invalid formatting for dependency '{name}'")));
      };
      crates_to_publish.insert(
        line_number,
        CrateToPublish {
          name,
          path,
          prefix,
          published_prefix,
          dir,
          line_number,
        },
      );
    }
  }
  if crates_to_publish.is_empty() {
    return Err(MaggError::new("no crates to publish"));
  }

  //---------------------
  // Validate crates
  //---------------------

  for crate_to_publish in crates_to_publish.values() {
    let name = crate_to_publish.name.to_string();
    let crate_manifest_file = utils::canonicalize(working_dir.join(Path::new(&crate_to_publish.path)).join(file_path))?;
    let crate_manifest_toml = utils::parse_toml(crate_manifest_file)?;
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
    if let Some(dependencies) = crate_package.get("dependencies") {
      validate_crate_dependencies(dependencies, crate_to_publish, &crates_to_publish)?;
    }
    if let Some(dependencies) = crate_package.get("dev-dependencies") {
      validate_crate_dependencies(dependencies, crate_to_publish, &crates_to_publish)?;
    }
  }

  //---------------------
  // Publish crates
  //---------------------

  // Ask if the version to be published is the right one.
  println!();
  println!("Publish version: {}", publish_version);
  if !utils::ask_for_choice("Is this version correct?", accept_all)? {
    return Ok(());
  }

  // List all crates to be publishe with versions and ask if the list is correct.\
  println!();
  println!("Publish crates:");
  for crate_to_publish in crates_to_publish.values() {
    println!("{} v{} {}", crate_to_publish.name, publish_version, crate_to_publish.dir.display());
  }
  println!();
  if !utils::ask_for_choice("Do you want to publish all these crates?", accept_all)? {
    return Ok(());
  }

  for crate_to_publish in crates_to_publish.values() {
    // Ask if perform dry-run before publishing.
    println!(
      "\nCrate (dry-run):\n  {}\n  v{}\n  {}",
      crate_to_publish.name,
      publish_version,
      crate_to_publish.dir.display()
    );
    if utils::ask_for_choice("Perform dry-run before publishing this crate?", accept_all)? {
      if simulation {
        execute_command("echo", ["simulating <dry-run>"], crate_to_publish.dir.clone())?;
      } else {
        execute_command("cargo", ["publish", "--dry-run", "--color=always"], crate_to_publish.dir.clone())?;
      }
    }
    // Ask if publish the crate.
    println!(
      "\nCrate (publish):\n  {}\n  v{}\n  {}",
      crate_to_publish.name,
      publish_version,
      crate_to_publish.dir.display()
    );
    if utils::ask_for_choice("Publish this crate?", accept_all)? {
      if simulation {
        execute_command("echo", ["simulating <publish>"], crate_to_publish.dir.clone())?;
      } else {
        execute_command("cargo", ["publish", "--color=always"], crate_to_publish.dir.clone())?;
      }
    }
    // Wait a timeout, just to make sure that the crate is fully published.
    if timeout > 0 && !simulation {
      print!("Waiting {} second(s), ", timeout);
      let one_second = std::time::Duration::new(1, 0);
      for _ in 0..timeout {
        utils::step_progress();
        std::thread::sleep(one_second);
      }
    }
    // After publishing the crate, replace the 'path' with the published 'version'.
    workspace_maifest_content = workspace_maifest_content.replace(&crate_to_publish.prefix, &crate_to_publish.published_prefix);
    // Save the modified version of the workspace manifest, so other crates will use the published versions of dependencies.
    utils::write_file(workspace_manifest_path, &workspace_maifest_content);
  }
  // TODO Read the workspace manifest from disk and check is all paths are replaces by versions.
  Ok(())
}

fn validate_crate_dependencies(dependencies: &toml::Value, crate_to_publish: &CrateToPublish, crates_to_publish: &CratesToPublish) -> Result<()> {
  // Make sure the dependency section is a TOML table.
  let Some(crate_dependencies_table) = dependencies.as_table() else {
    return Err(MaggError::new(format!("dependencies section is not a table in crate '{}'", crate_to_publish.name)));
  };
  // Iterate over all dependencies.
  for (key, value) in crate_dependencies_table {
    // Iterate over all crates to be published to check if this crate has them as dependencies.
    for dependency_crate_to_publish in crates_to_publish.values() {
      if *key == dependency_crate_to_publish.name {
        // Make sure the dependency is defined in the workspace manifest.
        let Some(crate_dependency_workspace) = value.get("workspace") else {
          return Err(MaggError::new(format!("missing dependency {key}.workspace attribute in crate '{}'", crate_to_publish.name)));
        };
        // Make sure the workspace dependency is ot type boolean.
        let Some(crate_dependency_workspace_value) = crate_dependency_workspace.as_bool() else {
          return Err(MaggError::new(format!("invalid dependency {key}.workspace attribute in crate '{}'", crate_to_publish.name)));
        };
        // Make sure the workspace dependency has value 'true'.
        if !crate_dependency_workspace_value {
          return Err(MaggError::new(format!(
            "dependency {key}.workspace attribute in crate '{}' must have value \"true\"",
            crate_to_publish.name
          )));
        }
        // Make sure, the line number of this crate is greater than the line number of the workspace dependency.
        if crate_to_publish.line_number <= dependency_crate_to_publish.line_number {
          return Err(MaggError::new(format!(
            "invalid publish order, crate '{}': {}, dependency '{}': {} ",
            crate_to_publish.name, crate_to_publish.line_number, dependency_crate_to_publish.name, dependency_crate_to_publish.line_number
          )));
        }
      }
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
  let exit_status = child.wait().map_err(|e| MaggError::new(e.to_string()))?;
  if !exit_status.success() {
    return Err(MaggError::new(format!("executing command failed with status code: {}", exit_status)));
  }
  Ok(())
}
