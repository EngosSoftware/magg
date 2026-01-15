use crate::{readme, utils};
use clap::{Arg, ArgAction, ArgMatches, Command, arg, command, crate_version};

enum Action {
  Readme(
    /// Name of the file containing the body text of scaffolded README.md file.
    String,
  ),
  Nothing,
}

/// Parses CLI argument matches.
fn get_matches() -> ArgMatches {
  command!()
    // disable the built-in version flag
    .disable_version_flag(true)
    // handle the version flag in a custom way
    .arg(Arg::new("version").short('V').long("version").help("Print version").action(ArgAction::SetTrue))
    // pfe - Parse FEEL Expression
    .subcommand(
      Command::new("readme")
        .about("Scaffolds README.md file")
        .display_order(1)
        .arg(arg!(<README_BODY>).help("File containing the body of the scaffolded README.md").required(true).index(1)),
    )
    .get_matches()
}

/// Checks the list of arguments passed from the command line
/// and returns an action related to a valid argument.
fn get_cli_action() -> Action {
  let matches = get_matches();
  // Replaces the built-in version flag with the custom handler.
  if matches.get_flag("version") {
    // Displays only the version number, without the name of the crate.
    println!("{}", crate_version!());
    return Action::Nothing;
  }
  match matches.subcommand() {
    Some(("readme", matches)) => {
      return Action::Readme(match_string(matches, "README_BODY"));
    }
    _ => {}
  }
  Action::Nothing
}

pub fn do_action() {
  //
  match get_cli_action() {
    Action::Readme(file_name) => {
      let contents = readme::scaffold_readme(file_name);
      utils::write_file("README.md", &contents);
    }
    Action::Nothing => {
      // No specific action was requested.
    }
  }
}

/// Matches a mandatory string argument.
fn match_string(matches: &ArgMatches, name: &str) -> String {
  matches.get_one::<String>(name).unwrap().to_string()
}
