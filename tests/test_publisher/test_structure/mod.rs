#[test]
fn _0001() {
  cli_assert::command!()
    .code(0)
    .arg("publish")
    .arg("-d")
    .arg("project_1")
    .arg("-f")
    .arg("Cargq.toml")
    .stdout("project_1/Cargq.toml\n")
    .stderr("")
    .execute();
}
