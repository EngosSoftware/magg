use crate::errors::*;
use crate::utils;
use cargo_metadata::MetadataCommand;
use std::path::{Path, PathBuf};

/// Workspace metadata.
pub struct Workspace {
  /// Full path to the workspace manifest file.
  manifest: PathBuf,
  /// Version defined in `[workspace.package]` table.
  version: String,
  /// Workspace dependencies.
  dependencies: Vec<Dependency>,
}

impl Workspace {
  pub fn manifest(&self) -> &Path {
    &self.manifest
  }

  pub fn version(&self) -> &str {
    &self.version
  }

  pub fn dependencies(&self) -> &[Dependency] {
    &self.dependencies
  }
}

#[derive(Default)]
pub struct Dependency {
  /// Name of the dependency.
  pub name: String,
  /// Version attribute when present.
  pub version: Option<String>,
  /// Local path attribute when present.
  pub path: Option<String>,
}

#[derive(Default)]
pub struct Member {
  /// Package name of the crate.
  pub name: String,
}

pub fn load_workspace(working_dir: &Path) -> Result<Workspace> {
  let working_dir = utils::canonicalize(Path::new(working_dir))?;
  let manifest_path = utils::canonicalize(working_dir.join(utils::RUST_MANIFEST_NAME))?;
  let manifest_toml = utils::parse_toml(&manifest_path)?;
  // Check if the manifest file is a workspace (required).
  let Some(workspace) = manifest_toml.get("workspace") else {
    return Err(MaggError::new("missing [workspace] table"));
  };
  // Check if the workspace manifest has a package section (required).
  let Some(package) = workspace.get("package") else {
    return Err(MaggError::new("missing [workspace.package] table"));
  };
  // Check if the workspace manifest has defined the version to be published (required).
  let Some(version) = package.get("version") else {
    return Err(MaggError::new("missing 'version' in [workspace.package] table"));
  };
  // Check if the version is a string (required).
  let Some(version) = version.as_str() else {
    return Err(MaggError::new("'version' is not a string in [workspace.package] table"));
  };

  //---------------------------------------------------------------------------
  // Collect workspace dependencies
  //---------------------------------------------------------------------------

  // Check if the workspace has dependencies table (required).
  let Some(dependencies_table) = workspace.get("dependencies") else {
    return Err(MaggError::new("missing [workspace.dependencies] table"));
  };
  // Check if dependencies is a table (required).
  let Some(dependencies_table) = dependencies_table.as_table() else {
    return Err(MaggError::new("[workspace.dependencies] is not a table"));
  };
  let mut dependencies = vec![];
  for (name, value) in dependencies_table {
    let mut dependency = Dependency {
      name: name.to_string(),
      ..Default::default()
    };
    if let Some(path) = value.get("path") {
      let Some(path) = path.as_str() else {
        return Err(MaggError::new(format!("'path' is not a string for '{}' in [workspace.dependencies] table", name)));
      };
      dependency.path = Some(utils::strip_quotes(path).to_string());
    }
    if let Some(version) = value.get("version") {
      let Some(version) = version.as_str() else {
        return Err(MaggError::new(format!("'version' is not a string for '{}' in [workspace.dependencies] table", name)));
      };
      dependency.version = Some(version.to_string());
    }
    dependencies.push(dependency);
  }

  //---------------------------------------------------------------------------
  // Collect workspace members
  //---------------------------------------------------------------------------

  let mut members_globs = vec![];
  let mut exclude_globs = vec![];
  // Check if the workspace has 'members' attribute (required).
  let Some(workspace_members) = workspace.get("members") else {
    return Err(MaggError::new("missing 'members' entry in [workspace] section"));
  };
  // Check if the workspace 'members' is an array (required).
  let Some(workspace_members_array) = workspace_members.as_array() else {
    return Err(MaggError::new("invalid 'members' entry [workspace] section"));
  };
  // Check if the workspace 'members' array contains only strings (required).
  for workspace_member_value in workspace_members_array {
    let Some(workspace_member_string) = workspace_member_value.as_str() else {
      return Err(MaggError::new("invalid value in 'members' attribute in [workspace] section"));
    };
    members_globs.push(workspace_member_string);
  }
  // Check if the workspace has 'exclude' attribute (optional).
  if let Some(workspace_exclude) = workspace.get("exclude") {
    // Check if the workspace 'exclude' is an array (required).
    let Some(workspace_exclude_array) = workspace_exclude.as_array() else {
      return Err(MaggError::new("invalid 'members' entry [workspace] section"));
    };
    // Check if the workspace 'exclude' array contains only strings (required).
    for workspace_exclude_value in workspace_exclude_array {
      let Some(workspace_exclude_string) = workspace_exclude_value.as_str() else {
        return Err(MaggError::new("invalid value in 'exclude' attribute in [workspace] section"));
      };
      exclude_globs.push(workspace_exclude_string);
    }
  }
  //let _members = collect_members(&working_dir, members_globs)?;

  let mut cmd = MetadataCommand::new();
  cmd.manifest_path(&manifest_path);
  // let result = cmd.exec();
  // println!("DDD = {:?}", result);
  // for a in &result.workspace_members {
  //   println!("{}", a);
  // }

  // Return the workspace metadata.
  Ok(Workspace {
    manifest: manifest_path,
    version: version.to_string(),
    dependencies,
  })
}

// fn collect_members(working_dir: &Path, members_globs: Vec<&str>) -> Result<Vec<Member>> {
//   let members = vec![];
//   for members_glob in members_globs {
//     let pattern = working_dir.join(members_glob).to_string_lossy().to_string();
//     let paths = glob(&pattern).map_err(|e| MaggError::new(format!("failed to resolve glob '{}', reason {}", pattern, e)))?;
//     for entry in paths {
//       match entry {
//         Ok(path) => {
//           if path.is_dir() {
//             let crate_manifest_file = path.join(utils::RUST_MANIFEST_NAME);
//             if crate_manifest_file.exists() {
//               let crate_manifest_toml = utils::parse_toml(&crate_manifest_file)?;
//               //println!("m={} {}", path.display(), crate_manifest_file.display());
//             }
//           }
//         }
//         Err(reason) => {
//           return Err(MaggError::new(format!("failed to retrieve glob path, reason: {}", reason)));
//         }
//       }
//     }
//   }
//   Ok(members)
// }

// fn paths(working_dir: &Path, members_globs: Vec<&str>) -> Result<Vec<PathBuf>> {
//   let members = vec![];
//   for members_glob in members_globs {
//     let pattern = working_dir.join(members_glob).to_string_lossy().to_string();
//     let paths = glob(&pattern).map_err(|e| MaggError::new(format!("failed to resolve glob '{}', reason {}", pattern, e)))?;
//     for entry in paths {
//       match entry {
//         Ok(path) => {
//           if path.is_dir() {
//             let crate_manifest_file = path.join(utils::RUST_MANIFEST_NAME);
//             if crate_manifest_file.exists() {
//               let crate_manifest_toml = utils::parse_toml(&crate_manifest_file)?;
//               //println!("m={} {}", path.display(), crate_manifest_file.display());
//             }
//           }
//         }
//         Err(reason) => {
//           return Err(MaggError::new(format!("failed to retrieve glob path, reason: {}", reason)));
//         }
//       }
//     }
//   }
//   Ok(members)
// }
