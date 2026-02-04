#![doc = include_str!("../docs/PUBLISHER.md")]

use crate::errors::*;
use crate::utils;
use crate::workspace::load_workspace;
use antex::{StyledText, auto};
use std::ffi::OsStr;
use std::path::{Path, PathBuf};

/// A crate metadata.
#[derive(Default)]
struct CrateMetadata {
  /// Name of the crate in the workspace manifest to be published.
  name: String,
  /// Local path where the crate is defined.
  local_path: String,
  /// Search prefix in the original workspace manifest.
  prefix_with_local_path: String,
  /// Published crate dependency.
  prefix_with_version: String,
  /// Working directory for published crate.
  working_dir: PathBuf,
  /// Number of the line in the workspace manifest where the dependency is placed.
  line_number: usize,
  /// Padding after crate name for aligning columns.
  padding: String,
}

pub fn publish_crates(dir: &str, timeout: u64, accept_all: bool, simulation: bool) -> Result<()> {
  let working_dir = utils::canonicalize(Path::new(dir))?;

  let workspace = load_workspace(&working_dir)?;
  let workspace_manifest = &workspace.manifest;
  let publish_version = &workspace.version;
  let mut workspace_maifest_content = utils::read_file(&workspace_manifest)?;
  let workspace_manifest_toml = utils::parse_toml(&workspace_manifest)?;

  // Check if the manifest file is a workspace (required).
  let Some(workspace) = workspace_manifest_toml.get("workspace") else {
    return Err(MaggError::new("missing [workspace] section in the workspace manifest file"));
  };
  // // Check if the workspace manifest has a package section (required).
  // let Some(workspace_package) = workspace.get("package") else {
  //   return Err(MaggError::new("missing [workspace.package] section"));
  // };
  // // Check if the workspace manifest has defined the version to be published (required).
  // let Some(workspace_package_version) = workspace_package.get("version") else {
  //   return Err(MaggError::new("missing 'version' entry in [workspace.package] section"));
  // };
  // // Check if the version is a string (required).
  // let Some(publish_version) = workspace_package_version.as_str() else {
  //   return Err(MaggError::new("invalid 'version' entry in [workspace.package] section"));
  // };
  // Check if the workspace has dependencies section (required).
  let Some(workspace_dependencies) = workspace.get("dependencies") else {
    return Err(MaggError::new("missing [workspace.dependencies] section"));
  };
  // Check if dependencies section is a table (required).
  let Some(workspace_dependencies_table) = workspace_dependencies.as_table() else {
    return Err(MaggError::new("[workspace.dependencies] section is not a table"));
  };
  // let mut members_globs = vec![];
  // let mut exclude_globs = vec![];
  // // Check if the workspace has 'members' attribute (required).
  // let Some(workspace_members) = workspace.get("members") else {
  //   return Err(MaggError::new("missing 'members' entry in [workspace] section"));
  // };
  // // Check if the workspace 'members' is an array (required).
  // let Some(workspace_members_array) = workspace_members.as_array() else {
  //   return Err(MaggError::new("invalid 'members' entry [workspace] section"));
  // };
  // // Check if the workspace 'members' array contains only strings (required).
  // for workspace_member_value in workspace_members_array {
  //   let Some(workspace_member_string) = workspace_member_value.as_str() else {
  //     return Err(MaggError::new("invalid value in 'members' attribute in [workspace] section"));
  //   };
  //   members_globs.push(workspace_member_string);
  // }
  // // Check if the workspace has 'exclude' attribute (optional).
  // if let Some(workspace_exclude) = workspace.get("exclude") {
  //   // Check if the workspace 'exclude' is an array (required).
  //   let Some(workspace_exclude_array) = workspace_exclude.as_array() else {
  //     return Err(MaggError::new("invalid 'members' entry [workspace] section"));
  //   };
  //   // Check if the workspace 'exclude' array contains only strings (required).
  //   for workspace_exclude_value in workspace_exclude_array {
  //     let Some(workspace_exclude_string) = workspace_exclude_value.as_str() else {
  //       return Err(MaggError::new("invalid value in 'exclude' attribute in [workspace] section"));
  //     };
  //     exclude_globs.push(workspace_exclude_string);
  //   }
  // }
  // let _members = collect_members(file_name, &working_dir, members_globs, exclude_globs)?;

  //----------------------------------------------------------
  // Collect crates to publish from [workspace.dependencies]
  //----------------------------------------------------------

  let mut crates_to_publish = vec![];
  for (key, value) in workspace_dependencies_table {
    if value.get("path").is_some() {
      let name = key.to_string();
      if value.get("version").is_some() {
        return Err(MaggError::new(format!("dependency '{name}' must not have 'version' attribute set")));
      }
      let local_path = utils::strip_quotes(value["path"].as_str().unwrap()).to_string();
      let prefix_with_local_path = format!("{} = {{ path = \"{}\"", name, local_path);
      let Some(line_number) = utils::get_line_number(&workspace_maifest_content, &prefix_with_local_path) else {
        return Err(MaggError::new(format!("invalid formatting for dependency '{name}', expected '{}'", prefix_with_local_path)));
      };
      let prefix_with_version = format!("{} = {{ version = \"{}\"", name, &publish_version);
      let working_dir = utils::canonicalize(working_dir.join(Path::new(&local_path)))?;
      crates_to_publish.push(CrateMetadata {
        name,
        local_path,
        prefix_with_local_path,
        prefix_with_version,
        working_dir,
        line_number,
        ..Default::default()
      });
    }
  }
  if crates_to_publish.is_empty() {
    return Err(MaggError::new("no crates to publish"));
  }
  crates_to_publish.sort_by_key(|crate_to_publish| crate_to_publish.line_number);
  update_padding(&mut crates_to_publish);

  //---------------------
  // Validate crates
  //---------------------

  for crate_to_publish in &crates_to_publish {
    let name = crate_to_publish.name.to_string();
    let crate_manifest_file = utils::canonicalize(working_dir.join(Path::new(&crate_to_publish.local_path)).join(utils::RUST_MANIFEST_NAME))?;
    let crate_manifest_toml = utils::parse_toml(crate_manifest_file)?;
    let Some(crate_package) = crate_manifest_toml.get("package") else {
      return Err(MaggError::new(format!("missing [package] section in manifest for dependency '{name}'")));
    };
    let Some(crate_package_name) = crate_package.get("name") else {
      return Err(MaggError::new(format!("missing [package].name attribute in manifest for dependency '{name}'")));
    };
    let Some(package_name) = crate_package_name.as_str() else {
      return Err(MaggError::new(format!("invalid [package].name attribute in manifest for dependency '{name}'")));
    };
    if crate_to_publish.name != package_name {
      return Err(MaggError::new(format!(
        "expected 'name = \"{name}\"', actual 'name = \"{package_name}\"' in manifest for dependency '{name}'"
      )));
    }
    let Some(crate_package_version) = crate_package.get("version") else {
      return Err(MaggError::new(format!("missing [package].version attribute in manifest for dependency '{name}'")));
    };
    let Some(crate_package_version_workspace) = crate_package_version.get("workspace") else {
      return Err(MaggError::new(format!("missing [package].version.workspace attribute in manifest for dependency '{name}'")));
    };
    let Some(crate_package_version_workspace_value) = crate_package_version_workspace.as_bool() else {
      return Err(MaggError::new(format!("invalid [package].version.workspace attribute in manifest for dependency '{name}'")));
    };
    if !crate_package_version_workspace_value {
      return Err(MaggError::new(format!("[package].version.workspace attribute in crate '{name}' must have value 'true'")));
    }
    if let Some(dependencies) = crate_manifest_toml.get("dependencies") {
      let Some(dependencies_table) = dependencies.as_table() else {
        return Err(MaggError::new(format!("[dependencies] section is not a table in crate '{name}'")));
      };
      validate_crate_dependencies(dependencies_table, crate_to_publish, &crates_to_publish)?;
    }
    if let Some(dev_dependencies) = crate_manifest_toml.get("dev-dependencies") {
      let Some(dev_dependencies_table) = dev_dependencies.as_table() else {
        return Err(MaggError::new(format!("[dev-dependencies] section is not a table in crate '{name}'")));
      };
      validate_crate_dependencies(dev_dependencies_table, crate_to_publish, &crates_to_publish)?;
    }
  }

  //---------------------
  // Publish crates
  //---------------------

  // Ask if the version to be published is the right one.
  println!();
  println!("Publish version: {}", auto().bold().green().s(publish_version).clear());
  if !utils::ask_for_choice("Is this version correct?", accept_all)? {
    return Ok(());
  }

  // List all the crates to be published with versions and ask if the list is correct.
  println!();
  println!("Publish crates:");

  for crate_to_publish in &crates_to_publish {
    println!(
      "{}  {}  {}",
      auto().bold().blue().s(&crate_to_publish.name).clear().s(&crate_to_publish.padding),
      auto().bold().green().s('v').s(publish_version).clear(),
      crate_to_publish.working_dir.display()
    );
  }
  println!();
  if !utils::ask_for_choice("Do you want to publish all these crates?", accept_all)? {
    return Ok(());
  }

  for crate_to_publish in &crates_to_publish {
    // Ask if perform dry-run before publishing.
    println!(
      "\n{} {} {} {}",
      auto().bold().bg_yellow().s("  DRY-RUN  ").clear(),
      auto().bold().blue().s(&crate_to_publish.name).clear(),
      auto().bold().green().s('v').s(publish_version).clear(),
      crate_to_publish.working_dir.display()
    );
    if utils::ask_for_choice("Perform dry-run before publishing this crate?", accept_all)? {
      if simulation {
        execute_command("echo", ["simulating <dry-run>"], crate_to_publish.working_dir.clone())?;
      } else {
        execute_command("cargo", ["publish", "--dry-run", "--color=always"], crate_to_publish.working_dir.clone())?;
      }
    }
    // Ask if publish the crate.
    println!(
      "\n{} {} {} {}",
      auto().bold().bg_red().s("  PUBLISH  ").clear(),
      auto().bold().blue().s(&crate_to_publish.name).clear(),
      auto().bold().green().s('v').s(publish_version).clear(),
      crate_to_publish.working_dir.display()
    );
    if utils::ask_for_choice("Publish this crate?", accept_all)? {
      if simulation {
        execute_command("echo", ["simulating <publish>"], crate_to_publish.working_dir.clone())?;
      } else {
        execute_command("cargo", ["publish", "--color=always"], crate_to_publish.working_dir.clone())?;
      }
    }
    // Wait a timeout, just to make sure that the crate is fully published.
    if timeout > 0 {
      print!("Waiting {} second{} ", timeout, if timeout > 1 { "s" } else { "" });
    }
    for _ in 0..timeout {
      utils::step_progress();
      std::thread::sleep(if simulation && timeout == 1 {
        std::time::Duration::new(0, 1)
      } else {
        std::time::Duration::new(1, 0)
      });
    }
    if timeout > 0 {
      println!();
    }
    // After publishing the crate, replace 'path' with 'version'.
    workspace_maifest_content = workspace_maifest_content.replace(&crate_to_publish.prefix_with_local_path, &crate_to_publish.prefix_with_version);
    // Save the modified version of the workspace manifest,
    // so other crates will use the published versions of dependencies.
    utils::write_file(&workspace_manifest, &workspace_maifest_content)?;
  }
  Ok(())
}

