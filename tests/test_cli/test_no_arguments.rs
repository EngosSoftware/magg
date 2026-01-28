#[test]
fn _0001() {
  cli_assert::command!().code(0).stdout("").stderr("").execute();
}
