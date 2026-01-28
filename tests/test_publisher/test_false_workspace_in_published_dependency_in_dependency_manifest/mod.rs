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
    .stderr("ERROR: dependency cosmwasm-vm.workspace attribute in crate 'cosmwasm-check' must have value 'true'\n")
    .execute();
}