fn validate_crate_dependencies(dependencies: &toml::Table, crate_to_publish: &CrateMetadata, crates_to_publish: &[CrateMetadata]) -> Result<()> {
  // Iterate over all dependencies.
  for (key, value) in dependencies {
    // Iterate over all crates to be published to check if this crate has them as dependencies.
    for dependency_crate_to_publish in crates_to_publish {
      if *key == dependency_crate_to_publish.name {
        // Make sure the dependency is defined in the workspace manifest.
        let Some(crate_dependency_workspace) = value.get("workspace") else {
          return Err(MaggError::new(format!("missing dependency {key}.workspace attribute in crate '{}'", crate_to_publish.name)));
        };
        // Make sure the workspace dependency is a boolean type.
        let Some(crate_dependency_workspace_value) = crate_dependency_workspace.as_bool() else {
          return Err(MaggError::new(format!("invalid dependency {key}.workspace attribute in crate '{}'", crate_to_publish.name)));
        };
        // Make sure the workspace dependency has value 'true'.
        if !crate_dependency_workspace_value {
          return Err(MaggError::new(format!(
            "dependency {key}.workspace attribute in crate '{}' must have value 'true'",
            crate_to_publish.name
          )));
        }
        // Make sure that the workspace dependency has no 'version' attribute set.
        if value.get("version").is_some() {
          return Err(MaggError::new(format!(
            "'{key}' dependency must not have 'version' attribute set in crate '{}'",
            crate_to_publish.name
          )));
        };
        // Make sure that the workspace dependency has no 'path' attribute set.
        if value.get("path").is_some() {
          return Err(MaggError::new(format!(
            "'{key}' dependency must not have 'path' attribute set in crate '{}'",
            crate_to_publish.name
          )));
        };
        // Make sure, the line number of this crate is greater than the line number of the workspace dependency.
        if crate_to_publish.line_number <= dependency_crate_to_publish.line_number {
          return Err(MaggError::new(format!(
            "invalid publish order, dependency '{}' must be published before crate '{}'",
            dependency_crate_to_publish.name, crate_to_publish.name,
          )));
        }
      }
    }
  }
  Ok(())
}

