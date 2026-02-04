#[test]
fn _0001() {
  cli_assert::command!()
    .code(1)
    .arg("publish")
    .arg("--simulation")
    .arg("--accept-all")
    .stdout("")
    .stderr("error: 'cosmwasm-vm' dependency must not have 'path' attribute set in crate 'cosmwasm-check'\n")
    .execute();
}
