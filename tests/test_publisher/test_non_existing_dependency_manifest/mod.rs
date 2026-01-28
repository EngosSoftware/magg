use std::path::Path;

#[test]
fn _0001() {
  #[cfg(not(target_os = "windows"))]
  let expected_stderr = format!(
    "ERROR: failed to canonicalize path: {}/./check/Carqo.toml, reason: No such file or directory (os error 2)\n",
    Path::new(file!()).parent().unwrap().canonicalize().unwrap().display()
  );

  #[cfg(target_os = "windows")]
  let expected_stderr = format!(
    "ERROR: failed to canonicalize path: {}\\check\\Carqo.toml, reason: The system cannot find the file specified. (os error 2)\n",
    Path::new(file!()).parent().unwrap().canonicalize().unwrap().display()
  );

  cli_assert::command!()
    .code(1)
    .arg("publish")
    .arg("-f")
    .arg("Carqo.toml")
    .arg("--simulation")
    .arg("--accept-all")
    .stdout("")
    .stderr(expected_stderr)
    .execute();
}
