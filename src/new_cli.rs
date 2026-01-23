use crate::licenses::{get_apache_2, get_apache_notice, get_mit};
use crate::{readme, utils};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const SUBCOMMAND_README: &str = "readme";
const SUBCOMMAND_LICENSES: &str = "licenses";
const SUBCOMMAND_CODE_OF_CONDUCT: &str = "code-of-conduct";
const SUBCOMMAND_CHANGELOG: &str = "changelog";
const SUBCOMMAND_PUBLISH: &str = "publish";

type Args = Vec<String>;

pub fn new_do_action() {
  let mut args: Args = std::env::args().skip(1).rev().collect::<Vec<String>>();
  if args.is_empty() {
    do_help();
  } else {
    match args.pop().unwrap().as_str() {
      "-h" => action(args, do_help_short),
      "--help" => action(args, do_help_long),
      "-v" | "--version" => action(args, do_version),
      other => parse_subcommand(args, other),
    }
  }
}

fn parse_subcommand(args: Args, subcommand: &str) {
  match subcommand {
    SUBCOMMAND_README => parse_subcommand_readme(args),
    SUBCOMMAND_LICENSES => parse_subcommand_licenses(args),
    SUBCOMMAND_CODE_OF_CONDUCT => {
      println!("> code-of-conduct");
    }
    SUBCOMMAND_CHANGELOG => {
      println!("> changelog");
    }
    SUBCOMMAND_PUBLISH => {
      println!("> publish");
    }
    other => suggest_subcommand(other),
  }
}

fn parse_subcommand_readme(mut args: Args) {
  if args.is_empty() {
    println!("error");
    do_help_readme();
  } else {
    match args.pop().unwrap().as_str() {
      "-h" | "--help" => action(args, do_help_readme),
      file_name => {
        if args.is_empty() {
          do_subcommand_readme(file_name)
        } else {
          println!("unexpected argument: {}", args.last().unwrap())
        }
      }
    }
  }
}

fn parse_subcommand_licenses(args: Args) {
  action_or_help(args, do_subcommand_licenses, do_help_licenses);
}

fn suggest_subcommand(_other: &str) {
  //
}

fn action(args: Args, action: fn()) {
  if args.is_empty() {
    action();
  } else {
    println!("unexpected argument: {}", args.last().unwrap())
  }
}

fn action_or_help(mut args: Args, fn_action: fn(), fn_help: fn()) {
  if args.is_empty() {
    fn_action();
  } else {
    match args.pop().unwrap().as_str() {
      "-h" | "--help" => action(args, fn_help),
      other => println!("unexpected argument: {other}"),
    }
  }
}

fn do_subcommand_readme(file_name: &str) {
  let contents = readme::scaffold_readme(file_name);
  utils::write_file("README.md", &contents);
}

fn do_help_readme() {
  println!("> help readme");
}

/// Executes `licenses` subcommand.
fn do_subcommand_licenses() {
  utils::write_file("LICENSE", &get_apache_2());
  utils::write_file("NOTICE", &get_apache_notice());
  utils::write_file("LICENSE-MIT", &get_mit());
}

fn do_help_licenses() {
  println!("> help licenses");
}

fn do_version() {
  println!("{} {}", NAME, VERSION);
}

fn do_help() {
  println!("> help general");
}

fn do_help_short() {
  println!("> help short");
}

fn do_help_long() {
  println!("> help long");
}
