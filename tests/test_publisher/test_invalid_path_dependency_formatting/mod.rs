#[test]
fn _0001() {
  cli_assert::command!()
    .code(1)
    .arg("publish")
    .arg("-f")
    .arg("Carqo.toml")
    .arg("--simulation")
    .arg("--accept-all")
    .stdout("")
    .stderr("error: invalid formatting for dependency 'cosmwasm-check', expected 'cosmwasm-check = { path = \"./packages/check\"'\n")
    .execute();
}