fn update_padding(crates_to_publish: &mut [CrateMetadata]) {
  let mut max_length = 0;
  for crate_to_publish in crates_to_publish.iter_mut() {
    if crate_to_publish.name.len() > max_length {
      max_length = crate_to_publish.name.len();
    }
  }
  for crate_to_publish in crates_to_publish.iter_mut() {
    crate_to_publish.padding = " ".repeat(max_length - crate_to_publish.name.len());
  }
}

/*
fn collect_members(file_name: &Path, working_dir: &Path, members_globs: Vec<&str>, exclude_globs: Vec<&str>) -> Result<Vec<CrateMetadata>> {
  let members = vec![];
  for members_glob in members_globs {
    let pattern = working_dir.join(members_glob).to_string_lossy().to_string();
    let paths = glob(&pattern).map_err(|e| MaggError::new(format!("failed to resolve glob '{}', reason {}", pattern, e)))?;
    for entry in paths {
      match entry {
        Ok(path) => {
          if path.is_dir() {
            let crate_manifest_file = path.join(file_name);
            if crate_manifest_file.exists() {
              let crate_manifest_toml = utils::parse_toml(&crate_manifest_file)?;
              println!("m={} {}", path.display(), crate_manifest_file.display());
            }
          }
        }
        Err(reason) => {
          return Err(MaggError::new(format!("failed to retrieve glob path, reason: {}", reason)));
        }
      }
    }
  }
  Ok(members)
}
*/

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
