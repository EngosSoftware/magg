use crate::licenses::{get_apache_2, get_apache_notice, get_mit};
use crate::{readme, utils};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const NAME: &str = env!("CARGO_PKG_NAME");
const COMMAND_README: &str = "readme";
const COMMAND_LICENSES: &str = "licenses";
const COMMAND_CODE_OF_CONDUCT: &str = "code-of-conduct";
const COMMAND_CHANGELOG: &str = "changelog";
const COMMAND_PUBLISH: &str = "publish";

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
      other => parse_command(args, other),
    }
  }
}

fn parse_command(args: Args, command: &str) {
  match command {
    COMMAND_README => parse_command_readme(args),
    COMMAND_LICENSES => parse_command_licenses(args),
    COMMAND_CODE_OF_CONDUCT => {
      println!("> code-of-conduct");
    }
    COMMAND_CHANGELOG => parse_command_changelog(args),
    COMMAND_PUBLISH => {
      println!("> publish");
    }
    other => suggest_command(other),
  }
}

fn parse_command_readme(mut args: Args) {
  if args.is_empty() {
    println!("error");
    do_help_readme();
  } else {
    match args.pop().unwrap().as_str() {
      "-h" | "--help" => action(args, do_help_readme),
      file_name => {
        action(args, || {
          do_command_readme(file_name);
        });
      }
    }
  }
}

fn parse_command_licenses(mut args: Args) {
  if args.is_empty() {
    do_command_licenses();
  } else {
    match args.pop().unwrap().as_str() {
      "-h" | "--help" => action(args, do_help_licenses),
      other => println!("unexpected argument: {other}"),
    }
  }
}

fn parse_command_changelog(mut args: Args) {
  if args.is_empty() {
    println!("error");
    do_help_changelog();
  } else {
    match args.pop().unwrap().as_str() {
      "-h" | "--help" => action(args, do_help_changelog),
      other => {
        args.push(other.to_string());
        for arg in args {
          match arg.as_str() {
            "-s" | "--start" => {}
            "-e" | "--end" => {}
            "-m" | "--milestone" => {}
            "-r" | "--repo" => {}
            "-d" | "--dir" => {}
            "--verbose" => {}
            _ => {}
          }
        }
      }
    }
  }
}

fn suggest_command(_other: &str) {
  //
}

fn action<F>(args: Args, action: F)
where
  F: Fn(),
{
  if args.is_empty() {
    action();
  } else {
    println!("unexpected argument: {}", args.last().unwrap())
  }
}

fn do_command_readme(file_name: &str) {
  let contents = readme::scaffold_readme(file_name);
  utils::write_file("README.md", &contents);
}

fn do_help_readme() {
  println!("> help readme");
}

/// Executes `licenses` subcommand.
fn do_command_licenses() {
  utils::write_file("LICENSE", &get_apache_2());
  utils::write_file("NOTICE", &get_apache_notice());
  utils::write_file("LICENSE-MIT", &get_mit());
}

fn do_help_licenses() {
  println!("> help licenses");
}

fn do_help_changelog() {
  println!("> help changelog");
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
