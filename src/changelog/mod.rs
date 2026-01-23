//! # Changelog generator
//!
//! Generates kind of reasonable changelog based on Git commits between revisions,
//! GitHub pull requests and GitHub issues belonging to the same milestone.

use crate::errors::*;
use crate::utils::SEPARATOR_LINE;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Write;
use std::sync::LazyLock;

/// Pattern for matching pull request numbers with the preceding hash.
const PULL_REQUEST_NUMBER_PATTERN: &str = r#"#\d+"#;

/// Regular expression for matching pull request numbers with the preceding hash.
pub static RE_PULL_REQUEST_NUMBER: LazyLock<Regex> = LazyLock::new(|| Regex::new(PULL_REQUEST_NUMBER_PATTERN).unwrap());

/// The commit.
#[derive(Clone)]
struct Commit {
  /// Full commit hash.
  hash: String,
  /// Commit title (subject).
  subject: String,
}

/// The issue.
#[derive(Clone)]
struct Issue {
  /// Issue number.
  number: String,
  /// Issue title.
  title: String,
  /// Issue URL on GitHub.
  url: String,
}

/// The pull request.
#[derive(Clone)]
struct PullRequest {
  /// Pull request number.
  number: String,
  /// Pull request title.
  title: String,
  /// Pull request URL on GitHub.
  url: String,
  /// List of commits that constitute this pull request.
  commits: Vec<Commit>,
}

pub fn get_changelog(verbose: bool, start_revision: &str, end_revision: &str, milestone: &str, repository: &str, dir: &str) -> Result<String> {
  if verbose {
    println!("\nCOMMANDS");
    println!("{SEPARATOR_LINE}");
  }
  // Retrieve issues with specified milestone from GitHub repository.
  let issues = get_issues(verbose, milestone, repository)?;
  // Retrieve pull requests with specified milestone from GitHub repository.
  let pull_requests = get_pull_requests(verbose, milestone, repository)?;
  // Retrieve commits in specified recision range.
  let commits = get_commits(verbose, dir, start_revision, end_revision)?;

  if verbose {
    println!("\nISSUES");
    println!("{SEPARATOR_LINE}");
    for issue in &issues {
      println!("{} | {} | {}", issue.number, issue.title, issue.url);
    }
    println!("\nPULL REQUESTS");
    println!("{SEPARATOR_LINE}");
    for pull_request in &pull_requests {
      println!("{} | {} | {}", pull_request.number, pull_request.title, pull_request.url);
      for commit in &pull_request.commits {
        println!("  {} | {}", commit.hash, commit.subject);
      }
    }
    println!("\nCOMMITS");
    println!("{SEPARATOR_LINE}");
    for commit in &commits {
      println!("{} | {}", commit.hash, commit.subject);
    }
  }

  // Move all commits to the map.
  let mut commit_map = HashMap::new();
  for commit in &commits {
    if !commit.subject.contains("[skip ci]") {
      commit_map.insert(commit.hash.clone(), commit.clone());
    }
  }

  // Move all issues to the sorted map.
  let mut issue_sorted_map = BTreeMap::new();
  for issue in &issues {
    issue_sorted_map.insert(issue.number.clone(), issue.clone());
  }

  // Move all pull requests to sorted map.
  let mut pull_request_map = BTreeMap::new();
  for pull_request in &pull_requests {
    // From commit map remove commits that are included in pull request.
    for commit in &pull_request.commits {
      commit_map.remove(&commit.hash);
    }
    pull_request_map.insert(pull_request.number.clone(), pull_request.clone());
  }

  // Check if there are commits, that contain a pull request number,
  // if such pull request exists in the map, then remove the commit,
  // otherwise display a warning with the pull request number.
  let mut warnings = BTreeMap::new();
  for commit in &commits {
    if let Some(captures) = RE_PULL_REQUEST_NUMBER.captures(&commit.subject) {
      let number = captures[0][1..].to_string();
      if pull_request_map.contains_key(&number) {
        commit_map.remove(&commit.hash);
      } else {
        warnings.insert(number.clone(), format!("PR: #{} not in milestone {} | {}", number, milestone, commit.subject));
      }
    }
  }

  // Prepare the string buffer for the changelog content.
  let mut changelog = String::new();
  // Write issue names.
  for issue in issue_sorted_map.values().rev() {
    let _ = writeln!(&mut changelog, "- {} ([#{}])", issue.title, issue.number);
  }
  // Write pull request names.
  for pull_request in pull_request_map.values().rev() {
    let _ = writeln!(&mut changelog, "- {} ([#{}])", pull_request.title, pull_request.number);
  }
  // Write commit names.
  for commit in commit_map.values() {
    let _ = writeln!(&mut changelog, "- {} ([0x{}])", commit.subject, &commit.hash[..7]);
  }
  let _ = writeln!(&mut changelog);
  // Write issue links.
  for issue in issue_sorted_map.values().rev() {
    let _ = writeln!(&mut changelog, "[#{}]: {}", issue.number, issue.url);
  }
  // Write pull request links.
  for pull_request in pull_request_map.values().rev() {
    let _ = writeln!(&mut changelog, "[#{}]: {}", pull_request.number, pull_request.url);
  }
  // Write commit links.
  for commit in commit_map.values() {
    let _ = writeln!(&mut changelog, "[0x{}]: https://github.com/{repository}/commit/{}", &commit.hash[..7], commit.hash);
  }

  if !warnings.is_empty() {
    let _ = writeln!(&mut changelog, "\nWARNINGS:");
    for warning in warnings.values().rev() {
      let _ = writeln!(&mut changelog, "{}", warning);
    }
  }
  Ok(changelog)
}

