#[test]
fn short_file_name_option_should_work() {
  cli_assert::command!()
    .code(0)
    .arg("publish")
    .arg("-f Cargø.toml")
    .stdout("./Cargø.toml\n")
    .stderr("")
    .execute();
}
