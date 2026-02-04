#[test]
fn _0001() {
  cli_assert::command!()
    .code(1)
    .arg("publish")
    .arg("--simulation")
    .arg("--accept-all")
    .stdout("")
    .stderr("error: invalid 'version' entry in [workspace.package] section\n")
    .execute();
}