fn parse_issues(input: String) -> Result<Vec<Issue>> {
  let mut issues = vec![];
  let rows = parse_columns(input, 3)?;
  for columns in rows {
    issues.push(Issue {
      number: columns[0].to_string(),
      title: columns[1].to_string(),
      url: columns[2].to_string(),
    });
  }
  Ok(issues)
}

fn get_issues(verbose: bool, milestone: &str, repository: &str) -> Result<Vec<Issue>> {
  let search = format!(r#"--search=milestone:{}"#, milestone);
  let repo = format!("--repo={}", repository);
  let args = &[
    "issue",
    "list",
    search.as_str(),
    "--state=all",
    "--limit=9999",
    repo.as_str(),
    "--json=number,title,url",
    r#"--template='{{range .}}{{printf "%v ||| %s ||| %s\n" .number .title .url}}{{end}}'"#,
  ];
  let stdout = execute_command(verbose, "gh", args, ".")?;
  parse_issues(stdout)
}

fn get_pull_request_commits(verbose: bool, number: &str, repository: &str) -> Result<Vec<Commit>> {
  // gh pr view 661 --repo=cosmwasm/wasmvm --json=commits --jq='.commits[] | "\(.oid) ||| \(.messageHeadline)"'
  let repo = format!("--repo={}", repository);
  let args = &[
    "pr",
    "view",
    number,
    repo.as_str(),
    "--json=commits",
    r#"--jq=.commits[]|"\(.oid) ||| \(.messageHeadline)""#,
  ];
  let stdout = execute_command(verbose, "gh", args, ".")?;
  parse_commits(stdout)
}

fn parse_pull_requests(verbose: bool, input: String, repository: &str) -> Result<Vec<PullRequest>> {
  let mut pull_requests = vec![];
  let rows = parse_columns(input, 3)?;
  for columns in rows {
    let number = columns[0].to_string();
    let commits = get_pull_request_commits(verbose, &number, repository)?;
    pull_requests.push(PullRequest {
      number,
      title: columns[1].to_string(),
      url: columns[2].to_string(),
      commits,
    });
  }
  Ok(pull_requests)
}

fn get_pull_requests(verbose: bool, milestone: &str, repository: &str) -> Result<Vec<PullRequest>> {
  let search = format!(r#"--search=milestone:{}"#, milestone);
  let repo = format!("--repo={}", repository);
  let args = &[
    "pr",
    "list",
    search.as_str(),
    "--state=all",
    "--limit=9999",
    repo.as_str(),
    "--json=number,title,url",
    r#"--template='{{range .}}{{printf "%v ||| %s ||| %s\n" .number .title .url}}{{end}}'"#,
  ];
  let stdout = execute_command(verbose, "gh", args, ".")?;
  parse_pull_requests(verbose, stdout, repository)
}

fn parse_commits(input: String) -> Result<Vec<Commit>> {
  let mut commits = vec![];
  let rows = parse_columns(input, 2)?;
  for columns in rows {
    commits.push(Commit {
      hash: columns[0].to_string(),
      subject: columns[1].to_string(),
    });
  }
  Ok(commits)
}

fn get_commits(verbose: bool, dir: &str, start_revision: &str, end_revision: &str) -> Result<Vec<Commit>> {
  let revisions = format!("{}...{}", start_revision, end_revision);
  let args = &["log", r#"--format="%H ||| %s""#, revisions.as_str(), "--"];
  let stdout = execute_command(verbose, "git", args, dir)?;
  parse_commits(stdout)
}

fn execute_command(verbose: bool, program: &str, args: &[&str], dir: &str) -> Result<String> {
  if verbose {
    println!("{} {}", program, args.join(" "));
  } else {
    {
      use std::io::{self, Write};
      print!("•");
      io::stdout().flush().unwrap();
    }
  }
  let mut command = std::process::Command::new(program);
  let child = command
    .args(args)
    .current_dir(dir)
    .stdin(std::process::Stdio::piped())
    .stdout(std::process::Stdio::piped())
    .stderr(std::process::Stdio::piped())
    .spawn()
    .map_err(|e| error_spawn_command(program, e.to_string()))?;
  let output = child.wait_with_output().map_err(|e| error_obtain_output(e.to_string()))?;
  let stdout = String::from_utf8_lossy(&output.stdout).to_string();
  let stderr = String::from_utf8_lossy(&output.stderr).to_string();
  let status = output.status;
  if status.success() {
    Ok(stdout)
  } else {
    Err(error_execute_command(status, stdout, stderr))
  }
}

fn parse_columns(input: String, col_count: usize) -> Result<Vec<Vec<String>>> {
  let mut rows = vec![];
  for mut line in input.lines().map(|line| line.trim()) {
    if line.starts_with("\"") {
      line = line.strip_prefix("\"").unwrap();
    }
    if line.starts_with("'") {
      line = line.strip_prefix("'").unwrap();
    }
    if line.ends_with("\"") {
      line = line.strip_suffix("\"").unwrap();
    }
    if line.ends_with("'") {
      line = line.strip_suffix("'").unwrap();
    }
    line = line.trim();
    if !line.is_empty() {
      let columns = line.split(" ||| ").map(|s| s.to_string()).collect::<Vec<String>>();
      if columns.len() != col_count {
        return Err(MaggError::new(format!("invalid number of columns, expected: {col_count}, actual: {}", columns.len())));
      }
      rows.push(columns);
    }
  }
  Ok(rows)
}
