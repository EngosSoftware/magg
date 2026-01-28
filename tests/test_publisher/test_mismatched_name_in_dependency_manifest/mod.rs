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
    .stderr("error: expected 'name = \"cosmwasm-check\"', actual 'name = \"check\"' in manifest for dependency 'cosmwasm-check'\n")
    .execute();
}
