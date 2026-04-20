use crate::code_of_conduct::get_code_of_conduct;
use crate::errors::*;
use crate::licenses::{get_apache_2, get_apache_notice, get_mit};
use crate::project_report::get_project_report;
use crate::utils::SEPARATOR_LINE;
use crate::{changelog, utils};
use crate::{readme, workflows};
use antex::{StyledText, Text, auto};
use clap::{Arg, ArgAction, ArgMatches, Command, command, crate_version};

enum Action {
  /// Generate README file for regular crate.
  ReadmeForRegularCrate(
    /// Name of the file containing the body text of README file.
    String,
    /// Scaffolded README file name.
    String,
  ),
  /// Generate README file for crate in ÐecisionToolkit project.
  ReadmeForDecisionToolkitCrate(
    /// Name of the file containing the body text of README file.
    String,
    /// Scaffolded README file name.
    String,
  ),
  /// Generate license files.
  Licenses,
  /// Generate code of conduct file.
  CodeOfConduct,
  /// Generate workflows files.
  Workflows,
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
    /// String patterns for excluding commits by subject.
    Vec<String>,
    /// String patterns for excluding pull requests by title.
    Vec<String>,
  ),
  ProjectReport(
    /// Owner name of the project.
    String,
    /// Project name.
    String,
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
    // Generate README file for regular crate.
    .subcommand(
      Command::new("readme")
        .about("Generates README file for regular crate")
        .display_order(1)
        .arg(
          Arg::new("input-file")
            .short('f')
            .long("input-file")
            .help("File containing the body of the scaffolded README")
            .action(ArgAction::Set)
            .default_value("docs/README.md")
            .display_order(1),
        )
        .arg(
          Arg::new("output-file")
            .short('o')
            .long("output-file")
            .help("Scaffolded README file name")
            .action(ArgAction::Set)
            .default_value("README.md")
            .display_order(1),
        ),
    )
    // Generate README file for a crate in ÐecisionToolkit project.
    .subcommand(
      Command::new("readme-dt")
        .about("Generates README file for crate in ÐecisionToolkit project")
        .display_order(2)
        .arg(
          Arg::new("input-file")
            .short('f')
            .long("input-file")
            .help("File containing the body of the scaffolded README")
            .action(ArgAction::Set)
            .default_value("docs/README.md")
            .display_order(1),
        )
        .arg(
          Arg::new("output-file")
            .short('o')
            .long("output-file")
            .help("Scaffolded README file name")
            .action(ArgAction::Set)
            .default_value("README.md")
            .display_order(1),
        ),
    )
    .subcommand(Command::new("licenses").about("Generates MIT and Apache 2.0 license files").display_order(3))
    .subcommand(Command::new("code-of-conduct").about("Generates code of conduct file").display_order(4))
    .subcommand(Command::new("workflows").about("Generates GitHub workflows").display_order(5))
    .subcommand(
      Command::new("changelog")
        .about("Generates changelog")
        .display_order(6)
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
        )
        .arg(
          Arg::new("exclude-commit")
            .long("exclude-commit")
            .help("Exclude commits that contain this text in subject")
            .action(ArgAction::Append)
            .display_order(7),
        )
        .arg(
          Arg::new("exclude-pr")
            .long("exclude-pr")
            .help("Exclude pull requests that contain this text in title")
            .action(ArgAction::Append)
            .display_order(8),
        ),
    )
    .subcommand(
      Command::new("project-report")
        .about("Generates project report")
        .display_order(7)
        .arg(
          Arg::new("owner")
            .short('o')
            .long("owner")
            .help("Project owner")
            .action(ArgAction::Set)
            .required(true)
            .display_order(1),
        )
        .arg(
          Arg::new("name")
            .short('n')
            .long("name")
            .help("Project name")
            .action(ArgAction::Set)
            .required(true)
            .display_order(2),
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
      let input_file = match_string(matches, "input-file");
      let output_file = match_string(matches, "output-file");
      return Action::ReadmeForRegularCrate(input_file, output_file);
    }
    Some(("readme-dt", matches)) => {
      let input_file = match_string(matches, "input-file");
      let output_file = match_string(matches, "output-file");
      return Action::ReadmeForDecisionToolkitCrate(input_file, output_file);
    }
    Some(("licenses", _matches)) => {
      return Action::Licenses;
    }
    Some(("code-of-conduct", _matches)) => {
      return Action::CodeOfConduct;
    }
    Some(("workflows", _matches)) => {
      return Action::Workflows;
    }
    Some(("changelog", matches)) => {
      let start_revision = match_string(matches, "start-revision");
      let end_revision = match_string(matches, "end-revision");
      let milestone = match_string(matches, "milestone");
      let repository = match_string(matches, "repository");
      let dir = match_string(matches, "directory");
      let verbose = match_boolean(matches, "verbose");
      let exclude_commit = match_strings(matches, "exclude-commit");
      let exclude_pr = match_strings(matches, "exclude-pr");
      return Action::Changelog(start_revision, end_revision, milestone, repository, dir, verbose, exclude_commit, exclude_pr);
    }
    Some(("project-report", matches)) => {
      let project_owner = match_string(matches, "owner");
      let project_name = match_string(matches, "name");
      return Action::ProjectReport(project_owner, project_name);
    }
    _ => {}
  }
  Action::Nothing
}

