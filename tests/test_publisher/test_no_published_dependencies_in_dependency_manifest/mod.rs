use super::*;
use std::path::Path;

const EXPECTED_FILE: &str = r#"[workspace]

[workspace.package]
version = "1.0.0"

[workspace.dependencies]
cosmwasm-check = { version = "1.0.0" }
cosmwasm-vm = { version = "1.0.0" }
"#;

#[test]
fn _0001() {
  let dir = Path::new(file!()).parent().unwrap().canonicalize().unwrap();
  let original = dir.join(Path::new("Carqo.toml"));
  let backup = dir.join(Path::new("Carqo.bak.toml"));
  std::fs::copy(&original, &backup).unwrap();

  let dir = dir.display().to_string();
  let expected_stdout = format!(
    r#"
Publish version: 1.0.0

Publish crates:
cosmwasm-check v1.0.0 {dir}/check
cosmwasm-vm v1.0.0 {dir}/vm


Crate (dry-run):
  cosmwasm-check
  v1.0.0
  {dir}/check
simulating <dry-run>

Crate (publish):
  cosmwasm-check
  v1.0.0
  {dir}/check
simulating <publish>
Waiting 1 second(s), ·

Crate (dry-run):
  cosmwasm-vm
  v1.0.0
  {dir}/vm
simulating <dry-run>

Crate (publish):
  cosmwasm-vm
  v1.0.0
  {dir}/vm
simulating <publish>
Waiting 1 second(s), ·
"#
  );

  cli_assert::command!()
    .code(0)
    .arg("publish")
    .arg("-f")
    .arg("Carqo.toml")
    .arg("--simulation")
    .arg("--accept-all")
    .arg("--timeout")
    .arg("1")
    .stdout(normalize(&expected_stdout))
    .stderr("")
    .execute();
  assert_eq!(normalize(EXPECTED_FILE), std::fs::read_to_string(&original).unwrap());
  std::fs::copy(&backup, original).unwrap();
  std::fs::remove_file(backup).unwrap()
}
