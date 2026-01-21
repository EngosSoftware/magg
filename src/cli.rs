use crate::changelog::get_changelog;
use crate::code_of_conduct::get_code_of_conduct;
use crate::licenses::{get_apache_2, get_apache_notice, get_mit};
use crate::{readme, utils};
use clap::{Arg, ArgAction, ArgMatches, Command, arg, command, crate_version};

enum Action {
  /// Generate README.md file
  Readme(
    /// Name of the file containing the body text of scaffolded README.md file.
    String,
  ),
  /// Generate license files.
  Licenses,
  /// Generate code of conduct file.
  CodeOfConduct,
  /// Generate changelog.
  Changelog(
    //
  ),
  /// Do nothing.
  Nothing,
}

/// Parses CLI argument matches.
fn get_matches() -> ArgMatches {
  command!()
    // disable the built-in version flag
    .disable_version_flag(true)
    // handle the version flag in a custom way
    .arg(Arg::new("version").short('V').long("version").help("Print version").action(ArgAction::SetTrue))
    // Generate README.md file.
    .subcommand(
      Command::new("readme")
        .about("Generates README.md file")
        .display_order(1)
        .arg(arg!(<README_BODY>).help("File containing the body of the scaffolded README.md").required(true).index(1)),
    )
    .subcommand(Command::new("licenses").about("Generates MIT and Apache 2.0 license files").display_order(2))
    .subcommand(Command::new("code-of-conduct").about("Generates code of conduct file").display_order(3))
    .subcommand(Command::new("changelog").about("Generates changelog").display_order(4))
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
    Some(("licenses", _matches)) => {
      return Action::Licenses;
    }
    Some(("code-of-conduct", _matches)) => {
      return Action::CodeOfConduct;
    }
    Some(("changelog", _matches)) => {
      return Action::Changelog();
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
    Action::Licenses => {
      utils::write_file("LICENSE", &get_apache_2());
      utils::write_file("NOTICE", &get_apache_notice());
      utils::write_file("LICENSE-MIT", &get_mit());
    }
    Action::CodeOfConduct => {
      utils::write_file("CODE_OF_CONDUCT.md", &get_code_of_conduct());
    }
    Action::Changelog() => match get_changelog() {
      Ok(changelog) => {
        println!("{}", changelog)
      }
      Err(reason) => {
        eprintln!("ERROR: {}", reason)
      }
    },
    Action::Nothing => {
      // No specific action was requested.
    }
  }
}

/// Matches a mandatory string argument.
fn match_string(matches: &ArgMatches, name: &str) -> String {
  matches.get_one::<String>(name).unwrap().to_string()
}