pub fn do_action() {
  fn error_message(reason: MaggError) -> Text {
    auto().bold().red().s("error").reset().s(": ").s(reason.to_string())
  }

  match get_cli_action() {
    Action::ReadmeForRegularCrate(input_file, output_file) => match readme::get_readme_for_regular_crate(input_file) {
      Ok(contents) => {
        utils::write_file(output_file, &contents).unwrap();
      }
      Err(reason) => {
        eprintln!("{}", error_message(reason));
        std::process::exit(1);
      }
    },
    Action::ReadmeForDecisionToolkitCrate(input_file, output_file) => match readme::get_readme_for_decision_toolkit_crate(input_file) {
      Ok(contents) => {
        utils::write_file(output_file, &contents).unwrap();
      }
      Err(reason) => {
        eprintln!("{}", error_message(reason));
        std::process::exit(1);
      }
    },
    Action::Licenses => {
      utils::write_file("LICENSE", &get_apache_2()).unwrap();
      utils::write_file("NOTICE", &get_apache_notice()).unwrap();
      utils::write_file("LICENSE-MIT", &get_mit()).unwrap();
    }
    Action::CodeOfConduct => {
      utils::write_file("CODE_OF_CONDUCT.md", &get_code_of_conduct()).unwrap();
    }
    Action::Workflows => {
      utils::write_file(".github/workflows/build-linux-gnu.yml", &workflows::get_build_linux_gnu()).unwrap();
      utils::write_file(".github/workflows/build-linux-musl.yml", &workflows::get_build_linux_musl()).unwrap();
      utils::write_file(".github/workflows/build-macos.yml", &workflows::get_build_macos()).unwrap();
      utils::write_file(".github/workflows/build-macos-aarch64.yml", &workflows::get_build_macos_aarch64()).unwrap();
      utils::write_file(".github/workflows/build-windows.yml", &workflows::get_build_windows()).unwrap();
    }
    Action::Changelog(start_revision, end_revision, milestone, repository, dir, verbose, exclude_commit, exclude_pr) => {
      match changelog::get_changelog(verbose, &start_revision, &end_revision, &milestone, &repository, &dir, exclude_commit, exclude_pr) {
        Ok(changelog) => {
          println!("\nCHANGELOG");
          println!("{SEPARATOR_LINE}");
          println!("{}", changelog)
        }
        Err(reason) => {
          eprintln!("\n{}", error_message(reason));
          std::process::exit(1);
        }
      }
    }
    Action::ProjectReport(project_owner, project_name) => {
      // Generates a report of the project.
      match get_project_report(&project_owner, &project_name) {
        Ok(report) => {
          println!("\n{}", report)
        }
        Err(reason) => {
          eprintln!("\n{}", error_message(reason));
          std::process::exit(1);
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

/// Matches an optional repeatable string argument.
fn match_strings(matches: &ArgMatches, name: &str) -> Vec<String> {
  matches.get_many(name).unwrap_or_default().cloned().collect()
}
