use crate::errors::*;
use crate::utils;
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

  // Return the workspace metadata.
  Ok(Workspace {
    manifest: manifest_path,
    version: version.to_string(),
    dependencies,
  })
}
