use std::path::Path;

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
    .stderr(format!(
      "ERROR: failed to canonicalize path: {}/./check/Carqo.toml, reason: No such file or directory (os error 2)\n",
      Path::new(file!()).parent().unwrap().canonicalize().unwrap().display()
    ))
    .execute();
}
