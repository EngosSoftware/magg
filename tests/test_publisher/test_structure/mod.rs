#[test]
fn _0001() {
  cli_assert::command!()
    .code(0)
    .arg("publish")
    .arg("-d")
    .arg("project_1")
    .arg("-f")
    .arg("Cargø.toml")
    .stdout("project_1/Cargø.toml\n")
    .stderr("")
    .execute();
}
