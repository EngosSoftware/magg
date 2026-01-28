const EXPECTED_STDERR: &str = r#"ERROR: TOML parse error at line 1, column 11
  |
1 | [workspace{{R}}
  |           ^
unclosed table, expected `]`

"#;

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
    .stderr(normalize_r(EXPECTED_STDERR))
    .execute();
}
