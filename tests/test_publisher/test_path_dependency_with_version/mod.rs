#[test]
fn _0001() {
  cli_assert::command!()
    .code(1)
    .arg("publish")
    .arg("--simulation")
    .arg("--accept-all")
    .stdout("")
    .stderr("error: dependency 'cosmwasm-check' must not have 'version' attribute set\n")
    .execute();
}
