#![doc = include_str!("../docs/PUBLISHER.md")]

#[cfg(test)]
mod tests {
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
}
