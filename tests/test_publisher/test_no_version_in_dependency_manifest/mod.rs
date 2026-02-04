#[test]
fn _0001() {
  cli_assert::command!()
    .code(1)
    .arg("publish")
    .arg("--simulation")
    .arg("--accept-all")
    .stdout("")
    .stderr("error: missing [package].version attribute in manifest for dependency 'cosmwasm-check'\n")
    .execute();
}
