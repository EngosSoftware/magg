#[test]
fn _0001() {
  // no arguments provided
  cli_assert::command!().code(0).stdout("").stderr("").execute();
}
