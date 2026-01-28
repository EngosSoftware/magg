use std::path::Path;

const EXPECTED: &str = r#"[workspace]
members = ["packages/*"]
exclude = ["contracts"]

resolver = "2"

[workspace.package]
version = "3.0.0"

[workspace.dependencies]
cosmwasm-core = { version = "3.0.0" }
cosmwasm-crypto = { version = "3.0.0" }
cosmwasm-derive = { version = "3.0.0" }
cosmwasm-schema = { version = "3.0.0" }
cosmwasm-schema-derive = { version = "3.0.0" }
cosmwasm-std = { version = "3.0.0", default-features = false }
cosmwasm-vm = { version = "3.0.0" }
cosmwasm-vm-derive = { version = "3.0.0" }
cw-schema = { version = "3.0.0" }
cw-schema-derive = { version = "3.0.0" }
cosmwasm-check = { version = "3.0.0" }

schemars = "0.8.4"
serde = { version = "1.0.192", default-features = false, features = ["alloc", "derive"] }
serde_json = "1.0.140"
thiserror = "1.0.26"
"#;

#[test]
fn _0001() {
  let dir = Path::new(file!()).parent().unwrap();
  let original = dir.join(Path::new("project_1/Carqo.toml"));
  let backup = dir.join(Path::new("project_1/Carqo.bak.toml"));
  std::fs::copy(&original, &backup).unwrap();
  cli_assert::command!()
    .code(0)
    .arg("publish")
    .arg("-d")
    .arg("project_1")
    .arg("-f")
    .arg("Carqo.toml")
    .arg("--simulation")
    .arg("--accept-all")
    .stderr("")
    .execute();
  assert_eq!(EXPECTED, std::fs::read_to_string(&original).unwrap());
  std::fs::copy(&backup, original).unwrap();
  std::fs::remove_file(backup).unwrap()
}
