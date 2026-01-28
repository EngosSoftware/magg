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
    .stderr("ERROR: 'cosmwasm-vm' dependency must not have 'path' attribute set in crate 'cosmwasm-check'\n")
    .execute();
}
