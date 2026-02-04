use super::*;
use std::path::Path;

const EXPECTED_FILE: &str = r#"[workspace]
members = ["check", "vm"]

[workspace.package]
version = "1.0.0"

[workspace.dependencies]
cosmwasm-check = { version = "1.0.0" }
cosmwasm-vm = { version = "1.0.0" }
"#;

#[test]
fn _0001() {
  let dir = Path::new(file!()).parent().unwrap().canonicalize().unwrap();
  let original = dir.join(Path::new("Cargo.toml"));
  let backup = dir.join(Path::new("Cargo.bak.toml"));
  std::fs::copy(&original, &backup).unwrap();

  let dir = dir.display().to_string();
  let expected_stdout = format!(
    r#"
Publish version: 1.0.0

Publish crates:
cosmwasm-check  v1.0.0  {dir}||PATH-SEPARATOR||check
cosmwasm-vm     v1.0.0  {dir}||PATH-SEPARATOR||vm


  DRY-RUN   cosmwasm-check v1.0.0 {dir}||PATH-SEPARATOR||check
simulating <dry-run>

  PUBLISH   cosmwasm-check v1.0.0 {dir}||PATH-SEPARATOR||check
simulating <publish>
Waiting 1 second ·

  DRY-RUN   cosmwasm-vm v1.0.0 {dir}||PATH-SEPARATOR||vm
simulating <dry-run>

  PUBLISH   cosmwasm-vm v1.0.0 {dir}||PATH-SEPARATOR||vm
simulating <publish>
Waiting 1 second ·
"#
  );

  cli_assert::command!()
    .code(0)
    .arg("publish")
    .arg("--simulation")
    .arg("--accept-all")
    .arg("--timeout")
    .arg("1")
    .stdout(normalize_path_separator(&expected_stdout))
    .stderr("")
    .execute();
  assert_eq!(normalize(EXPECTED_FILE), std::fs::read_to_string(&original).unwrap());
  std::fs::copy(&backup, original).unwrap();
  std::fs::remove_file(backup).unwrap()
}
