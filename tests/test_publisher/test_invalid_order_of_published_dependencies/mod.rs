#[test]
fn _0001() {
  cli_assert::command!()
    .code(1)
    .arg("publish")
    .arg("--file-name")
    .arg("Carqo.toml")
    .arg("--simulation")
    .arg("--accept-all")
    .stdout("")
    .stderr("error: invalid publish order, dependency 'cosmwasm-vm' must be published before crate 'cosmwasm-check'\n")
    .execute();
}
