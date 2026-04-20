use super::*;

#[test]
fn short() {
  let expected = r#"Sophisticated tooling for Rust developers

Usage: magg||EXE|| [COMMAND]

Commands:
  readme           Generates README file for regular crate
  readme-dt        Generates README file for crate in ÐecisionToolkit project
  licenses         Generates MIT and Apache 2.0 license files
  code-of-conduct  Generates code of conduct file
  workflows        Generates GitHub workflows
  changelog        Generates changelog
  project-report   Generates project report
  help             Print this message or the help of the given subcommand(s)

Options:
  -V, --version  Print version
  -h, --help     Print help
"#;
  cli_assert::command!().arg("-h").code(0).stdout(normalize_exe(expected)).stderr("").execute();
}
