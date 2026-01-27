#[test]
fn _0001() {
  cli_assert::command!()
    .code(0)
    .arg("publish")
    .arg("-f")
    .arg("Carqo.toml")
    .arg("--simulation")
    .arg("--accept-all")
    .stdout("")
    .stderr("ERROR: missing [workspace] section in the workspace manifest file\n")
    .execute();
}
