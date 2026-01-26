use crate::changelog;
use crate::code_of_conduct::get_code_of_conduct;
use crate::licenses::{get_apache_2, get_apache_notice, get_mit};
use crate::publisher;
use crate::utils::{RUST_MANIFEST_FILE_NAME, SEPARATOR_LINE};
use crate::{readme, utils};
use clap::{Arg, ArgAction, ArgMatches, Command, arg, command, crate_version};
use std::ffi::{OsStr, OsString};

/// Default timeout in seconds.
const DEFAULT_TIMEOUT: u64 = 30;
const DEFAULT_TIMEOUT_STR: &str = stringify!(DEFAULT_TIMEOUT);

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
    /// Start revision.
    String,
    /// End revision.
    String,
    /// Milestone.
    String,
    /// Organization/Repository name.
    String,
    /// Current directory.
    String,
    /// Verbose flag.
    bool,
  ),
  Publish(
    /// Name of the configuration and manifest file for Rust projects (default: Cargo.toml)
    String,
    /// Path to the workspace manifest file.
    String,
    /// Number of seconds to wait after publishing a crate.
    u64,
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
    .subcommand(
      Command::new("changelog")
        .about("Generates changelog")
        .display_order(4)
        .arg(
          Arg::new("start-revision")
            .short('s')
            .long("start")
            .help("Start revision for searching commits")
            .action(ArgAction::Set)
            .required(true)
            .display_order(1),
        )
        .arg(
          Arg::new("end-revision")
            .short('e')
            .long("end")
            .help("End revision for searching commits")
            .action(ArgAction::Set)
            .required(true)
            .display_order(2),
        )
        .arg(
          Arg::new("milestone")
            .short('m')
            .long("milestone")
            .help("GitHub milestone name for searching issues and pull requests")
            .action(ArgAction::Set)
            .required(true)
            .display_order(3),
        )
        .arg(
          Arg::new("repository")
            .short('r')
            .long("repo")
            .help("GitHub organization/repository name for searching issues and pull requests")
            .action(ArgAction::Set)
            .required(true)
            .display_order(4),
        )
        .arg(
          Arg::new("directory")
            .short('d')
            .long("dir")
            .help("Directory of a Git repository for searching commits")
            .action(ArgAction::Set)
            .default_value(".")
            .default_missing_value(".")
            .num_args(0..=1)
            .display_order(5),
        )
        .arg(
          Arg::new("verbose")
            .long("verbose")
            .help("Set this flag to display more detailed report")
            .action(ArgAction::SetTrue)
            .default_value("false")
            .default_missing_value("true")
            .display_order(6),
        ),
    )
    .subcommand(
      Command::new("publish")
        .about("Publishes Rust crates")
        .display_order(5)
        .arg(
          Arg::new("file-name")
            .short('f')
            .long("file-name")
            .help("Name of the Rust manifest file")
            .default_value(RUST_MANIFEST_FILE_NAME)
            .num_args(1)
            .action(ArgAction::Set)
            .display_order(1),
        )
        .arg(
          Arg::new("dir")
            .short('d')
            .long("dir")
            .help("Directory where the workspace manifest file is placed")
            .default_value(".")
            .num_args(1)
            .action(ArgAction::Set)
            .display_order(2),
        )
        .arg(
          Arg::new("timeout")
            .short('t')
            .long("timeout")
            .help("Number of seconds to wait after publishing a crate")
            .default_value(DEFAULT_TIMEOUT_STR)
            .num_args(1)
            .action(ArgAction::Set)
            .display_order(3),
        ),
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
    Some(("licenses", _matches)) => {
      return Action::Licenses;
    }
    Some(("code-of-conduct", _matches)) => {
      return Action::CodeOfConduct;
    }
    Some(("changelog", matches)) => {
      let start_revision = match_string(matches, "start-revision");
      let end_revision = match_string(matches, "end-revision");
      let milestone = match_string(matches, "milestone");
      let repository = match_string(matches, "repository");
      let dir = match_string(matches, "directory");
      let verbose = match_boolean(matches, "verbose");
      return Action::Changelog(start_revision, end_revision, milestone, repository, dir, verbose);
    }
    Some(("publish", matches)) => {
      let dir = match_string(matches, "dir");
      let file_name = match_string(matches, "file-name");
      let timeout = match_string(matches, "timeout").parse::<u64>().unwrap_or(DEFAULT_TIMEOUT).clamp(0, 60);
      return Action::Publish(file_name, dir, timeout);
    }
    _ => {}
  }
  Action::Nothing
}

pub fn do_action() {
  //
  match get_cli_action() {
    Action::Readme(file_name) => match readme::scaffold_readme(file_name) {
      Ok(contents) => {
        utils::write_file("README.md", &contents);
      }
      Err(reason) => {
        eprintln!("ERROR: {}", reason)
      }
    },
    Action::Licenses => {
      utils::write_file("LICENSE", &get_apache_2());
      utils::write_file("NOTICE", &get_apache_notice());
      utils::write_file("LICENSE-MIT", &get_mit());
    }
    Action::CodeOfConduct => {
      utils::write_file("CODE_OF_CONDUCT.md", &get_code_of_conduct());
    }
    Action::Changelog(start_revision, end_revision, milestone, repository, dir, verbose) => {
      match changelog::get_changelog(verbose, &start_revision, &end_revision, &milestone, &repository, &dir) {
        Ok(changelog) => {
          println!("\nCHANGELOG");
          println!("{SEPARATOR_LINE}");
          println!("{}", changelog)
        }
        Err(reason) => {
          eprintln!("ERROR: {}", reason)
        }
      }
    }
    Action::Publish(file_name, dir, timeout) => {
      //
      match publisher::publish_crates(&file_name, &dir, timeout) {
        Ok(()) => {}
        Err(reason) => {
          eprintln!("ERROR: {}", reason)
        }
      }
    }
    Action::Nothing => {
      // No specific action was requested.
    }
  }
}

/// Matches a mandatory string argument.
fn match_string(matches: &ArgMatches, name: &str) -> String {
  matches.get_one::<String>(name).unwrap().trim().to_string()
}

/// Matches a mandatory boolean argument.
fn match_boolean(matches: &ArgMatches, name: &str) -> bool {
  matches.get_flag(name)
}
