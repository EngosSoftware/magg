#[test]
fn short() {
  let expected = r#"Sophisticated tooling for Rust developers

Usage: magg [COMMAND]

Commands:
  readme           Generates README.md file
  licenses         Generates MIT and Apache 2.0 license files
  code-of-conduct  Generates code of conduct file
  changelog        Generates changelog
  publish          Publish Rust crates
  help             Print this message or the help of the given subcommand(s)

Options:
  -V, --version  Print version
  -h, --help     Print help
"#;
  cli_assert::command!().arg("-h").code(0).stdout(expected).stderr("").execute();
}
